use arc_swap::ArcSwap;
use async_trait::async_trait;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use rayon::iter::ParallelIterator;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use smartstring::alias::CompactString;
use std::{borrow::Cow, sync::Arc};
use taplo::dom::{node::Key, Keys};
use taplo_common::{
    environment::Environment,
    plugin::{CollectSchemasAction, Plugin},
    schema::Schemas,
};
use url::Url;

/// All non-yanked crate names and versions.
static ALL_CRATES: OnceCell<Vec<(CompactString, CompactString)>> = OnceCell::new();

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct CratesSettings {
    all_crates: bool,
}

#[derive(Default)]
pub struct CratesPlugin {
    index: OnceCell<crates_index::Index>,
    settings: ArcSwap<CratesSettings>,
}

#[async_trait(?Send)]
impl<E: Environment> Plugin<E> for CratesPlugin {
    fn name(&self) -> Cow<'static, str> {
        "crates".into()
    }

    fn settings(&self, value: Value) {
        match serde_json::from_value(value) {
            Ok(s) => {
                self.settings.store(Arc::new(s));
            }
            Err(err) => {
                tracing::error!(error = %err, "invalid plugin settings");
            }
        };
    }

    async fn possible_schemas(
        &self,
        schemas: &Schemas<E>,
        _root_url: &Url,
        schema: &Value,
        root_path: &Keys,
        relative_path: &Keys,
        child_schemas: &mut Vec<(Keys, Keys, Arc<Value>)>,
    ) -> CollectSchemasAction {
        let index = match self
            .index
            .get_or_try_init(crates_index::Index::new_cargo_default)
        {
            Ok(index) => index,
            Err(err) => {
                tracing::warn!(error = %err, "failed to retrieve crates index");
                return CollectSchemasAction::Continue;
            }
        };

        if self.settings.load().all_crates
            && schema["x-taplo"]["crates"]["schemas"] == "dependencies"
        {
            if ALL_CRATES.get().is_none() {
                ALL_CRATES.set(
                        schemas
                            .env()
                            .spawn_blocking(|| {
                                rayon::ThreadPoolBuilder::new()
                                .build()
                                .unwrap()
                                .install(move || {
                                    let index = match crates_index::Index::new_cargo_default() {
                                        Ok(idx) => idx,
                                        Err(err) => {
                                            tracing::warn!(error = %err, "failed to retrieve crates index");
                                            return Vec::new();
                                        }
                                    };
                                    index
                                        .crates_parallel()
                                        .filter_map(Result::ok)
                                        .filter_map(|c| {
                                            c.versions().iter().filter(|v| !v.is_yanked()).last().map(|v| {
                                                (
                                                    CompactString::from(c.name()),
                                                    CompactString::from(v.version()),
                                                )
                                            })
                                        })
                                        .collect()
                                })
                            })
                            .await
                    ).ok();
            }

            child_schemas.extend(ALL_CRATES.get().unwrap().iter().map(|(name, version)| {
                (
                    root_path.clone(),
                    relative_path.join(Key::from(&**name)),
                    Arc::new(json!({
                        "required": ["version"],
                        "properties": {
                            "version": {
                                "type": "string",
                                "default": version
                            }
                        }
                    })),
                )
            }));
        } else if schema["x-taplo"]["crates"]["schemas"] == "version" {
            // This will not work with the "version" crate, hopefully it's not a huge issue.
            let crate_name = match root_path
                .iter()
                .rev()
                .find(|k| !k.is_index() && *k != "version")
                .and_then(|k| k.as_key())
            {
                Some(crate_name) => crate_name,
                None => return CollectSchemasAction::Continue,
            };

            let c = match index.crate_(crate_name.value()) {
                Some(c) => c,
                None => return CollectSchemasAction::Continue,
            };

            let highest_version = c
                .highest_stable_version()
                .unwrap_or_else(|| c.highest_version());

            child_schemas.push((
                root_path.clone(),
                relative_path.clone(),
                Arc::new(json!({
                    "type": "string",
                    "enum": c.versions().iter().rev().filter(|v| !v.is_yanked()).map(|v| v.version()).collect::<Vec<_>>(),
                    "default": highest_version.version()
                })),
            ));
        } else if schema["x-taplo"]["crates"]["schemas"] == "feature" {
            let crate_name = match root_path
                .iter()
                .rev()
                .find(|k| !k.is_index() && *k != "features")
                .and_then(|k| k.as_key())
            {
                Some(crate_name) => crate_name,
                None => return CollectSchemasAction::Continue,
            };

            let c = match index.crate_(crate_name.value()) {
                Some(c) => c,
                None => return CollectSchemasAction::Continue,
            };

            // FIXME: currently there is no way to know the version of the crate,
            // so we list features from all versions.
            let c_feature_schemas = c
                .versions()
                .iter()
                .filter(|v| !v.is_yanked())
                .flat_map(|v| v.features().iter())
                .filter(|(f, _)| *f != "default")
                .unique_by(|f| f.0)
                .map(|(f, enables)| {
                    let desc = if enables.is_empty() {
                        String::from("This feature does not enable additional features.")
                    } else {
                        let mut s = String::from("Enables additional features:\n\n");

                        for enables in enables {
                            s += "- `";
                            s += enables;
                            s += "`\n"
                        }

                        s
                    };

                    Arc::new(json!({
                        "type": "string",
                        "enum": [f],
                        "description": desc
                    }))
                });

            child_schemas.extend(
                c_feature_schemas.map(|schema| (root_path.clone(), relative_path.clone(), schema)),
            );
        }

        CollectSchemasAction::Continue
    }
}
