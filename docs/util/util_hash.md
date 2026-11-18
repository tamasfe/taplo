# util.hash

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\util.rs` (lines 72-90)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            Value::Null => 0.hash(state),
            Value::Bool(v) => v.hash(state),
            Value::Number(v) => v.hash(state),
            Value::String(v) => v.hash(state),
            Value::Array(v) => {
                for v in v {
                    HashValue(v).hash(state);
                }
            }
            Value::Object(v) => {
                for (k, v) in v {
                    k.hash(state);
                    HashValue(v).hash(state);
                }
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            Value::Null => 0.hash(state),
            Value::Bool(v) => v.hash(state),
            Value::Number(v) => v.hash(state),
            Value::String(v) => v.hash(state),
            Value::Array(v) => {
                for v in v {
                    HashValue(v).hash(state);
                }
            }
            Value::Object(v) => {
                for (k, v) in v {
                    k.hash(state);
                    HashValue(v).hash(state);
                }
            }
        }
    }
```

## Git History

- Commits: 11
- Authors: 4
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
