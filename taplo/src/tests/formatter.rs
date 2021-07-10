use difference::Changeset;

use crate::formatter;

macro_rules! assert_format {
    ($expected:expr, $actual:expr) => {
        if $expected != $actual {
            println!("{}", Changeset::new($actual, $expected, "\n"));
            panic!("invalid formatting");
        }
    };
}

#[test]
fn comment_indentation() {
    let formatted = crate::formatter::format(
        r#"# aaasd

[profile]

# asd
   # asd

# bsd 
 # bsd
asd = ""

# csd
    [profile.release]

    incremental  = true 
    lol = 2 #yo
    debug = 0          # Set this to 1 or 2 to get more useful backtraces in debugger.

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
asd = ""

  # csd
  [profile.release]

  incremental = true
  lol = 2            #yo
  debug = 0          # Set this to 1 or 2 to get more useful backtraces in debugger.

  # asd
"#;
    assert_format!(expected, &formatted);
}

#[test]
fn comment_after_entry() {
    let expected = r#"incremental = true

debug = 0 # Set this to 1 or 2 to get more useful backtraces in debugger.
"#;

    let formatted = crate::formatter::format(expected, formatter::Options::default());

    assert_format!(expected, &formatted);
}

#[test]
fn comment_before_entry() {
    let expected = r#"

# hello
[lib]
# bello
incremental = true
"#;

    let formatted = crate::formatter::format(expected, formatter::Options::default());

    assert_format!(expected, &formatted);
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

    let expected = r#"k1 = 1                             # 111
k2 = false                         # 222
k3 = "public"                      # 333
k4 = ["/home/www", "/var/lib/www"] # 4444444444444444444444
k6 = { a = "yes", table = "yes" }  # 4444444444444444444444
k5 = false                         # 555
"#;

    assert_format!(expected, &formatted);
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

    let expected = r#"
[foo]

foo = "bar"

bar = "foo"


[bar]
foo = "bar"
"#;

    assert_format!(expected, &formatted);
}

#[test]
fn test_comment_in_array() {
    let expected = r#"
[features]
myfeature = [
  "feature1",
  # needed because blah blah blah reason that only makes sense when attached to feature2
  "feature2",
] # comment2
nextfeature = []
"#;
    let formatted = crate::formatter::format(
        expected,
        formatter::Options {
            align_entries: false,
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_comments_in_array() {
    let expected = r#"
[main]
my_array = [
  #Items
  "a",
  "b", # Some comment
  "c", # This is special

  # Other items
  "d",
  "e",
  "f",

  # Some other items we decided not to include
  # "g",
  # "h",
  # "i",

  "item",
]
"#;

    let formatted = crate::formatter::format(
        expected,
        formatter::Options {
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_align_comments() {
    let src = r#"
entry1 = "string"  # trailing comment
entry2 = "longer_string"  # trailing comment

my_array = [
  #Items
  "abc",  # comment
  "b", # Some comment
  "caa",    # This is special
 # comment
  # Other stuff
]
"#;

    let expected = r#"
entry1 = "string"        # trailing comment
entry2 = "longer_string" # trailing comment

my_array = [
  #Items
  "abc", # comment
  "b",   # Some comment
  "caa", # This is special
  # comment
  # Other stuff
]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            align_comments: true,
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_more_comment_alignments() {
    let src = r#"
entry1asdasd = "string"     # trailing comment
entry2asd = "longer_string" # trailing comment
a = "longer_string_hm"      # trailing comment
"#;

    let expected = r#"
entry1asdasd = "string"     # trailing comment
entry2asd = "longer_string" # trailing comment
a = "longer_string_hm"      # trailing comment
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            align_comments: true,
            align_entries: false,
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_align_entries_no_comments() {
    let src = r#"
entry1asdasd =  "string"     # trailing comment
entry2asd   = "longer_string"        # trailing comment
a         = "longer_string_hm" # trailing comment
"#;

    let expected = r#"
entry1asdasd = "string" # trailing comment
entry2asd    = "longer_string" # trailing comment
a            = "longer_string_hm" # trailing comment
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            align_comments: false,
            align_entries: true,
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}
