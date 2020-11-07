use crate::{
    config::Config,
    external::{get_paths_by_glob, read_file, read_stdin, write_file},
    print_message,
};
use clap::ArgMatches;
use pretty_lint::Severity;
use taplo::formatter::{self, Options};

pub(crate) struct FormatResult {
    pub matched_document_count: usize,
    pub excluded_document_count: usize,
    pub error_count: usize,
    pub forced: usize,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct CliOptions {
    pub force: bool,
}

pub(crate) fn format(config: Config, m: &ArgMatches) -> FormatResult {
    let mut res = FormatResult {
        matched_document_count: 0,
        error_count: 0,
        excluded_document_count: 0,
        forced: 0,
    };

    let opts = CliOptions {
        force: m.is_present("force"),
    };

    if let Some(files) = m.values_of("files") {
        format_paths(&config, opts, files, &mut res, false);
    } else {
        format_paths(
            &config,
            opts,
            config.get_include_paths().iter().map(|s| s.as_ref()),
            &mut res,
            true,
        );
    }

    res
}

fn format_paths<'i, F: Iterator<Item = &'i str>>(
    config: &Config,
    opts: CliOptions,
    files: F,
    res: &mut FormatResult,
    allow_exclude: bool, // for cli file args
) {
    for val in files {
        if val == "-" {
            let src = match read_stdin() {
                Ok(s) => s,
                Err(e) => {
                    print_message(Severity::Error, "error", &e.to_string());
                    continue;
                }
            };

            let format_opts = match config.get_formatter_options(None, None) {
                Ok(opts) => opts,
                Err(err) => {
                    print_message(Severity::Error, "error", &err.to_string());
                    continue;
                }
            };

            res.matched_document_count += 1;
            match format_source(&src, opts, format_opts, res) {
                Ok(s) => {
                    print!("{}", &s);
                }
                Err(_) => {
                    res.error_count += 1;
                }
            }
            continue;
        }

        match get_paths_by_glob(val) {
            Ok(sources) => {
                for path in sources {
                    if allow_exclude {
                        // Don't format taplo config files unless asked explicitly.
                        if let Some(file_name) = path.file_name() {
                            if file_name == "taplo.toml" || file_name == ".taplo.toml" {
                                // Don't count it as excluded.
                                continue;
                            }
                        }

                        if let Some(p) = path.to_str() {
                            match config.is_excluded(p) {
                                Ok(excluded) => {
                                    if excluded {
                                        res.excluded_document_count += 1;
                                        continue;
                                    }
                                }
                                Err(err) => {
                                    print_message(Severity::Error, "error", &err.to_string());
                                    return;
                                }
                            }
                        }
                    }

                    match read_file(&path) {
                        Ok(src) => {
                            res.matched_document_count += 1;

                            let format_opts =
                                match config.get_formatter_options(path.to_str(), None) {
                                    Ok(opts) => opts,
                                    Err(err) => {
                                        print_message(Severity::Error, "error", &err.to_string());
                                        continue;
                                    }
                                };

                            match format_source(&src, opts, format_opts, res) {
                                Ok(s) => {
                                    if src != s {
                                        match write_file(&path, s.as_bytes()) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                res.error_count += 1;
                                                print_message(
                                                    Severity::Error,
                                                    "error",
                                                    &err.to_string(),
                                                );
                                            }
                                        }
                                    }
                                }
                                Err(_) => res.error_count += 1,
                            }
                        }
                        Err(err) => {
                            print_message(Severity::Error, "error", &err.to_string());
                            res.error_count += 1;
                        }
                    }
                }
            }
            Err(e) => {
                print_message(Severity::Error, "error", &e.to_string());
                break;
            }
        }
    }
}

fn format_source(
    src: &str,
    opts: CliOptions,
    formatter_options: (Options, Vec<(String, formatter::OptionsIncomplete)>),
    res: &mut FormatResult,
) -> Result<String, ()> {
    let parse = taplo::parser::parse(src);

    let had_errors = !parse.errors.is_empty();

    if had_errors {
        if !opts.force {
            return Err(());
        } else {
            res.forced += 1;
        }
    }

    let dom = parse.into_dom();

    Ok(taplo::formatter::format_with_path_scopes(
        dom,
        formatter_options.0,
        formatter_options.1,
    ))
}
