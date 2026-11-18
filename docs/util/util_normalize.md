# util.Normalize

- **Type:** trait
- **Language:** rust
- **File:** `\crates\taplo-common\src\util.rs` (lines 93-100)
- **Role:** protocol
- **Stability:** unknown

## Signature

```
pub trait Normalize {
    /// Normalizing in the context of Taplo the following:
    ///
    /// - replaces `\` with `/` on windows
    /// - decodes all percent-encoded characters
    #[must_use]
    fn normalize(self) -> Self;
}
```

## Docstring

_No docstring provided._

## Source

```
pub trait Normalize {
    /// Normalizing in the context of Taplo the following:
    ///
    /// - replaces `\` with `/` on windows
    /// - decodes all percent-encoded characters
    #[must_use]
    fn normalize(self) -> Self;
}
```

## Git History

- Commits: 11
- Authors: 4
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
