# document_symbols.symbols_for_value

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\document_symbols.rs` (lines 54-161)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn symbols_for_value(
    name: String,
    key_range: Option<TextRange>,
    node: &Node,
    mapper: &Mapper,
    symbols: &mut Vec<DocumentSymbol>,
) {
    let own_range = mapper.range(join_ranges(node.text_ranges(true))).unwrap();

    let range = if let Some(key_r) = key_range {
        mapper
            .range(key_r.cover(join_ranges(node.text_ranges(true))))
            .unwrap()
    } else {
        own_range
    };

    let selection_range = key_range.map_or(own_range, |r| mapper.range(r).unwrap());

    match node {
        Node::Bool(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::BOOLEAN,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Str(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::STRING,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Integer(_) | Node::Float(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::NUMBER,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Date(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::FIELD,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Array(arr) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::ARRAY,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: {
                let mut child_symbols = Vec::with_capacity(arr.items().read().len());
                let items = arr.items().read();

                for (i, c) in items.iter().enumerate() {
                    symbols_for_value(i.to_string(), None, c, mapper, &mut child_symbols);
                }

                Some(child_symbols)
            },
        }),
        Node::Table(t) => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::OBJECT,
                range: range.into_lsp(),
                selection_range: selection_range.into_lsp(),
                detail: None,
                deprecated: None,
                tags: Default::default(),
                children: {
                    let mut child_symbols = Vec::with_capacity(t.entries().read().len());
                    let entries = t.entries().read();
                    for (key, entry) in entries.iter() {
                        symbols_for_value(
                            ensure_non_empty_key(key.value().to_string()),
                            None,
                            entry,
                            mapper,
                            &mut child_symbols,
                        );
                    }

                    Some(child_symbols)
                },
            });
        }
        Node::Invalid(_) => {}
    }
}
```

## Docstring

_No docstring provided._

## Source

```
fn symbols_for_value(
    name: String,
    key_range: Option<TextRange>,
    node: &Node,
    mapper: &Mapper,
    symbols: &mut Vec<DocumentSymbol>,
) {
    let own_range = mapper.range(join_ranges(node.text_ranges(true))).unwrap();

    let range = if let Some(key_r) = key_range {
        mapper
            .range(key_r.cover(join_ranges(node.text_ranges(true))))
            .unwrap()
    } else {
        own_range
    };

    let selection_range = key_range.map_or(own_range, |r| mapper.range(r).unwrap());

    match node {
        Node::Bool(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::BOOLEAN,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Str(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::STRING,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Integer(_) | Node::Float(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::NUMBER,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Date(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::FIELD,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Array(arr) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::ARRAY,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: {
                let mut child_symbols = Vec::with_capacity(arr.items().read().len());
                let items = arr.items().read();

                for (i, c) in items.iter().enumerate() {
                    symbols_for_value(i.to_string(), None, c, mapper, &mut child_symbols);
                }

                Some(child_symbols)
            },
        }),
        Node::Table(t) => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::OBJECT,
                range: range.into_lsp(),
                selection_range: selection_range.into_lsp(),
                detail: None,
                deprecated: None,
                tags: Default::default(),
                children: {
                    let mut child_symbols = Vec::with_capacity(t.entries().read().len());
                    let entries = t.entries().read();
                    for (key, entry) in entries.iter() {
                        symbols_for_value(
                            ensure_non_empty_key(key.value().to_string()),
                            None,
                            entry,
                            mapper,
                            &mut child_symbols,
                        );
                    }
... (truncated)
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
