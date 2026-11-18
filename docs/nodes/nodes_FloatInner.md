# nodes.FloatInner

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 594-598)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) struct FloatInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) value: OnceCell<f64>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) struct FloatInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) value: OnceCell<f64>,
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
