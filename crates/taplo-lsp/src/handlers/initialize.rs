use std::sync::Arc;

use super::{semantic_tokens, update_configuration};
use crate::config::InitConfig;
use crate::world::WorkspaceState;
use crate::World;
use lsp_async_stub::{rpc::Error, Context, Params};
use lsp_types::{
    CompletionOptions, DocumentLinkOptions, FoldingRangeProviderCapability,
    HoverProviderCapability, InitializedParams, OneOf, RenameOptions, SemanticTokensFullOptions,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensServerCapabilities,
    ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
    WorkDoneProgressOptions, WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
};
use lsp_types::{InitializeParams, InitializeResult};
use taplo_common::environment::Environment;

#[tracing::instrument(skip_all)]
pub async fn initialize<E: Environment>(
    context: Context<World<E>>,
    params: Params<InitializeParams>,
) -> Result<InitializeResult, Error> {
    let p = params.required()?;

    if let Some(init_opts) = p.initialization_options {
        match serde_json::from_value::<InitConfig>(init_opts) {
            Ok(c) => context.init_config.store(Arc::new(c)),
            Err(error) => {
                tracing::error!(%error, "invalid initialization options");
            }
        }
    }

    if let Some(workspaces) = p.workspace_folders {
        let mut wss = context.workspaces.write().await;
        let init_config = context.init_config.load();

        for workspace in workspaces {
            let ws = wss
                .entry(workspace.uri.clone())
                .or_insert(WorkspaceState::new(context.env.clone(), workspace.uri));

            ws.schemas
                .cache()
                .set_cache_path(init_config.cache_path.clone());

            if let Err(error) = ws.initialize(context.clone(), &context.env).await {
                tracing::error!(?error, "failed to initialize workspace");
            }
        }
    }

    Ok(InitializeResult {
        capabilities: ServerCapabilities {
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(true),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                ..Default::default()
            }),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: false.into(),
                    },
                    legend: SemanticTokensLegend {
                        token_types: semantic_tokens::TokenType::LEGEND.into(),
                        token_modifiers: semantic_tokens::TokenModifier::MODIFIERS.into(),
                    },
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                    range: Some(false),
                }),
            ),
            rename_provider: Some(OneOf::Right(RenameOptions {
                prepare_provider: Some(true),
                work_done_progress_options: Default::default(),
            })),
            folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            document_formatting_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(false),
                trigger_characters: Some(vec![
                    ".".into(),
                    "=".into(),
                    "[".into(),
                    "{".into(),
                    ",".into(),
                    "\"".into(),
                ]),
                ..Default::default()
            }),
            document_link_provider: Some(DocumentLinkOptions {
                resolve_provider: None,
                work_done_progress_options: Default::default(),
            }),
            ..Default::default()
        },
        server_info: Some(ServerInfo {
            name: "Taplo".into(),
            version: Some(env!("CARGO_PKG_VERSION").into()),
        }),
        offset_encoding: None,
    })
}

#[tracing::instrument(skip_all)]
pub async fn initialized<E: Environment>(
    context: Context<World<E>>,
    _params: Params<InitializedParams>,
) {
    context
        .env
        .spawn_local(update_configuration(context.clone()));
}
