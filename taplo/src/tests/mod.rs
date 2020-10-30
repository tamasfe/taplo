use std::fs;

mod generated {
    mod invalid;
    mod valid;
}

#[test]
fn rewrite() {
    let src = fs::read_to_string("../taplo-ide/vscode/sample/example-v0.4.0.toml").unwrap();

}