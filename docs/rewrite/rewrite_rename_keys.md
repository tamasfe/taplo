# rewrite.rename_keys

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\rewrite.rs` (lines 144-169)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn rename_keys() {
        let toml = r#"
[table.middle.inner]
[table.middle.inner.inner]
"#;

        let expected_toml = r#"
[table_new.middle_new.inner_new]
[table_new.middle_new.inner_new.inner2_new]
"#;

        let root = parse(toml).into_dom();

        let mut patches = Rewrite::new(root).unwrap();

        patches.rename_keys("table", "table_new").unwrap();
        patches.rename_keys("table.middle", "middle_new").unwrap();
        patches
            .rename_keys("table.middle.inner", "inner_new")
            .unwrap();
        patches
            .rename_keys("table.middle.inner.inner", "inner2_new")
            .unwrap();

        assert_eq!(expected_toml, patches.to_string());
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn rename_keys() {
        let toml = r#"
[table.middle.inner]
[table.middle.inner.inner]
"#;

        let expected_toml = r#"
[table_new.middle_new.inner_new]
[table_new.middle_new.inner_new.inner2_new]
"#;

        let root = parse(toml).into_dom();

        let mut patches = Rewrite::new(root).unwrap();

        patches.rename_keys("table", "table_new").unwrap();
        patches.rename_keys("table.middle", "middle_new").unwrap();
        patches
            .rename_keys("table.middle.inner", "inner_new")
            .unwrap();
        patches
            .rename_keys("table.middle.inner.inner", "inner2_new")
            .unwrap();

        assert_eq!(expected_toml, patches.to_string());
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
