# mod.update_options

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 219-225)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn update_options(&self, opts: &mut Options, range: TextRange) {
        for (r, s) in &self.scopes.0 {
            if r.contains_range(range) {
                opts.update(s.clone());
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn update_options(&self, opts: &mut Options, range: TextRange) {
        for (r, s) in &self.scopes.0 {
            if r.contains_range(range) {
                opts.update(s.clone());
            }
        }
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
