# mod.into_dom

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 917-919)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn into_dom(self) -> dom::node::Node {
        dom::Node::from_syntax(self.into_syntax().into())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn into_dom(self) -> dom::node::Node {
        dom::Node::from_syntax(self.into_syntax().into())
    }
```

## Git History

- Commits: 8
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
