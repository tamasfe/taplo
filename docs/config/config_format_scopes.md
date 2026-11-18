# config.format_scopes

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 129-139)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn format_scopes<'s>(
        &'s self,
        path: &'s Path,
    ) -> impl Iterator<Item = (&'s String, taplo::formatter::OptionsIncomplete)> + Clone + 's {
        self.rules_for(path)
            .filter_map(|rule| match (&rule.keys, &rule.options.formatting) {
                (Some(keys), Some(opts)) => Some(keys.iter().map(move |k| (k, opts.clone()))),
                _ => None,
            })
            .flatten()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn format_scopes<'s>(
        &'s self,
        path: &'s Path,
    ) -> impl Iterator<Item = (&'s String, taplo::formatter::OptionsIncomplete)> + Clone + 's {
        self.rules_for(path)
            .filter_map(|rule| match (&rule.keys, &rule.options.formatting) {
                (Some(keys), Some(opts)) => Some(keys.iter().map(move |k| (k, opts.clone()))),
                _ => None,
            })
            .flatten()
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
