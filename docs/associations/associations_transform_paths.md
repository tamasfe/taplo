# associations.transform_paths

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 444-454)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn transform_paths(&mut self) {
        if let SchemaCatalog::SchemaStore(index) = self {
            for s in &mut index.schemas {
                for fm in &mut s.file_match {
                    if !fm.starts_with("**/") {
                        *fm = String::from("**/") + fm.as_str();
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
    fn transform_paths(&mut self) {
        if let SchemaCatalog::SchemaStore(index) = self {
            for s in &mut index.schemas {
                for fm in &mut s.file_match {
                    if !fm.starts_with("**/") {
                        *fm = String::from("**/") + fm.as_str();
                    }
                }
            }
        }
    }
```

## Git History

- Commits: 16
- Authors: 5
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
