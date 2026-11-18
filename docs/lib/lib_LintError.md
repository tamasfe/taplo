# lib.LintError

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lib.rs` (lines 20-24)
- **Role:** internal
- **Stability:** unknown

## Signature

```
struct LintError {
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Range>,
    error: String,
}
```

## Docstring

_No docstring provided._

## Source

```
struct LintError {
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Range>,
    error: String,
}
```

## Git History

- Commits: 8
- Authors: 2
- Bug fixes: 5
- Features: 2

## Callers

None

## Callees

None
