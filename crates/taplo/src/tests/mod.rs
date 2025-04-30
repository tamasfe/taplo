use crate::parser::parse;

mod generated {
    mod invalid;
}

mod formatter;

#[test]
fn time_in_arrays() {
    let src = r#"
    a = [00:00:01, 02:03:04]
    "#;

    let errors = parse(src).errors;

    assert!(errors.is_empty(), "{:#?}", errors);
}

#[test]
fn comments_after_tables() {
    let src = r#"
[[array]] # foo
[table] # foo
"#;
    let errors = parse(src).errors;

    assert!(errors.is_empty(), "{:#?}", errors);
}

#[test]
fn dates_in_table_keys() {
    let src = r#"
[2024-01-01]
2024-01-01 = true

[[2024-01-02]]
2024-01-01 = true
"#;
    let errors = parse(src).errors;

    assert!(errors.is_empty(), "{:#?}", errors);
}
