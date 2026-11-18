# mod.add_error

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 173-181)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn add_error(&mut self, e: &Error) {
        if let Some(last_err) = self.errors.last_mut() {
            if last_err == e {
                return;
            }
        }

        self.errors.push(e.clone());
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn add_error(&mut self, e: &Error) {
        if let Some(last_err) = self.errors.last_mut() {
            if last_err == e {
                return;
            }
        }

        self.errors.push(e.clone());
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
