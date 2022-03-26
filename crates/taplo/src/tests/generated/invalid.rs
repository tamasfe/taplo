#[test]
fn array_of_tables_1() {
    let src = "# INVALID TOML DOC\nfruit = []\n\n[[fruit]] # Not allowed\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn array_of_tables_2() {
    let src = "# INVALID TOML DOC\n[[fruit]]\nname = \"apple\"\n\n[[fruit.variety]]\nname = \"red delicious\"\n\n# This table conflicts with the previous table\n[fruit.variety]\nname = \"granny smith\"\n" ;
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn bare_key_1() {
    let src = "bare!key = 123\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn bare_key_2() {
    let src = "barekey\n   = 123\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn bare_key_3() {
    let src = "barekey =";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn comment_control_1() {
    let src = "a = \"null\" # \u{0}\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn comment_control_2() {
    let src = "a = \"ctrl-P\" # \u{10}\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn comment_control_3() {
    let src = "a = \"ctrl-_\" # \u{1f}\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn comment_control_4() {
    let src = "a = \"0x7f\" # \u{7f}\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn inline_table_imutable_1() {
    let src = "[product]\ntype = { name = \"Nail\" } \ntype.edible = false # INVALID\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn inline_table_imutable_2() {
    let src = "[product]\ntype.name = \"Nail\"\ntype = { edible = false }# INVALID\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn inline_table_trailing_comma() {
    let src = "abc = { abc = 123, }\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn int_0_padded() {
    let src = "int = 0123\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn int_signed_bin() {
    let src = "bin = +0b10\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn int_signed_hex() {
    let src = "hex = +0xab\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn int_signed_oct() {
    let src = "oct = +0o23\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn key_value_pair_1() {
    let src = "key = # INVALID\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn key_value_pair_2() {
    let src = "first = \"Tom\" last = \"Preston-Werner\" # INVALID\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn multiple_dot_key() {
    let src = "# THE FOLLOWING IS INVALID\n\n# This defines the value of fruit.apple to be an integer.\nfruit.apple = 1\n\n# But then this treats fruit.apple like it's a table.\n# You can't turn an integer into a table.\nfruit.apple.smooth = true\n" ;
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn multiple_key() {
    let src = "# DO NOT DO THIS\nname = \"Tom\"\nname = \"Pradyun\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn no_key_name() {
    let src = "= \"no key name\"  # INVALID\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_control_1() {
    let src = "a = \"null\u{0}\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_control_2() {
    let src = "a = \"ctrl-P\u{10}\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_control_3() {
    let src = "a = \"ctrl-_\u{1f}\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_control_4() {
    let src = "a = \"0x7f\u{7f}\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_multiline_control_1() {
    let src = "a = \"\"\"null\u{0}\"\"\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_multiline_control_2() {
    let src = "a = \"\"\"null\u{10}\"\"\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_multiline_control_3() {
    let src = "a = \"\"\"null\u{1f}\"\"\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_multiline_control_4() {
    let src = "a = \"\"\"null\u{7f}\"\"\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_multiline_invalid_backslash() {
    let src = "a = \"\"\"\n  foo \\ \\n\n  bar\"\"\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_multiline_out_of_range_unicode_escape_1() {
    let src = "a = \"\"\"\\UFFFFFFFF\"\"\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_multiline_out_of_range_unicode_escape_2() {
    let src = "a = \"\"\"\\U00D80000\"\"\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_multiline_quotes() {
    let src = "str5 = \"\"\"Here are three quotation marks: \"\"\".\"\"\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_multiline_unknown_escape() {
    let src = "a = \"\"\"\\@\"\"\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_out_of_range_unicode_escape_1() {
    let src = "a = \"\\UFFFFFFFF\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_out_of_range_unicode_escape_2() {
    let src = "a = \"\\U00D80000\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_basic_unknown_escape() {
    let src = "a = \"\\@\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_literal_control_1() {
    let src = "a = 'null\u{0}'\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_literal_control_2() {
    let src = "a = 'null\u{10}'\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_literal_control_3() {
    let src = "a = 'null\u{1f}'\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_literal_control_4() {
    let src = "a = 'null\u{7f}'\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_literal_multiline_control_1() {
    let src = "a = '''null\u{0}'''\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_literal_multiline_control_2() {
    let src = "a = '''null\u{10}'''\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_literal_multiline_control_3() {
    let src = "a = '''null\u{1f}'''\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_literal_multiline_control_4() {
    let src = "a = '''null\u{7f}'''\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn string_literal_multiline_quotes() {
    let src = "apos15 = '''Here are fifteen apostrophes: ''''''''''''''''''  # INVALID\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn table_1() {
    let src = "# DO NOT DO THIS\n\n[fruit]\napple = \"red\"\n\n[fruit]\norange = \"orange\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn table_2() {
    let src = "# DO NOT DO THIS EITHER\n\n[fruit]\napple = \"red\"\n\n[fruit.apple]\ntexture = \"smooth\"\n" ;
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn table_3() {
    let src =
        "[fruit]\napple.color = \"red\"\napple.taste.sweet = true\n\n[fruit.apple] # INVALID\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn table_4() {
    let src = "[fruit]\napple.color = \"red\"\napple.taste.sweet = true\n\n[fruit.apple.taste] # INVALID\n" ;
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn table_invalid_1() {
    let src = "[fruit.physical] # subtable, but to which parent element should it belong?\ncolor = \"red\"\nshape = \"round\"\n\n[[fruit]] # parser must throw an error upon discovering that \"fruit\" is\n# an array rather than a table\nname = \"apple\"\n" ;
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn table_invalid_2() {
    let src = "# INVALID TOML DOC\nfruit = []\n\n[[fruit]] # Not allowed\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn table_invalid_3() {
    let src = "# INVALID TOML DOC\n[[fruit]]\nname = \"apple\"\n\n[[fruit.variety]]\nname = \"red delicious\"\n\n# INVALID: This table conflicts with the previous array of tables\n[fruit.variety]\nname = \"granny smith\"\n\n[fruit.physical]\ncolor = \"red\"\nshape = \"round\"\n" ;
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn taplo_duplicate_keys() {
    let src = "# THIS WILL NOT WORK\nspelling = \"favorite\"\n\"spelling\" = \"favourite\"\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn taplo_incomplete_inline_table() {
    let src = "schema = { enabled = false";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn taplo_inner_key_conflict() {
    let src = "package.something.else = 2\n\n[package]\nsomething.other = 2\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn taplo_invalid_array_comma_start() {
    let src = "arr = [,\"value\"]";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn taplo_invalid_array() {
    let src = "arr = [\"value\",,,,]";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn taplo_invalid_float() {
    let src = "what = 1.";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn taplo_invalid_inline_table() {
    let src = "cooldowns = { \n    aggressive = true, \n}";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn taplo_invalid_padding() {
    let src = "[int]\npadded_middle = 1__2\npadded_start = _1_2\npadded_end = 1_2_\n\npadded_plus = +_2\npadded_minus = -_2\n\n[int.bin]\npadded_middle = 0b1__0\npadded_start = 0b_1_0\npadded_end = 0b1_0_\n\n[int.hex]\npadded_middle = 0x1__0\npadded_start = 0x_1_0\npadded_end = 0x1_0_\n\n[int.oct]\npadded_middle = 0o1__0\npadded_start = 0o_1_0\npadded_end = 0o1_0_\n\n[float]\npadded_middle = 1__2.0\npadded_start = _1_2.0\npadded_end = 1_2_.0\n\npadded_plus = +_2.0\npadded_minus = -_2.0" ;
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
#[test]
fn taplo_table_before_array() {
    let src = "[foo.bar]\n[[foo]]\n";
    let p = crate::parser::parse(src);
    assert!(!p.errors.is_empty() || p.into_dom().validate().is_err());
}
