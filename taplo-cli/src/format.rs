use crate::{
    config::Config,
    external::{
        get_paths_by_glob, get_stdin_source, print_message, print_stdout, read_file, write_file,
    },
};
use clap::ArgMatches;
use pretty_lint::Severity;
use taplo::formatter::Options;

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
        format_paths(&config, opts, files, &mut res);
    } else {
        format_paths(
            &config,
            opts,
            config.get_include_paths().iter().map(|s| s.as_ref()),
            &mut res,
        );
    }

    res
}

fn format_paths<'i, F: Iterator<Item = &'i str>>(
    config: &Config,
    opts: CliOptions,
    files: F,
    res: &mut FormatResult,
) {
    for val in files {
        if val == "-" {
            let src = match get_stdin_source() {
                Ok(s) => s,
                Err(e) => {
                    print_message(Severity::Error, "error", &e.to_string());
                    continue;
                }
            };

            let format_opts = match config.get_formatter_options(None) {
                Ok(opts) => opts,
                Err(err) => {
                    print_message(Severity::Error, "error", &err.to_string());
                    continue;
                }
            };

            res.matched_document_count += 1;
            match format_source(&src, opts, format_opts, res) {
                Ok(s) => {
                    print_stdout(&s);
                }
                Err(_) => {
                    res.error_count += 1;
                }
            }
            continue;
        }

        match config.is_excluded(val) {
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

        match get_paths_by_glob(val) {
            Ok((sources, errors)) => {
                for err in errors {
                    print_message(Severity::Error, "error", &err.to_string());
                    res.error_count += 1;
                }

                for path in sources {
                    match read_file(&path) {
                        Ok(src) => {
                            res.matched_document_count += 1;

                            let format_opts = match config.get_formatter_options(path.to_str()) {
                                Ok(opts) => opts,
                                Err(err) => {
                                    print_message(Severity::Error, "error", &err.to_string());
                                    continue;
                                }
                            };

                            match format_source(&src, opts, format_opts, res) {
                                Ok(s) => match write_file(&path, s) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        res.error_count += 1;
                                        print_message(Severity::Error, "error", &err.to_string());
                                    }
                                },
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
    formatter_options: Options,
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

    Ok(taplo::formatter::format_green(
        parse.green_node,
        formatter_options,
    ))
}
