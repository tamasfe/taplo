# associations.add_builtins

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 85-100)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn add_builtins(&self) {
        self.retain(|(_, assoc)| assoc.meta["source"] != source::BUILTIN);

        self.associations.write().push((
            AssociationRule::Regex(Regex::new(r".*\.?taplo\.toml$").unwrap()),
            SchemaAssociation {
                url: builtins::TAPLO_CONFIG_URL.parse().unwrap(),
                meta: json!({
                    "name": "Taplo",
                    "description": "Taplo configuration file.",
                    "source": source::BUILTIN
                }),
                priority: priority::BUILTIN,
            },
        ));
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn add_builtins(&self) {
        self.retain(|(_, assoc)| assoc.meta["source"] != source::BUILTIN);

        self.associations.write().push((
            AssociationRule::Regex(Regex::new(r".*\.?taplo\.toml$").unwrap()),
            SchemaAssociation {
                url: builtins::TAPLO_CONFIG_URL.parse().unwrap(),
                meta: json!({
                    "name": "Taplo",
                    "description": "Taplo configuration file.",
                    "source": source::BUILTIN
                }),
                priority: priority::BUILTIN,
            },
        ));
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
