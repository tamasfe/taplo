# world.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\world.rs` (lines 87-101)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn new(env: E) -> Self {
        Self {
            init_config: Default::default(),
            workspaces: {
                let mut m = IndexMap::default();
                m.insert(
                    DEFAULT_WORKSPACE_URL.clone(),
                    WorkspaceState::new(env.clone(), DEFAULT_WORKSPACE_URL.clone()),
                );
                AsyncRwLock::new(Workspaces(m))
            },
            default_config: Default::default(),
            env,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn new(env: E) -> Self {
        Self {
            init_config: Default::default(),
            workspaces: {
                let mut m = IndexMap::default();
                m.insert(
                    DEFAULT_WORKSPACE_URL.clone(),
                    WorkspaceState::new(env.clone(), DEFAULT_WORKSPACE_URL.clone()),
                );
                AsyncRwLock::new(Workspaces(m))
            },
            default_config: Default::default(),
            env,
        }
    }
```

## Git History

- Commits: 13
- Authors: 3
- Bug fixes: 4
- Features: 5

## Callers

None

## Callees

None
