# lib.RequestWriter

- **Type:** trait
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 131-149)
- **Role:** protocol
- **Stability:** unknown

## Signature

```
pub trait RequestWriter {
    async fn write_request<
        R: Request<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<R::Params>,
    ) -> Result<rpc::Response<R::Result>, io::Error>;

    async fn write_notification<
        N: Notification<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<N::Params>,
    ) -> Result<(), io::Error>;

    async fn cancel(&mut self) -> Result<(), io::Error>;
}
```

## Docstring

_No docstring provided._

## Source

```
pub trait RequestWriter {
    async fn write_request<
        R: Request<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<R::Params>,
    ) -> Result<rpc::Response<R::Result>, io::Error>;

    async fn write_notification<
        N: Notification<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<N::Params>,
    ) -> Result<(), io::Error>;

    async fn cancel(&mut self) -> Result<(), io::Error>;
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
