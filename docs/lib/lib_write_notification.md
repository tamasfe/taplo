# lib.write_notification

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 245-260)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn write_notification<
        N: Notification<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<N::Params>,
    ) -> Result<(), io::Error> {
        let mut rw = self.rw.lock().await;
        rw.send(
            rpc::Request::new()
                .with_method(N::METHOD)
                .with_params(params)
                .into_message(),
        )
        .await
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn write_notification<
        N: Notification<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<N::Params>,
    ) -> Result<(), io::Error> {
        let mut rw = self.rw.lock().await;
        rw.send(
            rpc::Request::new()
                .with_method(N::METHOD)
                .with_params(params)
                .into_message(),
        )
        .await
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
