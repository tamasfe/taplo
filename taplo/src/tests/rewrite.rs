use crate::dom::{nodes::*, rewrite::prelude::*};
use std::fs;

#[test]
fn rewrite_nothing() {
    let src = fs::read_to_string("../test-data/rewrite/nothing.toml").unwrap();
    let expected = fs::read_to_string("../test-data/rewrite/nothing_expected.toml").unwrap();
    let dom = crate::parser::parse(&src).into_dom();
    let rewritten = dom.rewrite(|_, node| node.into());
    assert!(expected == rewritten)
}

#[test]
fn rewrite_key() {
    let src = fs::read_to_string("../test-data/rewrite/key.toml").unwrap();
    let expected = fs::read_to_string("../test-data/rewrite/key_expected.toml").unwrap();
    let dom = crate::parser::parse(&src).into_dom();

    let rewritten = dom.rewrite(|_, node| match node {
        crate::dom::Node::Key(k) => {
            if k.full_key_string_stripped() == "rewrite_me" {
                KeyNode::rewrite()
                    .with_dotted_keys("rewritten")
                    .build()
                    .into()
            } else {
                k.into()
            }
        }
        _ => node.into(),
    });

    assert!(expected == rewritten)
}

#[test]
fn rewrite_value() {
    let src = fs::read_to_string("../test-data/rewrite/value.toml").unwrap();
    let expected = fs::read_to_string("../test-data/rewrite/value_expected.toml").unwrap();
    let dom = crate::parser::parse(&src).into_dom();

    let rewritten = dom.rewrite(|_, node| match node {
        crate::dom::Node::Value(v) => match v {
            ValueNode::Integer(i) => {
                if i.as_i64() == 2 {
                    TableNode::rewrite()
                        .inline()
                        .with_entry(
                            EntryNode::rewrite()
                                .with_key("original_value")
                                .with_value(ValueNode::Integer(i))
                                .build(),
                        )
                        .with_entry(
                            EntryNode::rewrite()
                                .with_key("additional_value")
                                .with_value(
                                    ValueNode::rewrite()
                                        .with_value(IntegerNode::rewrite().with_value(3).build())
                                        .build(),
                                )
                                .build(),
                        )
                        .build()
                        .into()
                } else {
                    Node::from(ValueNode::Integer(i)).into()
                }
            }
            v => Node::from(v).into(),
        },
        _ => node.into(),
    });

    assert!(expected == rewritten)
}

#[test]
fn rewrite_table() {
    let src = fs::read_to_string("../test-data/rewrite/table.toml").unwrap();
    let expected = fs::read_to_string("../test-data/rewrite/table_expected.toml").unwrap();
    let dom = crate::parser::parse(&src).into_dom();

    let rewritten = dom.rewrite(|_, node| match node {
        Node::Entry(entry) => {
            if entry.key().full_key_string_stripped() == "value" {
                match entry.into_value() {
                    ValueNode::Integer(i) => {
                        if i.as_i64() == 2 {
                            TableNode::rewrite()
                                .top_level("table", false)
                                .with_entry(
                                    EntryNode::rewrite()
                                        .with_key("original_value")
                                        .with_value(ValueNode::Integer(i))
                                        .build(),
                                )
                                .with_entry(
                                    EntryNode::rewrite()
                                        .with_key("additional_value")
                                        .with_value(
                                            ValueNode::rewrite()
                                                .with_value(IntegerNode::rewrite().with_value(3).build())
                                                .build(),
                                        )
                                        .build(),
                                )
                                .build()
                                .into()
                        } else {
                            Node::from(ValueNode::Integer(i)).into()
                        }
                    }
                    v => Node::from(v).into(),
                }
            } else {
                Node::from(entry).into()
            }
        }
        _ => node.into(),
    });

    assert!(expected == rewritten)
}
