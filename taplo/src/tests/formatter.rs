use crate::formatter;

#[test]
fn comment_indentation() {
    let formatted = crate::formatter::format(
        r#"# aaasd

[profile]

# asd
   # asd

# bsd 
 # bsd

# csd
    [profile.release]

    incremental = true 
    debug = 0 # Set this to 1 or 2 to get more useful backtraces in debugger.

    # asd"#,
        formatter::Options {
            indent_tables: true,
            ..Default::default()
        },
    );

    let expected = r#"# aaasd

[profile]

# asd
# asd

# bsd 
# bsd

  # csd
  [profile.release]

  incremental = true
  debug = 0 # Set this to 1 or 2 to get more useful backtraces in debugger.

  # asd
"#;

    assert_eq!(formatted, expected);
}

#[test]
fn comment_after_entry() {
    let src = r#"incremental = true
debug = 0 # Set this to 1 or 2 to get more useful backtraces in debugger.
"#;

    let formatted = crate::formatter::format(src, formatter::Options::default());

    assert_eq!(src, formatted);
}

#[test]
fn comment_before_entry() {
    let src = r#"

# hello
[lib]
# bello
incremental = true
"#;

    let formatted = crate::formatter::format(src, formatter::Options::default());

    assert_eq!(src, formatted);
}
