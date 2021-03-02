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

#[test]
fn align_composite_entries() {
    let src = r#"k1 = 1                                                      # 111
k2 = false                                                  # 222
k3 = "public"                                               # 333
k4 = ["/home/www", "/var/lib/www"] # 4444444444444444444444
k6 = {a="yes", table="yes"} # 4444444444444444444444
k5 = false                                                  # 555
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            align_entries: true,
            ..Default::default()
        },
    );

    assert_eq!(
        r#"k1 = 1                             # 111
k2 = false                         # 222
k3 = "public"                      # 333
k4 = ["/home/www", "/var/lib/www"] # 4444444444444444444444
k6 = { a = "yes", table = "yes" }  # 4444444444444444444444
k5 = false                         # 555
"#,
        formatted
    );
}

#[test]
fn test_space_in_line() {
    let src = r#" 
[foo]
 
foo = "bar"
 
bar = "foo"
 

 

 

[bar]
foo = "bar"
"#;
    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            align_entries: true,
            ..Default::default()
        },
    );

    assert_eq!(
        r#"
[foo]

foo = "bar"

bar = "foo"


[bar]
foo = "bar"
"#,
        formatted
    );
}
