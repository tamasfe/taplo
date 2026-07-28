use taplo::{
    dom::{KeyOrIndex, Node},
    parser,
    rowan::TextSize,
    syntax::SyntaxKind,
};
use taplo_lsp::{
    query::{PositionInfo, Query},
};

const TOML_DOC: &str = r#"[training]
snr_range_extreme = [-20.0, -5.0]
"#;

fn parse_doc() -> Node {
    let parsed = parser::parse(TOML_DOC);
    assert!(parsed.errors.is_empty(), "parse errors: {:?}", parsed.errors);
    parsed.into_dom()
}

fn selected_position_info(query: &Query) -> PositionInfo {
    query
        .before
        .clone()
        .filter(|p| {
            p.syntax.kind() == SyntaxKind::IDENT
                || matches!(
                    p.syntax.kind(),
                    SyntaxKind::BOOL
                        | SyntaxKind::STRING
                        | SyntaxKind::INTEGER
                        | SyntaxKind::FLOAT
                )
        })
        .or_else(|| {
            query.after.clone().filter(|p| {
                p.syntax.kind() == SyntaxKind::IDENT
                    || matches!(
                        p.syntax.kind(),
                        SyntaxKind::BOOL
                            | SyntaxKind::STRING
                            | SyntaxKind::INTEGER
                            | SyntaxKind::FLOAT
                    )
            })
        })
        .expect("expected hover position info")
}

fn key_path(keys: &taplo::dom::Keys) -> Vec<String> {
    keys.iter()
        .map(|segment| match segment {
            KeyOrIndex::Key(k) => k.value().to_owned(),
            KeyOrIndex::Index(idx) => idx.to_string(),
        })
        .collect()
}

#[test]
fn tuple_array_values_resolve_to_distinct_indices() {
    let root = parse_doc();

    let first_offset = TOML_DOC.find("-20.0").expect("missing first float") + 1;
    let second_offset = TOML_DOC.find("-5.0").expect("missing second float") + 1;

    let first_query = Query::at(&root, TextSize::from(first_offset as u32));
    let second_query = Query::at(&root, TextSize::from(second_offset as u32));

    let first_info = selected_position_info(&first_query);
    let second_info = selected_position_info(&second_query);

    let first_keys = first_info
        .dom_node
        .as_ref()
        .map(|(keys, _)| key_path(keys))
        .expect("first value should have dom node");
    let second_keys = second_info
        .dom_node
        .as_ref()
        .map(|(keys, _)| key_path(keys))
        .expect("second value should have dom node");

    assert_eq!(
        first_keys,
        vec!["training", "snr_range_extreme", "0"],
        "first tuple element should resolve to index 0"
    );
    assert_eq!(
        second_keys,
        vec!["training", "snr_range_extreme", "1"],
        "second tuple element should resolve to index 1"
    );
}
