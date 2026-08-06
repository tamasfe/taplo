use crate::dom::{node::DateTimeValue, Node};
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

#[test]
fn get_matches_table_key() {
    let src = r#"
name1 = "v"
other = "v2"
"#;

    let root = parse(src).into_dom();

    let matched: Vec<String> = root
        .get_matches("na*")
        .unwrap()
        .map(|(k, _)| match k {
            crate::dom::KeyOrIndex::Key(key) => key.value().to_string(),
            crate::dom::KeyOrIndex::Index(idx) => idx.to_string(),
        })
        .collect();

    assert_eq!(matched, vec!["name1"]);
}

#[test]
fn datetime_with_leading_zeros() {
    let src = r#"
token_expires_at = 0001-01-01T00:00:00Z
"#;

    let errors = parse(src).errors;

    assert!(errors.is_empty(), "{:#?}", errors);
}

fn dom_of(src: &str) -> Node {
    let parsed = parse(src);
    assert!(parsed.errors.is_empty(), "{src}\n{:#?}", parsed.errors);
    parsed.into_dom()
}

fn date_value(node: &Node) -> DateTimeValue {
    match node {
        Node::Date(d) => d.value(),
        other => panic!("expected a date-time node, got {other:#?}"),
    }
}

fn array_len(src: &str) -> usize {
    match dom_of(src).get("arr") {
        Node::Array(arr) => arr.items().read().len(),
        other => panic!("expected an array, got {other:#?}"),
    }
}

fn local_datetime(h: u8, m: u8, s: u8, nanos: u32) -> DateTimeValue {
    DateTimeValue::LocalDateTime(time::PrimitiveDateTime::new(
        time::Date::from_calendar_date(1979, time::Month::May, 27).unwrap(),
        time::Time::from_hms_nano(h, m, s, nanos).unwrap(),
    ))
}

#[test]
fn local_datetime_keeps_its_value_before_any_terminator() {
    let expected = local_datetime(7, 32, 0, 0);

    for src in [
        "arr = [1979-05-27T07:32:00,]",
        "arr = [1979-05-27T07:32:00 ,]",
        "arr = [1979-05-27T07:32:00]",
        "arr = [1979-05-27T07:32:00, 1]",
        "arr = [1979-05-27T07:32:00,1]",
        "arr = [1979-05-27T07:32:00,2006-06-01]",
        "arr = [1979-05-27t07:32:00,]",
        "arr = [1979-05-27 07:32:00,]",
        "arr = [\n  1979-05-27T07:32:00,\n]",
        "arr = [1979-05-27T07:32:00] # trailing comment",
    ] {
        assert_eq!(
            date_value(&dom_of(src).get("arr").get(0)),
            expected,
            "{src}"
        );
    }

    for src in [
        "t = {a = 1979-05-27T07:32:00, b = 1}",
        "t = {a = 1979-05-27T07:32:00,b = 1}",
        "t = {a = 1979-05-27T07:32:00}",
    ] {
        assert_eq!(
            date_value(&dom_of(src).get("t").get("a")),
            expected,
            "{src}"
        );
    }
}

#[test]
fn comma_is_not_a_fractional_second_separator() {
    // TOML 1.0.0/1.1.0: time-secfrac = "." 1*DIGIT
    for src in [
        "x = 1987-07-05T17:45:00,999Z",
        "x = 1987-07-05T17:45:00,999+07:30",
        "x = 1979-05-27T07:32:00,999",
        "x = 11:00:00,999",
    ] {
        assert!(!parse(src).errors.is_empty(), "{src} must not parse");
    }
}

#[test]
fn period_is_still_a_fractional_second_separator() {
    assert_eq!(
        date_value(&dom_of("x = 1979-05-27T07:32:00.999").get("x")),
        local_datetime(7, 32, 0, 999_000_000)
    );
    assert_eq!(
        date_value(&dom_of("x = 1987-07-05T17:45:00.999Z").get("x")),
        DateTimeValue::OffsetDateTime(time::OffsetDateTime::new_utc(
            time::Date::from_calendar_date(1987, time::Month::July, 5).unwrap(),
            time::Time::from_hms_nano(17, 45, 0, 999_000_000).unwrap(),
        ))
    );
    assert_eq!(
        date_value(&dom_of("x = 11:00:00.999").get("x")),
        DateTimeValue::Time(time::Time::from_hms_nano(11, 0, 0, 999_000_000).unwrap())
    );
}

#[test]
fn datetime_does_not_swallow_the_following_array_element() {
    assert_eq!(array_len("arr = [11:00:00,1]"), 2);
    assert_eq!(array_len("arr = [1979-05-27T07:32:00,2006-06-01]"), 2);
    assert_eq!(array_len("arr = [1987-07-05T17:45:00Z,1]"), 2);
    assert_eq!(array_len("arr = [11:00:00,]"), 1);
}
