# document_symbols.create_symbols

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\document_symbols.rs` (lines 31-51)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn create_symbols(doc: &DocumentState) -> Vec<DocumentSymbol> {
    let mapper = &doc.mapper;
    let mut symbols: Vec<DocumentSymbol> = Vec::new();

    let dom = doc.dom.clone();

    let root_table = dom.as_table().unwrap();
    let entries = root_table.entries().read();

    for (key, entry) in entries.iter() {
        symbols_for_value(
            ensure_non_empty_key(key.value().to_string()),
            None,
            entry,
            mapper,
            &mut symbols,
        );
    }

    symbols
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) fn create_symbols(doc: &DocumentState) -> Vec<DocumentSymbol> {
    let mapper = &doc.mapper;
    let mut symbols: Vec<DocumentSymbol> = Vec::new();

    let dom = doc.dom.clone();

    let root_table = dom.as_table().unwrap();
    let entries = root_table.entries().read();

    for (key, entry) in entries.iter() {
        symbols_for_value(
            ensure_non_empty_key(key.value().to_string()),
            None,
            entry,
            mapper,
            &mut symbols,
        );
    }

    symbols
}
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
