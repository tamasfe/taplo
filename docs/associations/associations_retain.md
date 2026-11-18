# associations.retain

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 69-71)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn retain(&self, f: impl Fn(&(AssociationRule, SchemaAssociation)) -> bool) {
        self.associations.write().retain(f);
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn retain(&self, f: impl Fn(&(AssociationRule, SchemaAssociation)) -> bool) {
        self.associations.write().retain(f);
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
