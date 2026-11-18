# lib.ResponseWriter

- **Type:** trait
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 123-128)
- **Role:** protocol
- **Stability:** unknown

## Signature

```
pub trait ResponseWriter: Sized {
    async fn write_response<R: Serialize>(
        mut self,
        response: &rpc::Response<R>,
    ) -> Result<(), io::Error>;
}
```

## Docstring

_No docstring provided._

## Source

```
pub trait ResponseWriter: Sized {
    async fn write_response<R: Serialize>(
        mut self,
        response: &rpc::Response<R>,
    ) -> Result<(), io::Error>;
}
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
