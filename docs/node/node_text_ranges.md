# node.text_ranges

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 247-294)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn text_ranges(&self, include_children: bool) -> impl ExactSizeIterator<Item = TextRange> {
        let mut ranges = Vec::with_capacity(1);

        match self {
            Node::Table(v) => {
                if include_children {
                    let entries = v.entries().read();

                    for (k, entry) in entries.iter() {
                        ranges.extend(k.text_ranges());
                        ranges.extend(entry.text_ranges(true));
                    }
                }

                if let Some(mut r) = v.syntax().map(|s| s.text_range()) {
                    for range in &ranges {
                        r = r.cover(*range);
                    }

                    ranges.insert(0, r);
                }
            }
            Node::Array(v) => {
                if include_children {
                    let items = v.items().read();
                    for item in items.iter() {
                        ranges.extend(item.text_ranges(true));
                    }
                }

                if let Some(mut r) = v.syntax().map(|s| s.text_range()) {
                    for range in &ranges {
                        r = r.cover(*range);
                    }

                    ranges.insert(0, r);
                }
            }
            Node::Bool(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Str(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Integer(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Float(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Date(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Invalid(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
        }

        ranges.into_iter()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn text_ranges(&self, include_children: bool) -> impl ExactSizeIterator<Item = TextRange> {
        let mut ranges = Vec::with_capacity(1);

        match self {
            Node::Table(v) => {
                if include_children {
                    let entries = v.entries().read();

                    for (k, entry) in entries.iter() {
                        ranges.extend(k.text_ranges());
                        ranges.extend(entry.text_ranges(true));
                    }
                }

                if let Some(mut r) = v.syntax().map(|s| s.text_range()) {
                    for range in &ranges {
                        r = r.cover(*range);
                    }

                    ranges.insert(0, r);
                }
            }
            Node::Array(v) => {
                if include_children {
                    let items = v.items().read();
                    for item in items.iter() {
                        ranges.extend(item.text_ranges(true));
                    }
                }

                if let Some(mut r) = v.syntax().map(|s| s.text_range()) {
                    for range in &ranges {
                        r = r.cover(*range);
                    }

                    ranges.insert(0, r);
                }
            }
            Node::Bool(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Str(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Integer(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Float(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Date(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Invalid(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
        }

        ranges.into_iter()
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
