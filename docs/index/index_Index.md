# index.Index

- **Type:** trait
- **Language:** rust
- **File:** `\crates\taplo\src\dom\index.rs` (lines 4-7)
- **Role:** protocol
- **Stability:** unknown

## Signature

```
pub trait Index: Sealed + core::fmt::Display {
    #[doc(hidden)]
    fn index_into(&self, v: &Node) -> Option<Node>;
}
```

## Docstring

_No docstring provided._

## Source

```
pub trait Index: Sealed + core::fmt::Display {
    #[doc(hidden)]
    fn index_into(&self, v: &Node) -> Option<Node>;
}
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
