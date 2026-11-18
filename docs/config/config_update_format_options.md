# config.update_format_options

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 115-127)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn update_format_options(&self, path: &Path, options: &mut formatter::Options) {
        if let Some(opts) = &self.global_options.formatting {
            options.update(opts.clone());
        }

        for rule in self.rules_for(path) {
            if rule.keys.is_none() {
                if let Some(rule_opts) = rule.options.formatting.clone() {
                    options.update(rule_opts);
                }
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn update_format_options(&self, path: &Path, options: &mut formatter::Options) {
        if let Some(opts) = &self.global_options.formatting {
            options.update(opts.clone());
        }

        for rule in self.rules_for(path) {
            if rule.keys.is_none() {
                if let Some(rule_opts) = rule.options.formatting.clone() {
                    options.update(rule_opts);
                }
            }
        }
    }
```

## Git History

- Commits: 6
- Authors: 3
- Bug fixes: 4
- Features: 1

## Callers

None

## Callees

None
