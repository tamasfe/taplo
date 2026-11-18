# mod.value_internal

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 331-355)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn value_internal(&self) -> &CommentValue {
        self.value
            .get_or_init(|| match self.syntax.as_ref().and_then(|s| s.as_token()) {
                Some(t) => {
                    let text = t.text();

                    if let Some(directive_content) = text.strip_prefix("#:") {
                        let mut directive_content = directive_content.split_whitespace();
                        let directive_name = directive_content.next().unwrap_or("");
                        let directive_value = directive_content.next().unwrap_or("");
                        return CommentValue::Directive {
                            name: directive_name.into(),
                            value: directive_value.into(),
                        };
                    }

                    if let Some(comment_content) = text.strip_prefix('#') {
                        return CommentValue::Comment(comment_content.into());
                    }

                    Default::default()
                }
                None => Default::default(),
            })
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn value_internal(&self) -> &CommentValue {
        self.value
            .get_or_init(|| match self.syntax.as_ref().and_then(|s| s.as_token()) {
                Some(t) => {
                    let text = t.text();

                    if let Some(directive_content) = text.strip_prefix("#:") {
                        let mut directive_content = directive_content.split_whitespace();
                        let directive_name = directive_content.next().unwrap_or("");
                        let directive_value = directive_content.next().unwrap_or("");
                        return CommentValue::Directive {
                            name: directive_name.into(),
                            value: directive_value.into(),
                        };
                    }

                    if let Some(comment_content) = text.strip_prefix('#') {
                        return CommentValue::Comment(comment_content.into());
                    }

                    Default::default()
                }
                None => Default::default(),
            })
    }
```

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
