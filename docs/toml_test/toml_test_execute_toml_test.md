# toml_test.execute_toml_test

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\toml_test.rs` (lines 15-40)
- **Role:** test
- **Stability:** unknown

## Signature

```
pub async fn execute_toml_test(&self) -> Result<(), anyhow::Error> {
        let mut buf = String::new();
        self.env.stdin().read_to_string(&mut buf).await?;

        let parse = taplo::parser::parse(&buf);

        if !parse.errors.is_empty() {
            for err in parse.errors {
                eprintln!("{err}");
            }

            return Err(anyhow!("invalid toml"));
        }
        let dom = parse.into_dom();

        if let Err(err) = dom.validate() {
            for err in err {
                eprintln!("{err}");
            }
            return Err(anyhow!("invalid toml"));
        }

        serde_json::to_writer(std::io::stdout(), &TomlTestValue::new(&dom))?;

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn execute_toml_test(&self) -> Result<(), anyhow::Error> {
        let mut buf = String::new();
        self.env.stdin().read_to_string(&mut buf).await?;

        let parse = taplo::parser::parse(&buf);

        if !parse.errors.is_empty() {
            for err in parse.errors {
                eprintln!("{err}");
            }

            return Err(anyhow!("invalid toml"));
        }
        let dom = parse.into_dom();

        if let Err(err) = dom.validate() {
            for err in err {
                eprintln!("{err}");
            }
            return Err(anyhow!("invalid toml"));
        }

        serde_json::to_writer(std::io::stdout(), &TomlTestValue::new(&dom))?;

        Ok(())
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
