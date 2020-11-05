#![cfg_attr(
    any(target_arch = "wasm32", feature = "_internal_wasm_testing"),
    feature(set_stdio)
)]

use clap::{App, AppSettings, Arg, ArgMatches};
use external::load_config;
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

#[cfg(any(target_arch = "wasm32", feature = "_internal_wasm_testing"))]
use wasm_bindgen::prelude::*;

pub mod config;

#[cfg(any(target_arch = "wasm32", feature = "_internal_wasm_testing"))]
#[path = "external/wasm32/mod.rs"]
mod external;

#[cfg(not(any(target_arch = "wasm32", feature = "_internal_wasm_testing")))]
#[path = "external/native/mod.rs"]
mod external;

mod format;
mod lint;

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

#[cfg(any(target_arch = "wasm32", feature = "_internal_wasm_testing"))]
#[wasm_bindgen]
#[allow(improper_ctypes_definitions)]
pub async extern fn run_node(args: JsValue) -> bool {
    set_panic_hook();
    set_node_out();

    let args = args.into_serde::<Vec<String>>().unwrap();

    run(args).await
}

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
        .version(env!("CARGO_PKG_VERSION"))
        .about("A TOML linter and formatter tool.")
        .long_about("A TOML linter and formatter tool (https://github.com/tamasfe/taplo).")
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
                )
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .about("Path to the Taplo configuration file")
                        .value_name("PATH")
                        .takes_value(true)
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
            false
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
            let config = match load_config(format_matches.value_of("config")) {
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

            let format_result = format::format(config, format_matches);

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
                false
            } else {
                print_message(
                    Severity::Success,
                    "success",
                    &format!(
                        "processed {} {docs} with no errors {excluded}",
                        format_result.matched_document_count
                            - format_result.excluded_document_count,
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
        }
        Some(("lint", lint_matches)) => {
            let config = match load_config(lint_matches.value_of("config")) {
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

            let lint_result = lint::lint(config, lint_matches).await;

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

#[cfg(all(
    any(target_arch = "wasm32", feature = "_internal_wasm_testing"),
    feature = "console_error_panic_hook"
))]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(any(target_arch = "wasm32", feature = "_internal_wasm_testing"))]
pub fn set_node_out() {
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
