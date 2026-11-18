# nodes.StrInner

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 391-396)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) struct StrInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) repr: StrRepr,
    pub(crate) value: OnceCell<String>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) struct StrInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) repr: StrRepr,
    pub(crate) value: OnceCell<String>,
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
