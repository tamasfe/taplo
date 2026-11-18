# mod.find_node

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\mod.rs` (lines 145-155)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn find_node(&self, offset: TextSize, inclusive: bool) -> Option<SyntaxNode> {
        for d in self.descendants().skip(1) {
            let range = d.text_range();

            if (inclusive && range.contains_inclusive(offset)) || range.contains(offset) {
                return Some(d);
            }
        }

        None
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn find_node(&self, offset: TextSize, inclusive: bool) -> Option<SyntaxNode> {
        for d in self.descendants().skip(1) {
            let range = d.text_range();

            if (inclusive && range.contains_inclusive(offset)) || range.contains(offset) {
                return Some(d);
            }
        }

        None
    }
```

## Git History

- Commits: 4
- Authors: 2
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
