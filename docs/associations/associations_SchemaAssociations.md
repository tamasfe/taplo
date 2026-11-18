# associations.SchemaAssociations

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 44-50)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct SchemaAssociations<E: Environment> {
    concurrent_requests: Arc<Semaphore>,
    http: reqwest::Client,
    env: E,
    associations: Arc<RwLock<Vec<(AssociationRule, SchemaAssociation)>>>,
    cache: Cache<E>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct SchemaAssociations<E: Environment> {
    concurrent_requests: Arc<Semaphore>,
    http: reqwest::Client,
    env: E,
    associations: Arc<RwLock<Vec<(AssociationRule, SchemaAssociation)>>>,
    cache: Cache<E>,
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
