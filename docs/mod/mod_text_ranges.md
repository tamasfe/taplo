# mod.text_ranges

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 720-738)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn text_ranges(&self) -> Box<dyn Iterator<Item = TextRange> + '_> {
        match self.error.kind {
            ValidationErrorKind::AdditionalProperties { .. } => {
                let include_children = false;

                if self.keys.is_empty() {
                    return Box::new(self.node.text_ranges(include_children));
                }

                Box::new(
                    self.keys
                        .clone()
                        .into_iter()
                        .flat_map(move |key| self.node.get(key).text_ranges(include_children)),
                )
            }
            _ => Box::new(self.node.text_ranges(true)),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn text_ranges(&self) -> Box<dyn Iterator<Item = TextRange> + '_> {
        match self.error.kind {
            ValidationErrorKind::AdditionalProperties { .. } => {
                let include_children = false;

                if self.keys.is_empty() {
                    return Box::new(self.node.text_ranges(include_children));
                }

                Box::new(
                    self.keys
                        .clone()
                        .into_iter()
                        .flat_map(move |key| self.node.get(key).text_ranges(include_children)),
                )
            }
            _ => Box::new(self.node.text_ranges(true)),
        }
    }
```

## Git History

- Commits: 13
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
