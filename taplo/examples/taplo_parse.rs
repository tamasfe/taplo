use std::fs;

fn main() {
    let src = fs::read_to_string("taplo-ide/vscode/sample/example-v0.4.0.toml").unwrap();
    println!("{}", taplo::parser::parse(&src).into_dom())
}
