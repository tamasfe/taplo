# to_toml.to_toml

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\to_toml.rs` (lines 10-15)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn to_toml(&self, inline: bool, prefer_single_quote: bool) -> String {
        let mut s = String::new();
        self.to_toml_fmt(&mut s, inline, prefer_single_quote)
            .unwrap();
        s
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn to_toml(&self, inline: bool, prefer_single_quote: bool) -> String {
        let mut s = String::new();
        self.to_toml_fmt(&mut s, inline, prefer_single_quote)
            .unwrap();
        s
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
