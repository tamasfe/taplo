use std::fs;

mod generated {
    mod invalid;
    mod valid;
}

// Requires nightly
#[cfg(feature = "bench")]
mod benches;

// todo
#[test]
fn faszom() {
    let src = fs::read_to_string("../.local/testing.toml").unwrap();

    let d = crate::parser::parse(&src).into_dom();

    if !d.errors().is_empty() {
        println!("\n\nerrs:");
        for e in d.errors() {
            println!("{}", e);
        }
    }
}