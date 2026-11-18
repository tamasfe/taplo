# nodes.KeyInner

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 138-157)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) struct KeyInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) is_valid: bool,
    pub(crate) value: OnceCell<String>,

    /// The same key can appear at multiple positions
    /// in a TOML document.
    ///
    /// # Example
    ///  
    /// In the following both `table` and `inner` appear multiple times
    /// despite being the same key in the DOM.
    ///
    /// ```toml
    /// [table.inner.something]
    /// [table.inner.something_else]
    /// ```
    pub(crate) additional_syntaxes: Shared<Vec<SyntaxElement>>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) struct KeyInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) is_valid: bool,
    pub(crate) value: OnceCell<String>,

    /// The same key can appear at multiple positions
    /// in a TOML document.
    ///
    /// # Example
    ///  
    /// In the following both `table` and `inner` appear multiple times
    /// despite being the same key in the DOM.
    ///
    /// ```toml
    /// [table.inner.something]
    /// [table.inner.something_else]
    /// ```
    pub(crate) additional_syntaxes: Shared<Vec<SyntaxElement>>,
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
