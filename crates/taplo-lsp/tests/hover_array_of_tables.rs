// Assertion-based tests for array-of-tables hover/query code paths.
//
// These verify that query resolution and lookup_keys produce the correct
// key paths and syntax kinds for keys, values, and headers inside
// [[array_of_tables]] entries — the core logic used by the hover handler.

use taplo::{
    dom::{node::Key, KeyOrIndex, Keys, Node},
    parser,
    rowan::TextSize,
    syntax::SyntaxKind,
};

use taplo_lsp::query::{lookup_keys, Query};

const TOML_DOC: &str = r#"[training]
batch_size = 4

[[training.curriculum]]
name = "phase_one"
weight = 1.0

[[training.curriculum]]
name = "phase_two"
weight = 2.0
"#;

fn parse_doc() -> Node {
    let parsed = parser::parse(TOML_DOC);
    assert!(parsed.errors.is_empty(), "parse errors: {:?}", parsed.errors);
    parsed.into_dom()
}

/// Helper: extract key path segments as strings from a `Keys`.
fn key_path(keys: &Keys) -> Vec<String> {
    keys.iter()
        .map(|segment| match segment {
            KeyOrIndex::Key(k) => k.value().to_owned(),
            KeyOrIndex::Index(idx) => idx.to_string(),
        })
        .collect()
}

/// Helper: select the hover-relevant position info from a query
/// (same logic as the hover handler).
fn selected_position_info(query: &Query) -> Option<taplo_lsp::query::PositionInfo> {
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
}

#[test]
fn flat_iter_contains_expected_array_of_tables_entries() {
    let root = parse_doc();

    let all_paths: Vec<Vec<String>> = root
        .flat_iter()
        .map(|(keys, _)| key_path(&keys))
        .collect();

    // The document should contain entries for:
    //   training.batch_size
    //   training.curriculum[0].name
    //   training.curriculum[0].weight
    //   training.curriculum[1].name
    //   training.curriculum[1].weight
    assert!(
        all_paths.iter().any(|p| p == &["training", "batch_size"]),
        "expected [training, batch_size] in flat_iter, got: {all_paths:?}"
    );
    assert!(
        all_paths
            .iter()
            .any(|p| p == &["training", "curriculum", "0", "name"]),
        "expected [training, curriculum, 0, name] in flat_iter, got: {all_paths:?}"
    );
    assert!(
        all_paths
            .iter()
            .any(|p| p == &["training", "curriculum", "1", "name"]),
        "expected [training, curriculum, 1, name] in flat_iter, got: {all_paths:?}"
    );
    assert!(
        all_paths
            .iter()
            .any(|p| p == &["training", "curriculum", "0", "weight"]),
        "expected [training, curriculum, 0, weight] in flat_iter, got: {all_paths:?}"
    );
}

#[test]
fn query_at_key_name_resolves_to_ident_with_dom_node() {
    let root = parse_doc();

    let name_offset = TOML_DOC
        .find("name")
        .expect("could not find 'name' in TOML doc");

    let query = Query::at(&root, TextSize::from(name_offset as u32));
    let pi = selected_position_info(&query)
        .expect("position_info should be Some for a key IDENT");

    // The syntax token should be an IDENT with text "name"
    assert_eq!(pi.syntax.kind(), SyntaxKind::IDENT);
    assert_eq!(pi.syntax.text(), "name");

    // The dom_node should exist and contain the key path through the
    // first array-of-tables entry
    let (keys, _node) = pi.dom_node.as_ref().expect("dom_node should be present");
    let path = key_path(keys);
    assert_eq!(
        path,
        vec!["training", "curriculum", "0", "name"],
        "dom_node keys for 'name' key"
    );
}

#[test]
fn query_at_key_name_header_key_is_none() {
    let root = parse_doc();

    // "name" is an entry key, not a header key
    let name_offset = TOML_DOC
        .find("name")
        .expect("could not find 'name' in TOML doc");
    let query = Query::at(&root, TextSize::from(name_offset as u32));

    assert!(
        query.header_key().is_none(),
        "entry key 'name' should not have a header_key"
    );
}

#[test]
fn query_at_string_value_resolves_to_string_kind() {
    let root = parse_doc();

    // Move past the opening quote to land on the STRING token
    let value_offset = TOML_DOC
        .find("\"phase_one\"")
        .expect("could not find value in TOML doc")
        + 1;

    let query = Query::at(&root, TextSize::from(value_offset as u32));
    let pi = selected_position_info(&query)
        .expect("position_info should be Some for a string value");

    assert_eq!(pi.syntax.kind(), SyntaxKind::STRING);

    let (keys, _node) = pi.dom_node.as_ref().expect("dom_node should be present");
    let path = key_path(keys);
    assert_eq!(
        path,
        vec!["training", "curriculum", "0", "name"],
        "dom_node keys for the 'phase_one' string value"
    );
}

#[test]
fn query_at_header_curriculum_resolves_to_ident_with_header_key() {
    let root = parse_doc();

    let curriculum_offset = TOML_DOC
        .find("curriculum")
        .expect("could not find 'curriculum' in TOML doc");

    let query = Query::at(&root, TextSize::from(curriculum_offset as u32));

    // "curriculum" inside [[training.curriculum]] should be an IDENT
    let pi = selected_position_info(&query)
        .expect("position_info should be Some for header IDENT");
    assert_eq!(pi.syntax.kind(), SyntaxKind::IDENT);
    assert_eq!(pi.syntax.text(), "curriculum");

    // It should be inside a table array header
    assert!(
        query.in_table_array_header(),
        "curriculum should be in a table array header"
    );

    // header_key should be present (this is a header, not an entry key)
    let header_key = query.header_key();
    assert!(
        header_key.is_some(),
        "header_key should be Some for a table array header"
    );
}

#[test]
fn lookup_keys_appends_index_for_array_without_existing_index() {
    let root = parse_doc();

    // Path WITHOUT index — lookup_keys should append the array index
    let keys_without_idx = Keys::new(
        [
            KeyOrIndex::Key(Key::from("training")),
            KeyOrIndex::Key(Key::from("curriculum")),
        ]
        .into_iter(),
    );
    let result = lookup_keys(root.clone(), &keys_without_idx);
    let path = key_path(&result);

    assert_eq!(
        path,
        vec!["training", "curriculum", "1"],
        "lookup_keys should append last array index when no index follows"
    );
}

#[test]
fn lookup_keys_preserves_existing_index_without_duplication() {
    let root = parse_doc();

    // Path WITH index — lookup_keys must NOT duplicate the index
    let keys_with_idx = Keys::new(
        [
            KeyOrIndex::Key(Key::from("training")),
            KeyOrIndex::Key(Key::from("curriculum")),
            KeyOrIndex::Index(0),
            KeyOrIndex::Key(Key::from("name")),
        ]
        .into_iter(),
    );
    let result = lookup_keys(root.clone(), &keys_with_idx);
    let path = key_path(&result);

    assert_eq!(
        path,
        vec!["training", "curriculum", "0", "name"],
        "lookup_keys should preserve existing index without adding another"
    );
}
