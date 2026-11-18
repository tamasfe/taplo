# util.LspExt

- **Type:** trait
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 202-205)
- **Role:** protocol
- **Stability:** unknown

## Signature

```
pub trait LspExt<T>: private::Sealed {
    fn into_lsp(self) -> T;
    fn from_lsp(val: T) -> Self;
}
```

## Docstring

_No docstring provided._

## Source

```
pub trait LspExt<T>: private::Sealed {
    fn into_lsp(self) -> T;
    fn from_lsp(val: T) -> Self;
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
