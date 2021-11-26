#![cfg_attr(all(target_arch = "wasm32", feature = "nightly"), feature(set_stdio))]

use clap::{App, AppSettings, Arg, ArgMatches};
use external::{download_schema_index, load_config, load_schema_index, update_schemas};
use once_cell::sync::Lazy;
use pretty_lint::{
    colored::{self, Colorize},
    Severity,
};
use std::{
    ffi::OsString,
    sync::atomic::{AtomicBool, Ordering},
};
use taplo::formatter;
use util::cache_schema_index;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod config;

#[cfg(target_arch = "wasm32")]
#[path = "external/wasm32/mod.rs"]
mod external;

#[cfg(not(target_arch = "wasm32"))]
#[path = "external/native/mod.rs"]
mod external;

mod format;
mod lint;

pub mod util;

static ERROR_STATUS: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static SILENT_OUTPUT: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static WARN_AS_ERROR: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

/// For exit conditions.
pub(crate) fn is_error() -> bool {
    ERROR_STATUS.load(Ordering::SeqCst)
}

/// For exit conditions.
pub(crate) fn set_error(err: bool) {
    ERROR_STATUS.store(err, Ordering::SeqCst);
}

pub(crate) fn is_silent() -> bool {
    SILENT_OUTPUT.load(Ordering::SeqCst)
}
pub(crate) fn set_silent(s: bool) {
    SILENT_OUTPUT.store(s, Ordering::SeqCst);
}

pub(crate) fn is_warn_as_error() -> bool {
    WARN_AS_ERROR.load(Ordering::SeqCst)
}
pub(crate) fn set_warn_as_error(e: bool) {
    WARN_AS_ERROR.store(e, Ordering::SeqCst);
}

pub(crate) fn print_message(severity: Severity, name: &str, message: &str) {
    match severity {
        Severity::Error => {
            eprintln!("{}", &format!("{}: {}", name.red().bold(), message.bold()));
            set_error(true);
        }
        Severity::Warning => {
            if is_warn_as_error() {
                set_error(true)
            }

            if is_silent() {
                return;
            }

            eprintln!(
                "{}",
                &format!("{}: {}", name.yellow().bold(), message.bold())
            )
        }
        Severity::Info => {
            if is_silent() {
                return;
            }
            eprintln!(
                "{}",
                &format!("{}: {}", name.bright_blue().bold(), message.bold())
            )
        }
        Severity::Success => {
            if is_silent() {
                return;
            }
            eprintln!(
                "{}",
                &format!("{}: {}", name.green().bold(), message.bold())
            )
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[allow(improper_ctypes_definitions)]
pub async extern fn run_node(args: JsValue) -> bool {
    set_panic_hook();
    set_node_out();

    let args = args.into_serde::<Vec<String>>().unwrap();

    run(args).await
}

const SCHEMA_REPOSITORY: &str = "https://taplo.tamasfe.dev/schema_index.json";

pub async fn run<I, T>(itr: I) -> bool
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    if let Some(colors) = external::colored_output() {
        colored::control::set_override(colors);
    }

    let mut app = App::new("Taplo TOML Utility")
        .author("tamasfe (https://github.com/tamasfe)")
        .bin_name("taplo")
        .version(concat!(env!("CARGO_PKG_VERSION"), " (", env!("BUILD_TARGET"), ")"))
        .about("A TOML linter and formatter tool")
        .long_about("A TOML linter and formatter tool (https://taplo.tamasfe.dev)")
        .arg(
            Arg::new("colors")
                .long("colors")
                .about("Set when to colorize output")
                .takes_value(true)
                .possible_values(&["auto", "always", "never"])
                .default_value("auto")
                .global(true)
        )
        .arg(
            Arg::new("silent")
                .long("silent")
                .about("Disable non-error output")
                .global(true)
        )
        .arg(
            Arg::new("warn-as-error")
                .long("warn-as-error")
                .about("Treat warnings as errors")
                .global(true)
        )
        .subcommand(
            App::new("format")
                .visible_alias("fmt")
                .about("Format TOML documents in place")
                .long_about("Format TOML documents. Files are modified in-place unless the input comes from the standard input, in which case the formatted result is printed to the standard output")
                .arg(
                    Arg::new("files")
                        .about("Paths or glob patterns to TOML documents")
                        .long_about("Paths or glob patterns to TOML documents, can be omitted if a configuration file is provided or found that provides document paths")
                )
                .arg(
                    Arg::new("stdin")
                    .about("Provide a TOML document from the standard input")
                    .long_about("Provide a TOML document from the standard input. If this is supplied, the formatted document will be written to the standard output")
                    .possible_value("-")
                    .conflicts_with("files"),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .about("Ignore syntax errors and format anyway (potentially destructive)"),
                )
                .arg(
                    Arg::new("options")
                        .short('o')
                        .long("options")
                        .about("A comma-separated list of key=value pairs to pass to the formatter")
                        .long_about("A comma-separated list of key=value pairs to pass to the formatter. The valid options and values are available here: https://taplo.tamasfe.dev/configuration/#formatting-options")
                        .takes_value(true)
                        .multiple_occurrences(true)
                )
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .about("Path to the Taplo configuration file")
                        .value_name("PATH")
                        .takes_value(true)
                ).arg(
                    Arg::new("check")
                        .long("check")
                        .about("Return NonZero exit code if there are any format issues")
                )
        )
        .subcommand(
            App::new("lint")
            .visible_alias("check")
            .about("Lint TOML documents")
            .arg(
                Arg::new("schema")
                .about("Provide a JSON Schema for validation")
                .long_about("Provide a Schema for validation. This schema will be used for all documents. Only JSON format is accepted.")
                .short('s')
                .long("schema")
                .takes_value(true)
            )
            .arg(
                Arg::new("config")
                    .short('c')
                    .long("config")
                    .about("Path to the Taplo configuration file")
                    .value_name("PATH")
                    .takes_value(true)
            )
            .arg(
                Arg::new("files")
                    .about(r#"Paths or glob patterns to TOML documents, or "-" for standard input"#)
                    .long_about("Paths or glob patterns to TOML documents, can be omitted if a configuration file is provided or found that provides document paths.")
                    .multiple(true)
            )
            .arg(
                Arg::new("cache-path")
                    .long("cache-path")
                    .about("Path to a cache folder, if omitted no caching will be done")
                    .takes_value(true)
            )
            .arg(
                Arg::new("default-schema-repository")
                    .short('S')
                    .long("default-schema-repository")
                    .about("Use the default remote schema repository")
                    .conflicts_with("schema-repository")
                    .takes_value(false)
            )
            .arg(
                Arg::new("schema-repository")
                    .long("schema-repository")
                    .about("Use a remote schema repository")
                    .takes_value(true)
            )
        )
        .subcommand(
            App::new("config")
                .about("Configuration-related commands")
                .subcommand(
                    App::new("schema")
                        .about("Print the configuration JSON schema")
                )
                .subcommand(
                    App::new("example")
                        .about("Print an example configuration")
                )
        )
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::NoBinaryName);

    match app.try_get_matches_from_mut(itr) {
        Ok(matches) => execute(matches).await,
        Err(err) => {
            eprintln!("{}", err);
            matches!(
                err.kind,
                clap::ErrorKind::DisplayHelp | clap::ErrorKind::DisplayVersion
            )
        }
    }
}

async fn execute(matches: ArgMatches) -> bool {
    match matches.value_of("colors") {
        Some("always") => colored::control::set_override(true),
        Some("never") => colored::control::set_override(false),
        _ => {}
    }

    if matches.is_present("silent") {
        set_silent(true);
    }

    if matches.is_present("warn-as-error") {
        set_warn_as_error(true);
    }

    let success = match matches.subcommand() {
        Some(("config", config_matches)) => match config_matches.subcommand() {
            Some(("schema", _)) => {
                let schema = schemars::schema_for!(config::Config);

                print!("{}", serde_json::to_string_pretty(&schema).unwrap());
                !is_error()
            }
            Some(("example", _)) => {
                let mut c = config::Config::default();

                c.global_options.formatting = Some(formatter::OptionsIncomplete::from_options(
                    formatter::Options::default(),
                ));

                print!("{}", toml::to_string_pretty(&c).unwrap());
                !is_error()
            }
            _ => unreachable!(),
        },
        Some(("format", format_matches)) => {
            let config = match load_config(format_matches.value_of("config")).await {
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

            if let Err(e) = config.check_patterns() {
                print_message(
                    Severity::Error,
                    "error",
                    &format!("invalid glob pattern in config: {}", e),
                );
                return false;
            }

            let format_result = format::format(config, format_matches).await;

            if format_result.matched_document_count == 0 {
                print_message(Severity::Warning, "warning", "no documents were found");
            }

            if format_result.error_count > 0 {
                print_message(
                    Severity::Error,
                    "failure",
                    &format!(
                        "processed {} {docs} with {} {errors} {excluded}",
                        format_result.matched_document_count
                            - format_result.excluded_document_count,
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
                return false;
            }

            if format_matches.is_present("check") && format_result.different_document_count > 0 {
                print_message(
                    Severity::Error,
                    "failure",
                    &format!(
                        "processed {} {docs} with {} different {docs} {excluded}",
                        format_result.matched_document_count
                            - format_result.excluded_document_count,
                        format_result.different_document_count,
                        docs = if format_result.matched_document_count != 1 {
                            "documents"
                        } else {
                            "document"
                        },
                        excluded = if format_result.matched_document_count != 1 {
                            "documents"
                        } else {
                            "document"
                        },
                    ),
                );
                return false;
            }

            print_message(
                Severity::Success,
                "success",
                &format!(
                    "processed {} {docs} with no errors {excluded}",
                    format_result.matched_document_count - format_result.excluded_document_count,
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
            !is_error()
        }
        Some(("lint", lint_matches)) => {
            let config = match load_config(lint_matches.value_of("config")).await {
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

            let schema_repository = if lint_matches.is_present("default-schema-repository") {
                Some(SCHEMA_REPOSITORY)
            } else {
                lint_matches.value_of("schema-repository")
            };

            let mut schema_index = None;
            if let Some(v) = schema_repository {
                print_message(
                    Severity::Info,
                    "info",
                    &format!("updating schema index from {}", v),
                );
                match download_schema_index(v).await {
                    Ok(idx) => {
                        if let Some(v) = lint_matches.value_of("cache-path") {
                            if let Err(err) = cache_schema_index(&idx, v).await {
                                print_message(
                                    Severity::Warning,
                                    "warning",
                                    &format!("failed to save schema index: {}", err),
                                );
                            }
                        }

                        schema_index = Some(idx)
                    }
                    Err(err) => {
                        if is_warn_as_error() {
                            print_message(
                                Severity::Error,
                                "error",
                                &format!("failed to download schema index: {}", err),
                            );
                            return false;
                        } else {
                            print_message(
                                Severity::Warning,
                                "warning",
                                &format!("failed to download schema index: {}", err),
                            );
                        }
                    }
                };
            }

            if let Some(cache_path) = lint_matches.value_of("cache-path") {
                match &schema_index {
                    Some(idx) => {
                        print_message(Severity::Info, "info", "checking for schema updates");
                        let (updated, errors) = update_schemas(idx, cache_path).await;
                        if updated > 0 {
                            print_message(
                                Severity::Info,
                                "info",
                                &format!("updated remote {} schemas", updated),
                            );
                        }
                        if !errors.is_empty() {
                            print_message(
                                Severity::Warning,
                                "warn",
                                &format!("failed to update {} schemas", errors.len()),
                            );
                        }
                    }
                    None => match load_schema_index(cache_path).await {
                        Ok(idx) => schema_index = idx,
                        Err(err) => {
                            print_message(
                                Severity::Warning,
                                "warning",
                                &format!("failed to read schema index: {}", err),
                            );
                        }
                    },
                }
            }

            if let Err(e) = config.check_patterns() {
                print_message(
                    Severity::Error,
                    "error",
                    &format!("invalid glob pattern in config: {}", e),
                );
                return false;
            }

            let lint_result = lint::lint(config, lint_matches, schema_index).await;

            if lint_result.matched_document_count == 0 {
                print_message(Severity::Warning, "warning", "no documents were found");
            }

            if lint_result.error_count > 0 {
                print_message(
                    Severity::Error,
                    "failure",
                    &format!(
                        "processed {} {docs} with {} {errors} {excluded}",
                        lint_result.matched_document_count - lint_result.excluded_document_count,
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
                        "processed {} {docs} with no errors {excluded}",
                        lint_result.matched_document_count - lint_result.excluded_document_count,
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
                !is_error()
            }
        }
        _ => unreachable!(),
    };

    if !success && is_warn_as_error() {
        print_message(
            Severity::Error,
            "failure",
            r#"warnings occurred ("--warn-as-error" flag was used)"#,
        );
    }

    success
}

#[cfg(target_arch = "wasm32")]
fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(all(target_arch = "wasm32", not(feature = "nightly")))]
fn set_node_out() {}

#[cfg(all(target_arch = "wasm32", feature = "nightly"))]
fn set_node_out() {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern {
        #[wasm_bindgen(js_namespace = ["process", "stdout"], js_name = write)]
        fn node_stdout(b: &[u8]);

        #[wasm_bindgen(js_namespace = ["process", "stderr"], js_name = write)]
        fn node_stderr(b: &[u8]);
    }

    struct NodeStdOut;
    impl std::io::Write for NodeStdOut {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            node_stdout(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct NodeStdErr;
    impl std::io::Write for NodeStdErr {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            node_stderr(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    std::io::set_print(Some(Box::new(NodeStdOut)));
    std::io::set_panic(Some(Box::new(NodeStdErr)));
}
