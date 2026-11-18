# util.Position

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 8-13)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Position {
    /// Line position in a document (could be zero-based or one-based based on the usage).
    pub line: u64,
    /// Character offset on a line in a document (could be zero-based or one-based based on the usage).
    pub character: u64,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Position {
    /// Line position in a document (could be zero-based or one-based based on the usage).
    pub line: u64,
    /// Character offset on a line in a document (could be zero-based or one-based based on the usage).
    pub character: u64,
}
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
