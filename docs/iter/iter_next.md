# iter.next

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\iter.rs` (lines 24-32)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn next(&mut self) -> Option<Self::Item> {
        if self.len != 0 {
            let v = self.iter.next().expect("exact iterator ended too early");
            self.len -= 1;
            Some(v)
        } else {
            None
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn next(&mut self) -> Option<Self::Item> {
        if self.len != 0 {
            let v = self.iter.next().expect("exact iterator ended too early");
            self.len -= 1;
            Some(v)
        } else {
            None
        }
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
