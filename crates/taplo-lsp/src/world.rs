use anyhow::{anyhow, Context};
use lsp_async_stub::{rpc, util::Mapper};
use lsp_types::Url;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration, path::Path};
use taplo::{parser::Parse};
use taplo_common::{
    config::Config,
    environment::Environment,
    schema::{associations::DEFAULT_CATALOGS, Schemas},
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
    pub fn by_document(&self, url: &Url) -> &WorkspaceState<E> {
        self.0
            .iter()
            .filter(|(key, _)| url.as_str().starts_with(key.as_str()))
            .max_by(|(a, _), (b, _)| a.as_str().len().cmp(&b.as_str().len()))
            .map(|(_, ws)| ws)
            .unwrap_or_else(|| {
                tracing::warn!(document_url = %url, "using detached workspace");
                self.0.get(&*DEFAULT_WORKSPACE_URL).unwrap()
            })
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
    pub(crate) env: E,
    pub(crate) workspaces: AsyncRwLock<Workspaces<E>>,
}

static DEFAULT_WORKSPACE_URL: Lazy<Url> = Lazy::new(|| Url::parse("root:///").unwrap());

impl<E: Environment> WorldState<E> {
    pub fn new(env: E) -> Self {
        Self {
            workspaces: {
                let mut m = IndexMap::default();
                m.insert(
                    DEFAULT_WORKSPACE_URL.clone(),
                    WorkspaceState::new(env.clone(), DEFAULT_WORKSPACE_URL.clone()),
                );
                AsyncRwLock::new(Workspaces(m))
            },
            env,
        }
    }
}

pub struct WorkspaceState<E: Environment> {
    pub(crate) root: Url,
    pub(crate) documents: HashMap<lsp_types::Url, DocumentState>,
    pub(crate) taplo_config: Config,
    pub(crate) schemas: Schemas<E>,
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
    pub(crate) async fn initialize(&mut self, env: &impl Environment) -> Result<(), anyhow::Error> {
        self.load_config(env).await?;
        for catalog in DEFAULT_CATALOGS {
            self.schemas
                .associations()
                .add_from_config(&self.taplo_config);

            self.schemas
                .associations()
                .add_from_catalog(&Url::parse(catalog).unwrap())
                .await
                .with_context(|| "failed to load schema catalog")?;
        }
        Ok(())
    }

    pub(crate) async fn load_config(
        &mut self,
        env: &impl Environment,
    ) -> Result<(), anyhow::Error> {
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

            tracing::debug!("{:#?}", self.taplo_config);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DocumentState {
    pub(crate) parse: Parse,
    pub(crate) mapper: Mapper,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaConfiguration {
    pub enabled: Option<bool>,
    pub associations: Option<HashMap<String, String>>,
    pub catalogs: Option<Vec<String>>,
    pub links: Option<bool>,
}

// This is not exhaustive
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LspConfig {
    pub(crate) taplo_config: Option<String>,
    pub(crate) taplo_config_enabled: Option<bool>,
    pub(crate) schema: SchemaConfiguration,
    pub(crate) semantic_tokens: Option<bool>,
    pub(crate) cache_path: Option<String>,
    pub(crate) formatter: taplo::formatter::OptionsIncompleteCamel,
}
