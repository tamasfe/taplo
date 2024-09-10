use difference::Changeset;

use crate::formatter;
use crate::formatter::Options;

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

#[test]
fn test_nested_arrays() {
    let src = r#"
my_array = [
    [
        "my_value",
    ]
]
"#;

    let expected = r#"
my_array = [
    [
        "my_value",
    ],
]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            align_comments: false,
            align_entries: true,
            array_auto_collapse: false,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_too_long_array() {
    let src = r#"
array_is_just_right = ["this_line_is_exactly_80_characters_long", "filler_data"]
"#;

    let expected = r#"
array_is_just_right = ["this_line_is_exactly_80_characters_long", "filler_data"]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            array_auto_collapse: false,
            array_auto_expand: true,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);

    let src = r#"
array_is_a_bit_too_long = ["this_line_is_exactly_80_characters_long", "filler_data"]
"#;

    let expected = r#"
array_is_a_bit_too_long = [
    "this_line_is_exactly_80_characters_long",
    "filler_data",
]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            array_auto_collapse: false,
            array_auto_expand: true,
            column_width: 80,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_cargo_toml() {
    let src = r#"
[package]
authors = ["tamasfe"]
categories = ["parser-implementations", "parsing"]
description = "A TOML parser, analyzer and formatter library"
edition = "2018"
homepage = "https://taplo.tamasfe.dev"
keywords = ["toml", "parser", "formatter", "linter"]
license = "MIT"
name = "taplo"
readme = "../README.md"
repository = "https://github.com/tamasfe/taplo"
version = "0.5.4"

[lib]
crate-type = ["cdylib", "lib"]

[features]
serde = ["serde_crate", "serde_json"]
schema = ["once_cell", "schemars", "serde"]
rewrite = []

[dependencies]
glob = "0.3"
indexmap = "1.6.2"
logos = "0.12.0"
regex = "1.5.4"
rowan = "0.12.6"
semver = { version = "1.0.3", features = ["serde"] }
smallvec = "1.6.1"

chrono = { version = "0.4", optional = true }
time = { version = "0.2", optional = true }

once_cell = { version = "1.8.0", optional = true }
schemars = { version = "0.8.3", optional = true }
serde_crate = { package = "serde", version = "1", features = ["derive"], optional = true }
serde_json = { version = "1", optional = true }
verify = { version = "0.3", features = ["schemars", "serde"], optional = true }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = { version = "0.2", features = ["serde-serialize"] }
toml = "0.7"

[dev-dependencies]
assert-json-diff = "2"
serde_json = "1"
toml = "0.7"
difference = "2.0.0"

[package.metadata.docs.rs]
features = ["serde", "schema", "chrono", "rewrite"]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            array_auto_collapse: false,
            array_auto_expand: true,
            column_width: 90,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn test_very_nested_arrays() {
    let src = r#"
my_array = [
    [
        [
            [
                "my_value",
            ],
        ],
    ],
    [
        [
            [
                "my_value",
            ],
        ],
    ],
    [
        [
            [
                [{ even = { more = ["nested"] } }],
            ],
        ],
    ],
]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            array_auto_collapse: false,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn array_collapse() {
    let src = r#"
my_array = [
    [
        [
            [
                "my_value",
            ],
        ],
    ],
]
"#;

    let expected = r#"
my_array = [[[["my_value"]]]]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            array_auto_collapse: true,
            compact_arrays: true,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn trailing_newline() {
    let src = r#"trailing_new_line = {}"#;

    let expected = r#"trailing_new_line = {}
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            array_auto_collapse: true,
            compact_arrays: true,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn no_trailing_newline() {
    let src = r#"no_new_line = {}
"#;

    let expected = r#"no_new_line = {}"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            array_auto_collapse: true,
            compact_arrays: true,
            trailing_newline: false,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_compact_entries() {
    let src = r#"
entry1asdasd =  "string"     # trailing comment
entry2asd   = "longer_string"        # trailing comment
a         = "longer_string_hm" # trailing comment
inline_table = { key = "value" }
"#;

    let expected = r#"
entry1asdasd="string"        # trailing comment
entry2asd="longer_string"    # trailing comment
a="longer_string_hm"         # trailing comment
inline_table={ key="value" }
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            align_comments: true,
            align_entries: false,
            compact_entries: true,
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn array_no_trailing_comma() {
    let src = r#"
my_array = [
    [
        [
            [
                "my_value",
            ]
        ]
    ]
]
"#;

    let expected = r#"
my_array = [
    [
        [
            [
                "my_value"
            ]
        ]
    ]
]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            array_auto_collapse: false,
            array_trailing_comma: false,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn array_max_new_lines() {
    let src = r#"
my_array = [
    [
        [
            [
                "my_value"











            ]
        ]
    ]
]
"#;

    let expected = r#"
my_array = [
    [
        [
            [
                "my_value"


            ]
        ]
    ]
]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            array_auto_collapse: false,
            array_trailing_comma: false,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn indent_entries() {
    let src = r#"
[table]

  entry = "stuff"

  [table.subtable]
    nested_entry = 2

    [[table.subtable.array]]
      entry_array = [
        "value",
        [
          "nested_value"
        ]
      ]

    [[table.subtable.array]]
      entry_array = [
        "value",
        [
          "nested_value"
        ]
      ]

[not_sub_table]

  another_entry = 3
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            array_auto_collapse: false,
            array_trailing_comma: false,
            indent_entries: true,
            indent_tables: true,
            indent_string: "  ".into(),
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn multiple_comments() {
    let src = r#"
# comments at the start
# comments at the start
# comments at the start

[table1] # comment after table

# comment before table
[table2] # comment after table
# comment under table

# multiple
# comment
# lines
entry = "value"

entry_2 = true # comment
# comment
# comment

# free-standing comments
# free-standing comments
# free-standing comments

# table comment
# table comment
[table3]
# comment after table
# comment after table
another_entry = 2

# free-standing comments
# free-standing comments
# free-standing comments

array = [ # comment at start
    "value",
    # multiple comments in array
    # multiple comments in array
    # multiple comments in array
    # multiple comments in array

    # multiple comments in array
    # multiple comments in array

    "value",

    # multiple comments in array
    # multiple comments in array
    "value",
] # trailing comment
# trailing comment under

# trailing comments
# trailing comments
# trailing comments
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn multiple_comments_indented() {
    let src = r#"
#General settings
[general]
    #Is Enabled?
    enabled = true
    #Cost
    #Range: > -2147483648
    cost = 10
    #Is Starter Glyph?
    starter = false
    #The maximum number of times this glyph may appear in a single spell
    #Range: > 1
    per_spell_limit = 2147483647

# table comments
# table comments
# table comments
[another_table]
    # comment under table
    # comment under table
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            indent_entries: true,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn table_entries_no_blank_space() {
    let src = r#"
[a]
hello = "world"
[b]
foo = ["bar"]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn table_entries_no_blank_space_indent_entries() {
    let src = r#"
[a]
    hello = "world"
[b]
    foo = ["bar"]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            indent_entries: true,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn table_entries_no_blank_space_indent_entries_and_tables() {
    let src = r#"
[a]
    hello = "world"
    [a.b]
        foo = ["bar"]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            indent_entries: true,
            indent_tables: true,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn single_comment_in_array() {
    let src = r#"
runtime-benchmarks = [
    # a comment
]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            indent_entries: true,
            indent_tables: true,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn table_indents() {
    let src = r#"
[[table]]
    name = "Root Table 1"
    [table.nestedtable]
        name = "Nested parent"
        [[table.nestedtable.subtable]]
            name = "Subtable 1"
        [[table.nestedtable.subtable]]
            name = "Subtable 2"

[[table]]
    name = "Root Table 2"
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            indent_entries: true,
            indent_tables: true,
            indent_string: "    ".into(),
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn no_expand_inline_table() {
    let src = r#"
very_long_inline_table = { array = ["aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa"] }
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            indent_string: "  ".into(),
            inline_table_expand: false,
            ..Default::default()
        },
    );

    assert_format!(src, &formatted);
}

#[test]
fn test_sorted_inline_tables() {
    let src = r#"
foo = { b = 2, a = 1 }

bar = [
  { a = 1, b = 2, c = 3 },
  { b = 2, a = 1, d = 4, e = 5 },
]
"#;

    let expected = r#"
foo = { a = 1, b = 2 }

bar = [{ a = 1, b = 2, c = 3 }, { a = 1, b = 2, d = 4, e = 5 }]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            reorder_inline_tables: true,
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_sorted_groupings_in_array() {
    let src = r#"
foo = [
  "b",
  "a",
  "c",

  2021-01-01,
  1979-05-27,

  ["x", "a"],
  { b = 2, a = 1 },

  3,
  1,
  2,
  10, # due to the lexicographic order
  3
]
"#;

    let expected = r#"
foo = [
  "a",
  "b",
  "c",

  1979-05-27,
  2021-01-01,

  ["a", "x"],
  { b = 2, a = 1 },

  1,
  10, # due to the lexicographic order
  2,
  3,
  3,
]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            reorder_arrays: true,
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_single_comment_no_alignment() {
    let src = r#"
entry1 = "string"  # trailing comment
entry2 = "longer_string"

my_array = [
  #Items
  "abc",
  "b", # Some comment
  "caa",
 # comment
  # Other stuff
]
"#;

    let expected = r#"
entry1 = "string" # trailing comment
entry2 = "longer_string"

my_array = [
  #Items
  "abc",
  "b", # Some comment
  "caa",
  # comment
  # Other stuff
]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            align_comments: true,
            align_single_comments: false,
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_single_comment_alignment() {
    let src = r#"
entry1 = "string"  # trailing comment
entry2 = "longer_string"

my_array = [
  #Items
  "abc",
  "b", # Some comment
  "caa",
 # comment
  # Other stuff
]
"#;

    let expected = r#"
entry1 = "string"        # trailing comment
entry2 = "longer_string"

my_array = [
  #Items
  "abc",
  "b",   # Some comment
  "caa",
  # comment
  # Other stuff
]
"#;

    let formatted = crate::formatter::format(
        src,
        formatter::Options {
            align_comments: true,
            align_single_comments: true,
            ..Default::default()
        },
    );

    assert_format!(expected, &formatted);
}

#[test]
fn test_comment_with_brackets() {
    let src = r#"
my_array = [
  # [x]
  "y",
]
"#;

    let expected = r#"
my_array = [
  # [x]
  "y",
]
"#;

    let formatted = crate::formatter::format(src, Default::default());

    assert_format!(expected, &formatted);
}

#[test]
fn test_comment_after_entry() {
    let src = r#"
a = "b" # comment
"#;

    let expected = r#"
a = "b" # comment
"#;
    let opt = Options {
        column_width: 1,
        ..Default::default()
    };
    let formatted = crate::formatter::format(src, opt);

    assert_format!(expected, &formatted);
}
