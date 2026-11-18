# mod.write_to

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 408-416)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn write_to(&self, formatted: &mut String, options: &Options) {
        *formatted += &self.key;
        if options.compact_entries {
            *formatted += "=";
        } else {
            *formatted += " = ";
        }
        *formatted += &self.value;
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn write_to(&self, formatted: &mut String, options: &Options) {
        *formatted += &self.key;
        if options.compact_entries {
            *formatted += "=";
        } else {
            *formatted += " = ";
        }
        *formatted += &self.value;
    }
```

## Git History

- Commits: 17
- Authors: 14
- Bug fixes: 5
- Features: 4

## Callers

None

## Callees

None
