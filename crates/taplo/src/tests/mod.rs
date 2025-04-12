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
fn inline_table_with_linebreaks_and_trailing_comma() {
    let src = r#"
cooldowns = { 
    foo = "foo",
    bar = "bar",
}
"#;
    let errors = parse(src).errors;

    assert!(errors.is_empty(), "{:#?}", errors);
}
