use crate::{
    config::{InitConfig, LspConfig},
    lsp_ext::notification::{DidChangeSchemaAssociation, DidChangeSchemaAssociationParams},
};
use anyhow::anyhow;
use arc_swap::ArcSwap;
use lsp_async_stub::{rpc, util::Mapper, Context, RequestWriter};
use lsp_types::Url;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::json;
use std::{path::Path, sync::Arc, time::Duration};
use taplo::{dom::Node, parser::Parse};
use taplo_common::{
    config::Config,
    environment::Environment,
    plugin::Plugin,
    schema::{
        associations::{priority, source, AssociationRule, SchemaAssociation},
        Schemas,
    },
    AsyncRwLock, HashMap, IndexMap,
};

pub type World<E> = Arc<WorldState<E>>;

#[repr(transparent)]
pub struct Workspaces<E: Environment>(IndexMap<Url, WorkspaceState<E>>);

impl<E: Environment> std::ops::Deref for Workspaces<E> {
    type Target = IndexMap<Url, WorkspaceState<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E: Environment> std::ops::DerefMut for Workspaces<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E: Environment> Workspaces<E> {
    #[must_use]
    pub fn by_document(&self, url: &Url) -> &WorkspaceState<E> {
        self.0
            .iter()
            .filter(|(key, _)| url.as_str().starts_with(key.as_str()))
            .max_by(|(a, _), (b, _)| a.as_str().len().cmp(&b.as_str().len()))
            .map_or_else(
                || {
                    tracing::warn!(document_url = %url, "using detached workspace");
                    self.0.get(&*DEFAULT_WORKSPACE_URL).unwrap()
                },
                |(_, ws)| ws,
            )
    }

    pub fn by_document_mut(&mut self, url: &Url) -> &mut WorkspaceState<E> {
        self.0
            .iter_mut()
            .filter(|(key, _)| {
                url.as_str().starts_with(key.as_str()) || *key == &*DEFAULT_WORKSPACE_URL
            })
            .max_by(|(a, _), (b, _)| a.as_str().len().cmp(&b.as_str().len()))
            .map(|(k, ws)| {
                if k == &*DEFAULT_WORKSPACE_URL {
                    tracing::warn!(document_url = %url, "using detached workspace");
                }

                ws
            })
            .unwrap()
    }
}

pub struct WorldState<E: Environment> {
    pub(crate) init_config: ArcSwap<InitConfig>,
    pub(crate) env: E,
    pub(crate) workspaces: AsyncRwLock<Workspaces<E>>,
    pub(crate) default_config: ArcSwap<Config>,
}

pub static DEFAULT_WORKSPACE_URL: Lazy<Url> = Lazy::new(|| Url::parse("root:///").unwrap());

impl<E: Environment> WorldState<E> {
    pub fn new(env: E) -> Self {
        Self {
            init_config: Default::default(),
            workspaces: {
                let mut m = IndexMap::default();
                m.insert(
                    DEFAULT_WORKSPACE_URL.clone(),
                    WorkspaceState::new(env.clone(), DEFAULT_WORKSPACE_URL.clone()),
                );
                AsyncRwLock::new(Workspaces(m))
            },
            default_config: Default::default(),
            env,
        }
    }

    /// Set the world state's default config.
    pub fn set_default_config(&self, default_config: Arc<Config>) {
        self.default_config.store(default_config);
    }
}

pub struct WorkspaceState<E: Environment> {
    pub(crate) root: Url,
    pub(crate) documents: HashMap<lsp_types::Url, DocumentState>,
    pub(crate) taplo_config: Config,
    pub(crate) schemas: Schemas<E>,
    pub(crate) config: LspConfig,
    pub(crate) plugins: ArcSwap<Vec<Arc<dyn Plugin<E>>>>,
}

impl<E: Environment> WorkspaceState<E> {
    pub(crate) fn new(env: E, root: Url) -> Self {
        Self {
            root,
            documents: Default::default(),
            taplo_config: Default::default(),
            schemas: Schemas::new(
                env,
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap(),
            ),
            config: LspConfig::default(),
            plugins: Default::default(),
        }
    }
}

impl<E: Environment> WorkspaceState<E> {
    pub(crate) fn document(&self, url: &Url) -> Result<&DocumentState, rpc::Error> {
        self.documents
            .get(url)
            .ok_or_else(rpc::Error::invalid_params)
    }

    #[tracing::instrument(skip_all, fields(%self.root))]
    pub(crate) async fn initialize(
        &mut self,
        context: Context<World<E>>,
        env: &impl Environment,
    ) -> Result<(), anyhow::Error> {
        self.load_config(env, &*context.world().default_config.load())
            .await?;

        let plugins = Arc::new(
            self.taplo_config
                .plugins
                .iter()
                .filter_map(|(name, p_cfg)| {
                    load_plugin::<E>(name).map(|p| {
                        if let Some(settings) = p_cfg.settings.clone() {
                            p.settings(settings);
                        }
                        p
                    })
                })
                .collect::<Vec<_>>(),
        );

        self.schemas.set_plugins(&plugins);
        self.plugins.store(plugins);

        self.schemas
            .associations()
            .retain(|(_, assoc)| assoc.meta["source"] == "manual");

        if !self.config.schema.enabled {
            return Ok(());
        }

        self.schemas.cache().set_expiration_times(
            Duration::from_secs(self.config.schema.cache.memory_expiration),
            Duration::from_secs(self.config.schema.cache.disk_expiration),
        );

        self.schemas
            .associations()
            .add_from_config(&self.taplo_config);

        for (pattern, schema_url) in &self.config.schema.associations {
            let pattern = match Regex::new(pattern) {
                Ok(p) => p,
                Err(error) => {
                    tracing::error!(%error, "invalid association pattern");
                    continue;
                }
            };

            let url = if schema_url.starts_with("./") {
                self.root.join(schema_url)
            } else {
                schema_url.parse()
            };

            let url = match url {
                Ok(u) => u,
                Err(error) => {
                    tracing::error!(%error, url = %schema_url, "invalid schema url");
                    continue;
                }
            };

            self.schemas.associations().add(
                AssociationRule::Regex(pattern),
                SchemaAssociation {
                    url,
                    meta: json!({
                        "source": source::LSP_CONFIG,
                    }),
                    priority: priority::LSP_CONFIG,
                },
            );
        }

        for catalog in &self.config.schema.catalogs {
            if let Err(error) = self.schemas.associations().add_from_catalog(catalog).await {
                tracing::error!(%error, "failed to add schemas from catalog");
            }
        }

        self.emit_associations(context).await;
        Ok(())
    }

    pub(crate) async fn load_config(
        &mut self,
        env: &impl Environment,
        default_config: &Config,
    ) -> Result<(), anyhow::Error> {
        if !self.config.taplo.config_file.enabled {
            self.taplo_config = default_config.clone();
            return Ok(());
        }

        let root_path = env
            .to_file_path(&self.root)
            .ok_or_else(|| anyhow!("invalid root URL"))?;
        if let Some(config_path) = env.find_config_file(&root_path).await {
            tracing::info!(path = ?config_path, "found config file");
            self.taplo_config = toml::from_slice(&env.read_file(&config_path).await?)?;

            // This is different from the path we found the config in, in this case
            // we wish to keep the scheme of the path.
            let base_path = Path::new(self.root.as_str());
            self.taplo_config.prepare(env, base_path)?;

            tracing::debug!("using config: {:#?}", self.taplo_config);
        }
        Ok(())
    }

    pub(crate) async fn emit_associations(&self, mut context: Context<World<E>>) {
        for document_url in self.documents.keys() {
            if let Some(assoc) = self
                .schemas
                .associations()
                .association_for(document_url.as_str())
            {
                if let Err(error) = context
                    .write_notification::<DidChangeSchemaAssociation, _>(Some(
                        DidChangeSchemaAssociationParams {
                            document_uri: document_url.clone(),
                            schema_uri: Some(assoc.url.clone()),
                            meta: Some(assoc.meta.clone()),
                        },
                    ))
                    .await
                {
                    tracing::error!(%error, "failed to write notification");
                }
            } else if let Err(error) = context
                .write_notification::<DidChangeSchemaAssociation, _>(Some(
                    DidChangeSchemaAssociationParams {
                        document_uri: document_url.clone(),
                        schema_uri: None,
                        meta: None,
                    },
                ))
                .await
            {
                tracing::error!(%error, "failed to write notification");
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocumentState {
    pub(crate) parse: Parse,
    pub(crate) dom: Node,
    pub(crate) mapper: Mapper,
}

fn load_plugin<E: Environment>(name: &str) -> Option<Arc<dyn Plugin<E>>> {
    match name {
        #[cfg(feature = "plugin-crates")]
        "crates" => Some(Arc::new(taplo_plugin_crates::CratesPlugin::default())),
        _ => {
            tracing::warn!(%name, "unknown plugin");
            None
        }
    }
}
