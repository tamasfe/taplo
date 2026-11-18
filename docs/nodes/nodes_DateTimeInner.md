# nodes.DateTimeInner

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 628-632)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) struct DateTimeInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) value: OnceCell<DateTimeValue>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) struct DateTimeInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) value: OnceCell<DateTimeValue>,
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
