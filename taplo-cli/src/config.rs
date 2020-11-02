use std::iter::FromIterator;

use crate::glob_match_options;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use taplo::{dom::Path, formatter};

pub const CONFIG_FILE_NAMES: &[&str] = &[".taplo.toml", "taplo.toml"];

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Config {
    /// Files to include.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    /// Globstars (`**`) are supported.
    pub include: Option<Vec<String>>,

    /// Files to exclude (ignore).
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    /// Globstars (`**`) are supported.
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

    pub fn get_include_paths(&self) -> Vec<String> {
        match &self.include {
            Some(i) => i.clone(),
            None => vec!["**/*.toml".to_string()],
        }
    }

    pub fn check_include_patterns(&self) -> Result<(), glob::PatternError> {
        if let Some(included) = &self.include {
            for p in included {
                if let Err(e) = glob::Pattern::new(p) {
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    pub fn check_exclude_patterns(&self) -> Result<(), glob::PatternError> {
        if let Some(excluded) = &self.exclude {
            for p in excluded {
                if let Err(e) = glob::Pattern::new(p) {
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    pub fn get_formatter_options(
        &self,
        path: Option<&str>,
    ) -> Result<
        (
            formatter::Options,
            Vec<(Path, formatter::OptionsIncomplete)>,
        ),
        glob::PatternError,
    > {
        let mut opts = formatter::Options::default();

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
                                .filter_map::<Vec<_>, _>(|r| {
                                    if r.paths.is_none() || r.options.formatting.is_none() {
                                        match &r.keys {
                                            Some(k) => Some(
                                                k.iter()
                                                    .map(|s| {
                                                        (
                                                            Path::from_iter(s.split('.')),
                                                            r.options.formatting.clone().unwrap(),
                                                        )
                                                    })
                                                    .collect(),
                                            ),
                                            None => None,
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .flatten()
                                .collect()
                        })
                        .unwrap_or_default(),
                ))
            }
        };

        let mut scoped_opts: Vec<(Path, formatter::OptionsIncomplete)> = Vec::new();

        if let Some(rules) = &self.rule {
            for rule in rules {
                if let Some(f) = &rule.options.formatting {
                    if let Some(paths) = &rule.paths {
                        for p in paths {
                            let pat = glob::Pattern::new(p)?;

                            if pat.matches_with(path, glob_match_options()) {
                                match &rule.keys {
                                    Some(s) => {
                                        if let Some(opts) = &rule.options.formatting {
                                            for key in s {
                                                scoped_opts.push((
                                                    Path::from_iter(key.split('.')),
                                                    opts.clone(),
                                                ))
                                            }
                                        }
                                    }
                                    None => {
                                        opts.update(f.clone());
                                    }
                                }
                            }
                        }
                    } else {
                        match &rule.keys {
                            Some(s) => {
                                if let Some(opts) = &rule.options.formatting {
                                    for key in s {
                                        scoped_opts
                                            .push((Path::from_iter(key.split('.')), opts.clone()))
                                    }
                                }
                            }
                            None => {
                                opts.update(f.clone());
                            }
                        }
                    }
                }
            }
        }

        Ok((opts, scoped_opts))
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Options {
    /// Schema validation options.
    pub schema: Option<SchemaOptions>,
    /// Formatting options.
    pub formatting: Option<formatter::OptionsIncomplete>,
}

/// A rule to override options by either name or file.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Rule {
    /// The name of the rule.
    ///
    /// Used in `taplo::<name>` comments.
    pub name: Option<String>,

    /// Files the rule is valid for.
    ///
    /// A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
    /// Globstars (`**`) are supported.
    ///
    /// If omitted, the rule will always be valid.
    pub paths: Option<Vec<String>>,

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

/// Options for schema validation and completion
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SchemaOptions {
    /// Whether the schema should be enabled or not.
    pub enabled: Option<bool>,

    /// Path to the schema.
    pub path: Option<String>,
}
