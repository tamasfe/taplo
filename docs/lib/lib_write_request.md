# lib.write_request

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 204-242)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn write_request<
        R: Request<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<R::Params>,
    ) -> Result<rpc::Response<R::Result>, io::Error> {
        let mut inner = self.inner.lock().await;
        let req_id = inner.next_request_id;
        inner.next_request_id += 1;

        let mut rw = self.rw.lock().await;

        let id = NumberOrString::Number(req_id);

        let request = rpc::Request::new()
            .with_id(id.clone().into())
            .with_method(R::METHOD)
            .with_params(params);

        let span = tracing::debug_span!("sending request", ?request);

        rw.send(request.into_message()).instrument(span).await?;

        self.last_req_id = Some(id.clone());

        let (send, recv) = oneshot::channel();
        inner.requests.insert(id, send);

        drop(inner);

        let res = recv.await.unwrap();

        tracing::trace!(response = ?res, "received response");

        self.last_req_id = None;

        Ok(res.into_params())
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn write_request<
        R: Request<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<R::Params>,
    ) -> Result<rpc::Response<R::Result>, io::Error> {
        let mut inner = self.inner.lock().await;
        let req_id = inner.next_request_id;
        inner.next_request_id += 1;

        let mut rw = self.rw.lock().await;

        let id = NumberOrString::Number(req_id);

        let request = rpc::Request::new()
            .with_id(id.clone().into())
            .with_method(R::METHOD)
            .with_params(params);

        let span = tracing::debug_span!("sending request", ?request);

        rw.send(request.into_message()).instrument(span).await?;

        self.last_req_id = Some(id.clone());

        let (send, recv) = oneshot::channel();
        inner.requests.insert(id, send);

        drop(inner);

        let res = recv.await.unwrap();

        tracing::trace!(response = ?res, "received response");

        self.last_req_id = None;

        Ok(res.into_params())
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
