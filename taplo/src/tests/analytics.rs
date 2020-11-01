use crate::{analytics::NodeRef, dom::NodeSyntax, syntax::SyntaxKind::*, util::coords::Mapper};
use lsp_types::Position;
use std::fs;

fn cargo_toml(idx: usize) -> String {
    fs::read_to_string(&format!("../test-data/analytics/_cargo{}.toml", idx)).unwrap()
}

#[test]
fn query_author() {
    let src = cargo_toml(1);
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let start = mapper.offset(Position::new(2, 12)).unwrap();
    let middle = mapper.offset(Position::new(2, 16)).unwrap();
    let end = mapper.offset(Position::new(2, 45)).unwrap();

    let start = dom.query_position(start);
    let middle = dom.query_position(middle);
    let end = dom.query_position(end);

    assert!(start.after.syntax.is_kind(STRING));
    assert!(start.after.syntax.text.unwrap() == r#""tamasf97 <tamasf97@outlook.com>""#);
    assert!(middle.after.syntax.is_kind(STRING));
    assert!(middle.after.syntax.text.unwrap() == r#""tamasf97 <tamasf97@outlook.com>""#);
    assert!(end.before.as_ref().unwrap().syntax.is_kind(STRING));
    assert!(end.before.unwrap().syntax.text.unwrap() == r#""tamasf97 <tamasf97@outlook.com>""#);
    assert!(!end.after.syntax.is_kind(STRING));
    assert!(end.after.syntax.text.as_ref().unwrap() != r#""tamasf97 <tamasf97@outlook.com>""#);
}

#[test]
fn query_package_field() {
    let src = cargo_toml(1);
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(6, 1)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());

    let first_query_node = pos.after.nodes.last().copied().unwrap();

    let is_table = match first_query_node {
        NodeRef::Table(_) => true,
        _ => false,
    };

    assert!(is_table);

    let pos = mapper.offset(Position::new(7, 1)).unwrap();
    let pos = dom.query_position(pos);
    assert!(!pos.is_completable());

    let before_node = pos.before.unwrap().nodes.last().copied().unwrap();
    let after_node = pos.after.nodes.last().copied().unwrap();

    assert!(before_node == first_query_node);
    assert!(before_node != after_node);
}

#[test]
fn query_lib_table() {
    let src = cargo_toml(1);
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(7, 5)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());

    let first_query_node = pos.after.nodes.last().copied().unwrap();

    let is_table = match first_query_node {
        NodeRef::Table(_) => true,
        _ => false,
    };

    assert!(is_table);

    let before_node = pos.before.unwrap().nodes.last().copied().unwrap();

    let is_key = match before_node {
        NodeRef::Key(_) => true,
        _ => false,
    };

    assert!(is_key);
}

#[test]
fn query_table_header() {
    let src = cargo_toml(1);
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(49, 1)).unwrap();
    let pos = dom.query_position(pos);
    assert!(!pos.is_completable());

    let pos = mapper.offset(Position::new(49, 3)).unwrap();
    let pos = dom.query_position(pos);
    assert!(!pos.is_completable());

    let pos = mapper.offset(Position::new(49, 2)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.is_inside_header());
    assert!(pos.is_empty_header());
}

#[test]
fn query_incomplete_key() {
    let src = cargo_toml(1);
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(51, 1)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());

    let key = pos.after.syntax.text.unwrap();
    assert!(key == "asd.bsd");

    let pos = mapper.offset(Position::new(51, 8)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());

    assert!(pos.before.unwrap().syntax.text.unwrap() == key);
}

#[test]
fn query_subtable() {
    let src = cargo_toml(2);
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(5, 14)).unwrap();
    let pos = dom.query_position(pos);

    assert!(pos.is_completable());

    assert!(pos.after.path.dotted() == "profile.release");

    let pos = mapper.offset(Position::new(5, 4)).unwrap();
    let pos = dom.query_position(pos);

    assert!(pos.is_completable());

    assert!(pos.after.path.dotted() == "profile");
}

#[test]
fn query_table_key() {
    let src = cargo_toml(1);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(6, 1)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.after.path.dotted() == "some.package");
}

#[test]
fn query_key_period() {
    let src = cargo_toml(1);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(53, 6)).unwrap();
    let pos = dom.query_position(pos);

    assert!(pos.before.unwrap().path.dotted() == "lib");

    let pos = mapper.offset(Position::new(54, 11)).unwrap();
    let pos = dom.query_position(pos);

    assert!(pos.before.unwrap().path.dotted() == "some.lib");

    let pos = mapper.offset(Position::new(48, 7)).unwrap();
    let pos = dom.query_position(pos);

    assert!(pos.before.unwrap().path.dotted() == "dev-dependencies.stuff");

    let pos = mapper.offset(Position::new(60, 7)).unwrap();
    let pos = dom.query_position(pos);

    assert!(pos.before.unwrap().path.dotted() == "test.1.thing");
}

#[test]
fn query_start() {
    let src = cargo_toml(3);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(1, 1)).unwrap();
    let pos = dom.query_position(pos);

    assert!(pos.after.nodes.last().unwrap().is_root());
}

#[test]
fn query_comment() {
    let src = cargo_toml(3);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(18, 11)).unwrap();
    let pos = dom.query_position(pos);
    assert!(!pos.is_completable());

    let pos = mapper.offset(Position::new(18, 1)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
}

#[test]
fn query_key() {
    let src = cargo_toml(4);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(1, 4)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "lib");

    let pos = mapper.offset(Position::new(5, 4)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "package.asd");

    let src = cargo_toml(5);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();
    let pos = mapper.offset(Position::new(1, 10)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "lib.bench");

    let src = cargo_toml(5);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();
    let pos = mapper.offset(Position::new(1, 9)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "lib.bench");

    let src = cargo_toml(6);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();
    let pos = mapper.offset(Position::new(1, 5)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "lib");

    let src = cargo_toml(7);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();
    let pos = mapper.offset(Position::new(2, 6)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "lib.bench");
}

#[test]
fn check_token_before() {
    let src = cargo_toml(7);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(4, 11)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(
        pos.before
            .unwrap()
            .syntax
            .first_token_before()
            .unwrap()
            .1
            .kind()
            == BRACE_START
    );

    let pos = mapper.offset(Position::new(4, 13)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(
        pos.before
            .unwrap()
            .syntax
            .first_token_before()
            .unwrap()
            .1
            .kind()
            == COMMA
    );
}

#[test]
fn query_value() {
    let src = cargo_toml(8);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(1, 7)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "lib");

    let pos = mapper.offset(Position::new(3, 6)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "b");
}

#[test]
fn query_value2() {
    let src = cargo_toml(9);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(2, 8)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "lib.bench");
}

#[test]
fn query_value_in_array() {
    let src = cargo_toml(10);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(2, 10)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "features.asad");

    for node in &pos.before.as_ref().unwrap().nodes {
        dbg!(node.name());
    }
}

#[test]
fn query_complete_value() {
    let src = cargo_toml(10);
    let mapper = Mapper::new(&src);
    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(6, 9)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().path.dotted() == "lib.asd");
}