use crate::{
    config::Config,
    external::{get_paths_by_glob, read_file, read_stdin, write_file},
    print_message,
};
use anyhow::anyhow;
use clap::ArgMatches;
use pretty_lint::Severity;
use std::collections::HashSet;
use taplo::formatter::{self, Options};

pub(crate) struct FormatResult {
    pub matched_document_count: usize,
    pub excluded_document_count: usize,
    pub different_document_count: usize,
    pub error_count: usize,
    pub forced: usize,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct CliOptions {
    pub force: bool,
    pub check: bool,
}

pub(crate) async fn format(config: Config, m: &ArgMatches) -> FormatResult {
    let mut res = FormatResult {
        matched_document_count: 0,
        excluded_document_count: 0,
        different_document_count: 0,
        error_count: 0,
        forced: 0,
    };

    let opts = CliOptions {
        force: m.is_present("force"),
        check: m.is_present("check"),
    };

    let mut cli_opts = None;

    if let Some(values) = m.values_of("options") {
        match collect_cli_opts(values) {
            Ok(values) => cli_opts = Some(values),
            Err(e) => {
                print_message(
                    Severity::Error,
                    "error",
                    &format!("invalid formatting options: {}", e),
                );
                res.error_count += 1;
                return res;
            }
        }
    }

    if let Some(files) = m.values_of("files") {
        format_paths(&config, opts, files, &mut res, false, cli_opts).await;
    } else {
        format_paths(
            &config,
            opts,
            config.get_include_paths().iter().map(|s| s.as_ref()),
            &mut res,
            true,
            cli_opts,
        )
        .await;
    }

    res
}

fn collect_cli_opts<'i, I: Iterator<Item = &'i str>>(
    cli_values: I,
) -> Result<Vec<(String, String)>, anyhow::Error> {
    let mut existing: HashSet<&str> = HashSet::new();
    let mut values = Vec::new();

    for val in cli_values {
        let comma_split = val.split(',');

        for key_value in comma_split {
            let mut eq_split = key_value.split('=');

            let opt = eq_split
                .next()
                .ok_or_else(|| anyhow!("expected option name"))?
                .trim_matches('"')
                .trim_matches('\'');

            let val = eq_split
                .next()
                .ok_or_else(|| anyhow!("expected option value"))?
                .trim_matches('"')
                .trim_matches('\'');

            if !existing.insert(opt) {
                return Err(anyhow!("duplicate option: {}", opt));
            }

            values.push((opt.into(), val.into()));
        }
    }

    Ok(values)
}

async fn format_paths<'i, F: Iterator<Item = &'i str>>(
    config: &Config,
    opts: CliOptions,
    files: F,
    res: &mut FormatResult,
    allow_exclude: bool, // for cli file args
    cli_format_opts: Option<Vec<(String, String)>>,
) {
    for val in files {
        if val == "-" {
            let src = match read_stdin() {
                Ok(s) => s,
                Err(e) => {
                    print_message(Severity::Error, "error", &e.to_string());
                    res.error_count += 1;
                    continue;
                }
            };

            let mut format_opts = match config.get_formatter_options(None, None) {
                Ok(opts) => opts,
                Err(err) => {
                    print_message(Severity::Error, "error", &err.to_string());
                    res.error_count += 1;
                    continue;
                }
            };

            if let Some(cli_format_opts) = &cli_format_opts {
                if let Err(err) = format_opts
                    .0
                    .update_from_str(cli_format_opts.iter().map(|s| (&s.0, &s.1)))
                {
                    print_message(Severity::Error, "error", &err.to_string());
                    res.error_count += 1;
                    return;
                }
            }

            res.matched_document_count += 1;
            match format_source(&src, opts, format_opts, res) {
                Ok(s) => {
                    if src != s {
                        res.different_document_count += 1;
                    }

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
                                    res.error_count += 1;
                                    return;
                                }
                            }
                        }
                    }

                    match read_file(path.to_str().unwrap()).await {
                        Ok(src) => {
                            let src = match String::from_utf8(src) {
                                Ok(src) => src,
                                Err(err) => {
                                    print_message(
                                        Severity::Error,
                                        "error",
                                        &format!("file {:?} is not valid UTF-8: {}", path, err),
                                    );
                                    res.error_count += 1;
                                    continue;
                                }
                            };

                            res.matched_document_count += 1;

                            let mut format_opts =
                                match config.get_formatter_options(path.to_str(), None) {
                                    Ok(opts) => opts,
                                    Err(err) => {
                                        print_message(Severity::Error, "error", &err.to_string());
                                        res.error_count += 1;
                                        continue;
                                    }
                                };

                            if let Some(cli_format_opts) = &cli_format_opts {
                                if let Err(err) = format_opts
                                    .0
                                    .update_from_str(cli_format_opts.iter().map(|s| (&s.0, &s.1)))
                                {
                                    print_message(Severity::Error, "error", &err.to_string());
                                    res.error_count += 1;
                                    return;
                                }
                            }

                            match format_source(&src, opts, format_opts, res) {
                                Ok(s) => {
                                    if src != s {
                                        res.different_document_count += 1;
                                        if !opts.check {
                                            match write_file(path.to_str().unwrap(), s.as_bytes())
                                                .await
                                            {
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
                res.error_count += 1;
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
