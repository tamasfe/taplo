# config.make_absolute

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 179-229)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn make_absolute(&mut self, e: &impl Environment, base: &Path) {
        if let Some(included) = &mut self.include {
            for pat in included {
                if !e.is_absolute(Path::new(pat)) {
                    *pat = base
                        .join(pat.as_str())
                        .normalize()
                        .to_string_lossy()
                        .into_owned();
                }
            }
        }

        if let Some(excluded) = &mut self.exclude {
            for pat in excluded {
                if !e.is_absolute(Path::new(pat)) {
                    *pat = base
                        .join(pat.as_str())
                        .normalize()
                        .to_string_lossy()
                        .into_owned();
                }
            }
        }

        for rule in &mut self.rule {
            if let Some(included) = &mut rule.include {
                for pat in included {
                    if !e.is_absolute(Path::new(pat)) {
                        *pat = base
                            .join(pat.as_str())
                            .normalize()
                            .to_string_lossy()
                            .into_owned();
                    }
                }
            }

            if let Some(excluded) = &mut rule.exclude {
                for pat in excluded {
                    if !e.is_absolute(Path::new(pat)) {
                        *pat = base
                            .join(pat.as_str())
                            .normalize()
                            .to_string_lossy()
                            .into_owned();
                    }
                }
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn make_absolute(&mut self, e: &impl Environment, base: &Path) {
        if let Some(included) = &mut self.include {
            for pat in included {
                if !e.is_absolute(Path::new(pat)) {
                    *pat = base
                        .join(pat.as_str())
                        .normalize()
                        .to_string_lossy()
                        .into_owned();
                }
            }
        }

        if let Some(excluded) = &mut self.exclude {
            for pat in excluded {
                if !e.is_absolute(Path::new(pat)) {
                    *pat = base
                        .join(pat.as_str())
                        .normalize()
                        .to_string_lossy()
                        .into_owned();
                }
            }
        }

        for rule in &mut self.rule {
            if let Some(included) = &mut rule.include {
                for pat in included {
                    if !e.is_absolute(Path::new(pat)) {
                        *pat = base
                            .join(pat.as_str())
                            .normalize()
                            .to_string_lossy()
                            .into_owned();
                    }
                }
            }

            if let Some(excluded) = &mut rule.exclude {
                for pat in excluded {
                    if !e.is_absolute(Path::new(pat)) {
                        *pat = base
                            .join(pat.as_str())
                            .normalize()
                            .to_string_lossy()
                            .into_owned();
                    }
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
