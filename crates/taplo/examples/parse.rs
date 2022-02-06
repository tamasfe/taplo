fn main() {
    const SOURCE: &str = "value = 1
value = 2

[table]
string = 'some string'";

    let parse_result = taplo::parser::parse(SOURCE);

    // Check for syntax errors.
    // These are not carried over to DOM errors.
    assert!(parse_result.errors.is_empty());

    // let root_node = parse_result.into_dom();

    // Check for semantic errors.
    // In this example "value" is a duplicate key.
    // assert_eq!(root_node.errors().len(), 1);
}
