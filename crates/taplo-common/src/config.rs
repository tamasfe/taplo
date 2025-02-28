use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use anyhow::Context;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use taplo::formatter;
use url::Url;

use crate::{
    environment::Environment,
    util::{GlobRule, Normalize},
    HashMap,
};

pub const CONFIG_FILE_NAMES: &[&str] = &[".taplo.toml", "taplo.toml"];

#[derive(Default, Clone, Serialize, Deserialize, JsonSchema)]
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<Rule>,

    #[serde(flatten)]
    pub global_options: Options,

    #[serde(skip)]
    pub file_rule: Option<GlobRule>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<HashMap<String, Plugin>>,
}

impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("include", &self.include)
            .field("exclude", &self.exclude)
            .field("rule", &self.rule)
            .field("global_options", &self.global_options)
            .finish()
    }
}

impl Config {
    /// Prepare the configuration for further use.
    pub fn prepare(&mut self, e: &impl Environment, base: &Path) -> Result<(), anyhow::Error> {
        self.make_absolute(e, base);

        let default_include = String::from("**/*.toml");

        self.file_rule = Some(GlobRule::new(
            self.include
                .as_deref()
                .unwrap_or(&[default_include] as &[String]),
            self.exclude.as_deref().unwrap_or(&[] as &[String]),
        )?);

        for rule in &mut self.rule {
            rule.prepare(e, base).context("invalid rule")?;
        }

        self.global_options.prepare(e, base)?;

        Ok(())
    }

    #[must_use]
    pub fn is_included(&self, path: &Path) -> bool {
        match &self.file_rule {
            Some(r) => r.is_match(path),
            None => {
                tracing::debug!("no file matches were set up");
                false
            }
        }
    }

    #[must_use]
    pub fn rules_for<'r>(
        &'r self,
        path: &'r Path,
    ) -> impl DoubleEndedIterator<Item = &'r Rule> + Clone + 'r {
        self.rule.iter().filter(|r| r.is_included(path))
    }

    pub fn update_format_options(&self, path: &Path, options: &mut formatter::Options) {
        if let Some(opts) = &self.global_options.formatting {
            options.update(opts.clone());
        }

        for rule in self.rules_for(path) {
            if rule.keys.is_none() {
                if let Some(rule_opts) = rule.options.formatting.clone() {
                    options.update(rule_opts);
                }
            }
        }
    }

    pub fn format_scopes<'s>(
        &'s self,
        path: &'s Path,
    ) -> impl Iterator<Item = (&'s String, taplo::formatter::OptionsIncomplete)> + Clone + 's {
        self.rules_for(path)
            .filter_map(|rule| match (&rule.keys, &rule.options.formatting) {
                (Some(keys), Some(opts)) => Some(keys.iter().map(move |k| (k, opts.clone()))),
                _ => None,
            })
            .flatten()
    }

    #[must_use]
    pub fn is_schema_enabled(&self, path: &Path) -> bool {
        let enabled = self
            .global_options
            .schema
            .as_ref()
            .and_then(|s| s.enabled)
            .unwrap_or(true);

        for rule in &self.rule {
            let rule_matched = match &self.file_rule {
                Some(r) => r.is_match(path),
                None => {
                    tracing::debug!("no file matches were set up");
                    false
                }
            };

            if !rule_matched {
                continue;
            }

            let rule_schema_enabled = rule
                .options
                .schema
                .as_ref()
                .and_then(|s| s.enabled)
                .unwrap_or(true);

            if !rule_schema_enabled {
                return false;
            }
        }

        enabled
    }

    /// Transform all relative glob patterns to have the given base path.
    fn make_absolute(&mut self, e: &impl Environment, base: &Path) {
        if let Some(included) = &mut self.include {
            for pat in included {
                if !e.is_absolute(Path::new(pat)) {
                    *pat = base
                        .join(pat.as_str())
                        .normalize()
                        .to_string_lossy()
                        .into_owned();
                }
            }
        }

        if let Some(excluded) = &mut self.exclude {
            for pat in excluded {
                if !e.is_absolute(Path::new(pat)) {
                    *pat = base
                        .join(pat.as_str())
                        .normalize()
                        .to_string_lossy()
                        .into_owned();
                }
            }
        }

        for rule in &mut self.rule {
            if let Some(included) = &mut rule.include {
                for pat in included {
                    if !e.is_absolute(Path::new(pat)) {
                        *pat = base
                            .join(pat.as_str())
                            .normalize()
                            .to_string_lossy()
                            .into_owned();
                    }
                }
            }

            if let Some(excluded) = &mut rule.exclude {
                for pat in excluded {
                    if !e.is_absolute(Path::new(pat)) {
                        *pat = base
                            .join(pat.as_str())
                            .normalize()
                            .to_string_lossy()
                            .into_owned();
                    }
                }
            }
        }
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

impl Options {
    fn prepare(&mut self, e: &impl Environment, base: &Path) -> Result<(), anyhow::Error> {
        if let Some(schema_opts) = &mut self.schema {
            let url = match schema_opts.path.take() {
                Some(p) => {
                    if let Ok(url) = p.parse() {
                        Some(url)
                    } else {
                        let p = if e.is_absolute(Path::new(&p)) {
                            PathBuf::from(p)
                        } else {
                            base.join(p).normalize()
                        };

                        let s = p.to_string_lossy();

                        Some(Url::parse(&format!("file://{s}")).context("invalid schema path")?)
                    }
                }
                None => schema_opts.url.take(),
            };

            schema_opts.url = url;
        }

        Ok(())
    }
}

/// A rule to override options by either name or file.
#[derive(Default, Clone, Serialize, Deserialize, JsonSchema)]
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

    #[serde(skip)]
    pub file_rule: Option<GlobRule>,
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("name", &self.name)
            .field("include", &self.include)
            .field("exclude", &self.exclude)
            .field("keys", &self.keys)
            .field("options", &self.options)
            .finish()
    }
}

impl Rule {
    pub fn prepare(&mut self, e: &impl Environment, base: &Path) -> Result<(), anyhow::Error> {
        let default_include = String::from("**");
        self.file_rule = Some(GlobRule::new(
            self.include
                .as_deref()
                .unwrap_or(&[default_include] as &[String]),
            self.exclude.as_deref().unwrap_or(&[] as &[String]),
        )?);
        self.options.prepare(e, base)?;
        Ok(())
    }

    #[must_use]
    pub fn is_included(&self, path: &Path) -> bool {
        match &self.file_rule {
            Some(r) => r.is_match(path),
            None => true,
        }
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

    /// A local file path to the schema, overrides `url` if set.
    ///
    /// URLs are also accepted here, but it's not a guarantee and might
    /// change in newer releases.
    /// Please use the `url` field instead whenever possible.
    pub path: Option<String>,

    /// A full absolute URL to the schema.
    ///
    /// The url of the schema, supported schemes are `http`, `https`, `file` and `taplo`.
    pub url: Option<Url>,
}

/// A plugin to extend Taplo's capabilities.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Plugin {
    /// Optional settings for the plugin.
    #[serde(default)]
    pub settings: Option<Value>,
}
