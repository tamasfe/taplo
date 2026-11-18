# to_toml.to_toml_fmt

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\to_toml.rs` (lines 17-24)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn to_toml_fmt(
        &self,
        f: &mut impl Write,
        inline: bool,
        prefer_single_quote: bool,
    ) -> core::fmt::Result {
        self.to_toml_impl(f, Keys::empty(), inline, false, prefer_single_quote)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn to_toml_fmt(
        &self,
        f: &mut impl Write,
        inline: bool,
        prefer_single_quote: bool,
    ) -> core::fmt::Result {
        self.to_toml_impl(f, Keys::empty(), inline, false, prefer_single_quote)
    }
```

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
