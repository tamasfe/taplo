# node.header_comments

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 314-326)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn header_comments(&self) -> impl Iterator<Item = Comment> {
        let first_item = self
            .syntax()
            .and_then(|syntax| syntax.ancestors().last())
            .and_then(|root| root.descendants().nth(1));

        match first_item {
            Some(it) => Either::Left(self.comments().take_while(move |c| {
                c.syntax.as_ref().unwrap().text_range().end() <= it.text_range().start()
            })),
            None => Either::Right(self.comments()),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn header_comments(&self) -> impl Iterator<Item = Comment> {
        let first_item = self
            .syntax()
            .and_then(|syntax| syntax.ancestors().last())
            .and_then(|root| root.descendants().nth(1));

        match first_item {
            Some(it) => Either::Left(self.comments().take_while(move |c| {
                c.syntax.as_ref().unwrap().text_range().end() <= it.text_range().start()
            })),
            None => Either::Right(self.comments()),
        }
    }
```

## Git History

- Commits: 8
- Authors: 6
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
