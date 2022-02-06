use lsp_async_stub::{rpc::Error, util::LspExt, Context, Params};
use lsp_types::{DocumentFormattingParams, TextEdit};
use std::path::Path;
use taplo::formatter;
use taplo_common::environment::Environment;

use crate::World;

pub(crate) async fn format<E: Environment>(
    context: Context<World<E>>,
    params: Params<DocumentFormattingParams>,
) -> Result<Option<Vec<TextEdit>>, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.text_document.uri);
    let doc = ws.document(&p.text_document.uri)?;

    let doc_path = Path::new(p.text_document.uri.as_str());

    let mut format_opts = formatter::Options {
        indent_string: if p.options.insert_spaces {
            " ".repeat(p.options.tab_size as usize)
        } else {
            "\t".into()
        },
        ..Default::default()
    };
    if let Some(v) = p.options.insert_final_newline {
        format_opts.trailing_newline = v;
    }

    ws.taplo_config
        .update_format_options(doc_path, &mut format_opts);

    Ok(Some(vec![TextEdit {
        range: doc.mapper.all_range().into_lsp(),
        new_text: taplo::formatter::format_with_path_scopes(
            doc.parse.clone().into_dom(),
            format_opts,
            ws.taplo_config.format_scopes(doc_path),
        ),
    }]))
}
