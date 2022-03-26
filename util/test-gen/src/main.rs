//! A very simple test generator for Taplo that
//! generates Rust tests from TOML files.

use getopts::Options;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::{env, fs, io::prelude::*, path::Path, process::Command};
use walkdir::WalkDir;

const IGNORED_TESTS: &'static [&'static str] = &[
    "qa-array-inline-nested-1000",
    "qa-table-inline-nested-1000",
    "table-invalid-4"
];

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.reqopt(
        "o",
        "output",
        "output directory for the generated tests",
        "DIR",
    );
    opts.reqopt("i", "input", "input test data directory", "DIR");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            return;
        }
    };

    let output_dir = matches.opt_str("o").unwrap();
    let test_data_dir = matches.opt_str("i").unwrap();

    let valid_dir = Path::new(&test_data_dir).join("valid");
    let invalid_dir = Path::new(&test_data_dir).join("invalid");

    let mut valid_src = quote! {};

    let mut files_in_valid_dir = 0;
    let mut added_valid_files = 0;

    fs::create_dir_all(Path::new(&output_dir).join("generated")).unwrap();

    for valid_file in WalkDir::new(&valid_dir)
        .into_iter()
        .filter_map(|r| match r {
            Ok(entry) => Some(entry),
            Err(e) => {
                println!("ERROR: {}", e);
                None
            }
        })
    {
        if valid_file.path().is_dir() {
            continue;
        }

        files_in_valid_dir += 1;

        if valid_file
            .path()
            .extension()
            .map(|e| e != "toml")
            .unwrap_or(false)
        {
            continue;
        }

        let base_name = valid_file
            .path()
            .file_stem()
            .map(|s| s.to_str().map(ToString::to_string).unwrap_or_default())
            .unwrap_or_default();

        if base_name.is_empty() {
            println!(
                "ERROR: invalid file name for {}",
                valid_file.path().display()
            );
            continue;
        }

        let ignore_attr = if IGNORED_TESTS.contains(&base_name.as_str()) {
            quote!(#[ignore])
        } else {
            quote!()
        };

        let test_name = base_name.replace("-", "_");

        let test_src = match fs::read_to_string(valid_file.path()) {
            Ok(s) => s,
            Err(e) => {
                println!("ERROR: {}", e);
                continue;
            }
        };

        let test_fn_ident = Ident::new(&test_name, Span::call_site());

        let json_validation = quote! {};

        valid_src.extend(quote! {
            #[test]
            #ignore_attr
            fn #test_fn_ident() {
                let src = #test_src;

                let p = crate::parser::parse(&src);

                assert!(
                    p.errors.is_empty(),
                    "Parse errors:\n{}",
                    p.errors
                        .iter()
                        .map(|e| { format!("{}\n", e) })
                        .collect::<String>()
                );

                let dom = p.into_dom();

                assert!(
                    dom.errors().is_empty(),
                    "Semantic errors:\n{}",
                    dom.errors()
                        .iter()
                        .map(|e| { format!("{}\n", e) })
                        .collect::<String>()
                );

                #json_validation
            }
        });

        added_valid_files += 1;
    }

    let mut valid_f =
        fs::File::create(Path::new(&output_dir).join("generated").join("valid.rs")).unwrap();
    write!(&mut valid_f, "{}", valid_src.to_string()).unwrap();

    Command::new("rustfmt")
        .arg(Path::new(&output_dir).join("generated").join("valid.rs"))
        .output()
        .ok();

    println!(
        "Added {} valid toml tests out of {} files.",
        added_valid_files, files_in_valid_dir
    );

    let mut invalid_src = TokenStream::new();

    let mut files_in_invalid_dir = 0;
    let mut added_invalid_files = 0;

    for invalid_file in WalkDir::new(&invalid_dir)
        .into_iter()
        .filter_map(|r| match r {
            Ok(entry) => Some(entry),
            Err(e) => {
                println!("ERROR: {}", e);
                None
            }
        })
    {
        if invalid_file.path().is_dir() {
            continue;
        }

        files_in_invalid_dir += 1;

        if invalid_file
            .path()
            .extension()
            .map(|e| e != "toml")
            .unwrap_or(false)
        {
            continue;
        }

        let base_name = invalid_file
            .path()
            .file_stem()
            .map(|s| {
                s.to_str()
                    .map(ToString::to_string)
                    .unwrap_or_default()
                    .replace("-", "_")
            })
            .unwrap_or_default();

        let ignore_attr = if IGNORED_TESTS.contains(&base_name.as_str()) {
            quote!(#[ignore])
        } else {
            quote!()
        };

        let test_name = base_name.replace("-", "_");

        if test_name.is_empty() {
            println!(
                "ERROR: invalid file name for {}",
                invalid_file.path().display()
            );
            continue;
        }

        let test_src = match fs::read_to_string(invalid_file.path()) {
            Ok(s) => s,
            Err(e) => {
                println!("ERROR: {}", e);
                continue;
            }
        };

        let test_fn_ident = Ident::new(&test_name, Span::call_site());

        invalid_src.extend(quote! {
            #[test]
            #ignore_attr
            fn #test_fn_ident() {
                let src = #test_src;

                let p = crate::parser::parse(&src);

                assert!(!p.errors.is_empty() || !p.into_dom().errors().is_empty());
            }
        });

        added_invalid_files += 1;
    }

    let mut invalid_f =
        fs::File::create(Path::new(&output_dir).join("generated").join("invalid.rs")).unwrap();
    write!(&mut invalid_f, "{}", invalid_src.to_string()).unwrap();

    Command::new("rustfmt")
        .arg(Path::new(&output_dir).join("generated").join("invalid.rs"))
        .output()
        .ok();

    println!(
        "Added {} invalid toml tests out of {} files.",
        added_invalid_files, files_in_invalid_dir
    );
}
