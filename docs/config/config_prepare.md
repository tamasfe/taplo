# config.prepare

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 332-342)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn prepare(&mut self, e: &impl Environment, base: &Path) -> Result<(), anyhow::Error> {
        let default_include = String::from("**");
        self.file_rule = Some(GlobRule::new(
            self.include
                .as_deref()
                .unwrap_or(&[default_include] as &[String]),
            self.exclude.as_deref().unwrap_or(&[] as &[String]),
        )?);
        self.options.prepare(e, base)?;
        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn prepare(&mut self, e: &impl Environment, base: &Path) -> Result<(), anyhow::Error> {
        let default_include = String::from("**");
        self.file_rule = Some(GlobRule::new(
            self.include
                .as_deref()
                .unwrap_or(&[default_include] as &[String]),
            self.exclude.as_deref().unwrap_or(&[] as &[String]),
        )?);
        self.options.prepare(e, base)?;
        Ok(())
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
