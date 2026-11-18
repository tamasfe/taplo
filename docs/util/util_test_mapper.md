# util.test_mapper

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 247-273)
- **Role:** test
- **Stability:** unknown

## Signature

```
fn test_mapper() {
    let s1 = r#"
line-2
line-3"#;

    let mapper = Mapper::new_utf16(s1, false);

    assert!(s1.len() == mapper.mappings().0.len() - 1);

    assert!(
        mapper.position(0.into()).unwrap()
            == Position {
                line: 0,
                character: 0
            }
    );

    assert!(
        mapper
            .position(TextSize::from(s1.len() as u32 - 1_u32))
            .unwrap()
            == Position {
                line: 2,
                character: 5
            }
    );
}
```

## Docstring

_No docstring provided._

## Source

```
fn test_mapper() {
    let s1 = r#"
line-2
line-3"#;

    let mapper = Mapper::new_utf16(s1, false);

    assert!(s1.len() == mapper.mappings().0.len() - 1);

    assert!(
        mapper.position(0.into()).unwrap()
            == Position {
                line: 0,
                character: 0
            }
    );

    assert!(
        mapper
            .position(TextSize::from(s1.len() as u32 - 1_u32))
            .unwrap()
            == Position {
                line: 2,
                character: 5
            }
    );
}
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
