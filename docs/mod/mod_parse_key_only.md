# mod.parse_key_only

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 82-90)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn parse_key_only(mut self) -> Parse {
        self.key_pattern_syntax = true;
        let _ = with_node!(self.builder, KEY, self.parse_key());

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) fn parse_key_only(mut self) -> Parse {
        self.key_pattern_syntax = true;
        let _ = with_node!(self.builder, KEY, self.parse_key());

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }
```

## Git History

- Commits: 8
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
