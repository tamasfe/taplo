# mod.FormattedEntry

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 365-373)
- **Role:** internal
- **Stability:** unknown

## Signature

```
struct FormattedEntry {
    syntax: SyntaxElement,
    key: String,
    /// This field is used to cache the "cleaned" version of the key and should only
    /// be accessed through the `cleaned_key` helpers method.
    cleaned_key: OnceCell<Vec<String>>,
    value: String,
    comment: Option<String>,
}
```

## Docstring

_No docstring provided._

## Source

```
struct FormattedEntry {
    syntax: SyntaxElement,
    key: String,
    /// This field is used to cache the "cleaned" version of the key and should only
    /// be accessed through the `cleaned_key` helpers method.
    cleaned_key: OnceCell<Vec<String>>,
    value: String,
    comment: Option<String>,
}
```

## Git History

- Commits: 17
- Authors: 14
- Bug fixes: 5
- Features: 4

## Callers

None

## Callees

None
