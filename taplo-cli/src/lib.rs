use clap::{App, Arg, ArgMatches};
use external::{is_windows, load_config, print_message, print_stdout};
use pretty_lint::Severity;
use std::ffi::OsString;

mod config;
mod external;
mod format;
mod lint;

fn glob_match_options() -> glob::MatchOptions {
    glob::MatchOptions {
        case_sensitive: is_windows(),
        require_literal_leading_dot: false,
        require_literal_separator: false,
    }
}

pub fn run<I, T>(itr: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let app = App::new("Taplo TOML Utility")
    .author("tamasfe (https://github.com/tamasfe)")
    .bin_name("taplo")
    .version(env!("CARGO_PKG_VERSION"))
    .about("A TOML linter and formatter tool.")
    .long_about("A TOML linter and formatter tool (https://github.com/tamasfe/taplo).")
    .arg(
        Arg::new("config")
            .short('c')
            .long("config")
            .about("Path to the Taplo configuration file")
            .value_name("PATH")
            .takes_value(true)
            .global(true),
    )
    .subcommand(
        App::new("format")
            .visible_alias("fmt")
            .about("Format TOML documents")
            .arg(
                Arg::new("files")
                    .about("Paths or glob patterns to TOML documents")
                    .long_about("Paths or glob patterns to TOML documents, can be omitted if a configuration file is provided or found that provides document paths.")
            )
            .arg(
                Arg::new("stdin")
                .about("Provide a TOML document from the standard input")
                .long_about("Provide a TOML document from the standard input. If this is supplied, the formatted document will be written to the standard output.")
                .possible_value("-")
                .conflicts_with("files"),
            )
            .arg(
                Arg::new("force")
                    .short('f')
                    .long("force")   
                    .about("Ignore syntax errors and format anyway (potentially destructive)"),
            ),
    )
    .subcommand(
        App::new("lint")
        .visible_alias("check")
        .about("Lint TOML documents")
        .arg(
            Arg::new("schema")
            .about("Provide a JSON Schema for validation")
            .long_about("Provide a JSON Schema for validation. This schema will be used for all documents. Only JSON format is accepted.")
            .short('s')
            .long("schema")
            .takes_value(true)
        )
        .arg(
            Arg::new("files")
                .about(r#"Paths or glob patterns to TOML documents, or "-" for standard input"#)
                .long_about("Paths or glob patterns to TOML documents, can be omitted if a configuration file is provided or found that provides document paths.")
                .multiple(true)
        )
    )
    .subcommand(
        App::new("config")
            .about("Configuration-related commands")
            .subcommand(
                App::new("schema")
                    .about("Print the configuration JSON schema")
            )
    );

    execute(app.get_matches_from(itr));
}

fn execute(matches: ArgMatches) -> bool {
    let config = match load_config(matches.value_of("config")) {
        Ok(c) => c,
        Err(err) => {
            print_message(
                Severity::Error,
                "error",
                &format!("failed to read configuration: {}", err),
            );
            return false;
        }
    };

    if let Err(e) = config.check_exclude_patterns() {
        print_message(
            Severity::Error,
            "error",
            &format!("invalid exclude glob pattern: {}", e),
        );
        return false;
    }

    if let Err(e) = config.check_include_patterns() {
        print_message(
            Severity::Error,
            "error",
            &format!("invalid include glob pattern: {}", e),
        );
        return false;
    }

    match matches.subcommand() {
        Some(("config", config_matches)) => match config_matches.subcommand() {
            Some(("schema", _)) => {
                let schema = schemars::schema_for!(config::Config);

                print_stdout(&serde_json::to_string_pretty(&schema).unwrap());
                true
            }
            _ => unreachable!(),
        },
        Some(("format", format_matches)) => {
            let format_result = format::format(config, format_matches);

            if format_result.error_count > 0 {
                print_message(
                    Severity::Error,
                    "failure",
                    &format!(
                        "found {} {docs} with {} {errors}.{excluded}",
                        format_result.matched_document_count,
                        format_result.error_count,
                        docs = if format_result.matched_document_count != 1 {
                            "documents"
                        } else {
                            "document"
                        },
                        errors = if format_result.error_count != 1 {
                            "errors"
                        } else {
                            "error"
                        },
                        excluded = if format_result.excluded_document_count > 0 {
                            format!(" (excluded {})", format_result.excluded_document_count)
                        } else {
                            format!("")
                        }
                    ),
                );
                false
            } else {
                print_message(
                    Severity::Success,
                    "success",
                    &format!(
                        "found {} {docs} with no errors.{excluded}",
                        format_result.matched_document_count,
                        docs = if format_result.matched_document_count != 1 {
                            "documents"
                        } else {
                            "document"
                        },
                        excluded = if format_result.excluded_document_count > 0 {
                            format!(" (excluded {})", format_result.excluded_document_count)
                        } else {
                            format!("")
                        }
                    ),
                );
                true
            }
        }
        Some(("lint", lint_matches)) => {
            let lint_result = lint::lint(config, lint_matches);

            if lint_result.error_count > 0 {
                print_message(
                    Severity::Error,
                    "failure",
                    &format!(
                        "found {} {docs} with {} {errors}.{excluded}",
                        lint_result.matched_document_count,
                        lint_result.error_count,
                        docs = if lint_result.matched_document_count != 1 {
                            "documents"
                        } else {
                            "document"
                        },
                        errors = if lint_result.error_count != 1 {
                            "errors"
                        } else {
                            "error"
                        },
                        excluded = if lint_result.excluded_document_count > 0 {
                            format!(" (excluded {})", lint_result.excluded_document_count)
                        } else {
                            format!("")
                        }
                    ),
                );
                false
            } else {
                print_message(
                    Severity::Success,
                    "success",
                    &format!(
                        "found {} {docs} with no errors.{excluded}",
                        lint_result.matched_document_count,
                        docs = if lint_result.matched_document_count != 1 {
                            "documents"
                        } else {
                            "document"
                        },
                        excluded = if lint_result.excluded_document_count > 0 {
                            format!(" (excluded {})", lint_result.excluded_document_count)
                        } else {
                            format!("")
                        }
                    ),
                );
                true
            }
        }
        _ => unreachable!(),
    }
}
