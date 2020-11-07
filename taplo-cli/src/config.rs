use anyhow::anyhow;
use schemars::{schema::RootSchema, JsonSchema};
use serde::{Deserialize, Serialize};
use taplo::formatter;

use crate::external::{self, glob_match_options};

pub const CONFIG_FILE_NAMES: &[&str] = &[".taplo.toml", "taplo.toml"];

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Files to include.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    /// Globstars (`**`) are supported.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// Omitting this property includes all files, **however an empty array will include none**.
    pub include: Option<Vec<String>>,

    /// Files to exclude (ignore).
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    /// Globstars (`**`) are supported.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// This has priority over `include`.
    pub exclude: Option<Vec<String>>,

    /// Rules are used to override configurations by path and keys.
    pub rule: Option<Vec<Rule>>,

    #[serde(flatten)]
    pub global_options: Options,
}

impl Config {
    /// Tell if a path is excluded
    pub fn is_excluded(&self, path: &str) -> Result<bool, glob::PatternError> {
        match &self.exclude {
            Some(excluded) => {
                for p in excluded {
                    let pat = glob::Pattern::new(p)?;

                    if pat.matches_with(path, glob_match_options()) {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            None => Ok(false),
        }
    }

    pub async fn get_schema(&self, path: &str) -> Result<Option<RootSchema>, anyhow::Error> {
        if let Some(rules) = &self.rule {
            for rule in rules.iter().rev() {
                if let Some(schema_opts) = &rule.options.schema {
                    if schema_opts.enabled.unwrap_or(false) {
                        match &schema_opts.path {
                            Some(schema_path) => {
                                if rule.includes(path)? {
                                    return Ok(Some(external::get_schema(schema_path).await?));
                                }
                            }
                            None => return Err(anyhow!("schema is enabled, but path is missing")),
                        }
                    }
                }
            }
        }

        if let Some(schema_opts) = &self.global_options.schema {
            if schema_opts.enabled.unwrap_or(false) {
                match &schema_opts.path {
                    Some(schema_path) => return Ok(Some(external::get_schema(schema_path).await?)),
                    None => return Err(anyhow!("schema is enabled, but path is missing")),
                }
            }
        }

        Ok(None)
    }

    pub fn get_schema_path(&self, path: &str) -> Result<Option<String>, anyhow::Error> {
        if let Some(rules) = &self.rule {
            for rule in rules.iter().rev() {
                if let Some(schema_opts) = &rule.options.schema {
                    if schema_opts.enabled.unwrap_or(false) {
                        match &schema_opts.path {
                            Some(schema_path) => {
                                if rule.includes(path)? {
                                    return Ok(Some(schema_path.clone()));
                                }
                            }
                            None => return Err(anyhow!("schema is enabled, but path is missing")),
                        }
                    }
                }
            }
        }

        if let Some(schema_opts) = &self.global_options.schema {
            if schema_opts.enabled.unwrap_or(false) {
                match &schema_opts.path {
                    Some(schema_path) => return Ok(Some(schema_path.clone())),
                    None => return Err(anyhow!("schema is enabled, but path is missing")),
                }
            }
        }

        Ok(None)
    }

    pub fn collect_schemas(&self) -> Vec<String> {
        let mut schemas = Vec::new();

        if let Some(s) = self
            .global_options
            .schema
            .as_ref()
            .and_then(|s| s.path.as_ref())
        {
            schemas.push(s.clone());
        }

        if let Some(rules) = &self.rule {
            for r in rules {
                if let Some(s) = r.options.schema.as_ref().and_then(|s| s.path.as_ref()) {
                    schemas.push(s.clone());
                }
            }
        }

        schemas
    }

    pub fn get_include_paths(&self) -> Vec<String> {
        match &self.include {
            Some(i) => i.clone(),
            None => vec!["**/*.toml".to_string()],
        }
    }

    pub fn check_patterns(&self) -> Result<(), glob::PatternError> {
        if let Some(included) = &self.include {
            for p in included {
                if let Err(e) = glob::Pattern::new(p) {
                    return Err(e);
                }
            }
        }

        if let Some(excluded) = &self.exclude {
            for p in excluded {
                if let Err(e) = glob::Pattern::new(p) {
                    return Err(e);
                }
            }
        }

        if let Some(r) = &self.rule {
            for r in r {
                if let Some(included) = &r.include {
                    for p in included {
                        if let Err(e) = glob::Pattern::new(p) {
                            return Err(e);
                        }
                    }
                }

                if let Some(excluded) = &r.exclude {
                    for p in excluded {
                        if let Err(e) = glob::Pattern::new(p) {
                            return Err(e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn get_formatter_options(
        &self,
        path: Option<&str>,
        default_opts: Option<formatter::Options>,
    ) -> Result<
        (
            formatter::Options,
            Vec<(String, formatter::OptionsIncomplete)>,
        ),
        glob::PatternError,
    > {
        let mut opts = default_opts.unwrap_or_default();

        if let Some(opt) = &self.global_options.formatting {
            opts.update(opt.clone());
        }

        let path = match path {
            Some(p) => p,
            None => {
                return Ok((
                    opts,
                    self.rule
                        .as_ref()
                        .map(|r| {
                            r.iter()
                                .filter(|r| r.include.is_none())
                                .filter_map::<Vec<_>, _>(|r| match &r.options.formatting {
                                    Some(format_opts) => match &r.keys {
                                        Some(k) => Some(
                                            k.iter()
                                                .map(|s| (s.clone(), format_opts.clone()))
                                                .collect(),
                                        ),
                                        None => None,
                                    },
                                    None => None,
                                })
                                .flatten()
                                .collect()
                        })
                        .unwrap_or_default(),
                ))
            }
        };

        return Ok((
            opts,
            match &self.rule {
                Some(r) => r
                    .iter()
                    .filter_map(|r| match r.includes(path) {
                        Ok(includes) => {
                            if includes {
                                Some(Ok(r))
                            } else {
                                None
                            }
                        }
                        Err(err) => Some(Err(err)),
                    })
                    .filter_map(|result| match result {
                        Ok(r) => match &r.options.formatting {
                            Some(format_opts) => match &r.keys {
                                Some(k) => Some(Ok(k
                                    .iter()
                                    .map(|s| (s.clone(), format_opts.clone()))
                                    .collect())),
                                None => None,
                            },
                            None => None,
                        },
                        Err(err) => Some(Err(err)),
                    })
                    .collect::<Result<Vec<Vec<(String, formatter::OptionsIncomplete)>>, _>>()?
                    .into_iter()
                    .flatten()
                    .collect(),
                None => Vec::new(),
            },
        ));
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Options {
    /// Schema validation options.
    pub schema: Option<SchemaOptions>,
    /// Formatting options.
    pub formatting: Option<formatter::OptionsIncomplete>,
}

/// A rule to override options by either name or file.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    /// The name of the rule.
    ///
    /// Used in `taplo::<name>` comments.
    pub name: Option<String>,

    /// Files this rule is valid for.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// Omitting this property includes all files, **however an empty array will include none**.
    pub include: Option<Vec<String>>,

    /// Files that are excluded from this rule.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    ///
    /// Relative paths are **not** relative to the configuration file, but rather
    /// depends on the tool using the configuration.
    ///
    /// This has priority over `include`.
    pub exclude: Option<Vec<String>>,

    /// Keys the rule is valid for in a document.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) dotted key patterns.
    ///
    /// This allows enabling the rule for specific paths in the document.
    ///
    /// For example:
    ///
    /// - `package.metadata` will enable the rule for everything inside the `package.metadata` table, including itself.
    ///
    /// If omitted, the rule will always be valid for all keys.
    pub keys: Option<Vec<String>>,

    #[serde(flatten)]
    pub options: Options,
}

impl Rule {
    fn includes(&self, p: &str) -> Result<bool, glob::PatternError> {
        let mut included = false;

        match &self.include {
            Some(include) => {
                for inc in include {
                    let pat = glob::Pattern::new(inc)?;

                    if pat.matches_with(p, glob_match_options()) {
                        included = true;
                        break;
                    }
                }
            }
            None => included = true,
        }

        if let Some(ex) = &self.exclude {
            for ex in ex {
                let pat = glob::Pattern::new(ex)?;
                if pat.matches_with(p, glob_match_options()) {
                    return Ok(false);
                }
            }
        }

        Ok(included)
    }
}

/// Options for schema validation and completion.
///
/// Schemas in rules with defined keys are ignored.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SchemaOptions {
    /// Whether the schema should be enabled or not.
    ///
    /// Defaults to true if omitted.
    pub enabled: Option<bool>,

    /// Path to the schema.
    ///
    /// The path of the schema, this can be either path to a local file path or an URL with the schemes `taplo`,
    /// `http` or `https`. (`file` scheme is also accepted, it is the same as specifying a local path)
    ///
    pub path: Option<String>,
}
