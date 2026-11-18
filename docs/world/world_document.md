# world.document

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\world.rs` (lines 141-145)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn document(&self, url: &Url) -> Result<&DocumentState, rpc::Error> {
        self.documents
            .get(url)
            .ok_or_else(rpc::Error::invalid_params)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) fn document(&self, url: &Url) -> Result<&DocumentState, rpc::Error> {
        self.documents
            .get(url)
            .ok_or_else(rpc::Error::invalid_params)
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
