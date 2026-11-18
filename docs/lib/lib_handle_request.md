# lib.handle_request

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 347-553)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn handle_request(
        inner: Arc<AsyncMutex<Inner<W>>>,
        data: W,
        request: rpc::Request<serde_json::Value>,
        mut writer: impl MessageWriter + Clone + 'static,
    ) -> Result<(), io::Error> {
        if &request.jsonrpc != "2.0" {
            tracing::error!("JSON-RPC message version is not 2.0");
            return writer
                .send(
                    rpc::Response::error(
                        rpc::Error::invalid_request()
                            .with_data("only JSON-RPC version 2.0 is accepted"),
                    )
                    .into_message(),
                )
                .await;
        }

        if request.id.is_some() {
            tracing::debug!(
                id = ?request.id.as_ref().unwrap(),
                method = %request.method,
                "request received"
            );
            let mut s = inner.lock().await;

            if s.shutting_down {
                tracing::warn!(
                    id = ?request.id.as_ref().unwrap(),
                    method = %request.method,
                    "received request while shutting down"
                );

                writer
                    .send(
                        rpc::Response::error(
                            rpc::Error::invalid_request().with_data("server is shutting down"),
                        )
                        .with_request_id(request.id.clone().unwrap())
                        .into_message(),
                    )
                    .await?;
                return Ok(());
            }

            if request.method == req::Shutdown::METHOD {
                tracing::info!(
                    id = ?request.id.as_ref().unwrap(),
                    method = %request.method,
                    "received shutdown request"
                );

                s.shutting_down = true;
            }

            let is_initialize = request.method == req::Initialize::METHOD;

            if !s.initialized && !is_initialize {
                tracing::error!(
                    id = ?request.id.as_ref().unwrap(),
                    method = %request.method,
                    "server not yet initialized"
                );

                writer
                    .send(
                        rpc::Response::error(rpc::Error::server_not_initialized())
                            .with_request_id(request.id.clone().unwrap())
                            .into_message(),
                    )
                    .await?;
                return Ok(());
            }

            if s.handlers.contains_key(&request.method) {
                let mut handler = s.handlers.get_mut(&request.method).unwrap().clone();

                let id = request.id.clone().unwrap();

                // We expect the handler to run for a longer time
                drop(s);

                let ctx = Server::create_context(
                    inner.clone(),
                    Arc::new(AsyncMutex::new(Box::new(writer.clone()))),
                    data,
                    &request,
                )
                .await;

                let handler_span = tracing::trace_span!(
                    "request handler",
                    method = %request.method,
                );

                let method = request.method.clone();

                handler
                    .handle(ctx.clone(), request, Some(&mut writer))
                    .instrument(handler_span)
                    .await;

                let deferred = mem::take(&mut (*ctx.deferred.lock().await));

                for d in deferred {
                    let deferred_span = tracing::trace_span!(
                        "deferred task",
                        %method,
                    );

                    d.instrument(deferred_span).await
                }

                let mut s = inner.lock().await;

                s.task_done(&id);
                if is_initialize {
                    s.initialized = true;
                }
                drop(s);

                Ok(())
            } else if request.method == req::Shutdown::METHOD {
                // Shutting down without handler, everything should be OK.
                writer
                    .send(
                        rpc::Response::success(())
                            .with_request_id(request.id.unwrap())
                            .into_message(),
                    )
                    .await
            } else {
                tracing::error!(
                    method = %request.method,
                    "no request handler registered"
                );

                writer
                    .send(
                        rpc::Response::error(rpc::Error::method_not_found())
                            .with_request_id(request.id.unwrap())
                            .into_message(),
                    )
                    .await
            }
        } else {
            tracing::debug!(
                method = %request.method,
                "notification received"
            );

            if request.method == lsp_types::notification::Cancel::METHOD {
                if let Some(p) = request.params {
                    if let Ok(c) = serde_json::from_value::<lsp_types::CancelParams>(p) {
                        inner.lock().await.task_done(&c.id);
                    }
                }
                return Ok(());
            }

            let mut s = inner.lock().await;

            if s.handlers.contains_key(&request.method) {
                let mut handler = s.handlers.get_mut(&request.method).unwrap().clone();
                drop(s);

                let ctx = Server::create_context(
                    inner,
                    Arc::new(AsyncMutex::new(Box::new(writer))),
                    data,
                    &request,
                )
                .await;

                let handler_span = tracing::trace_span!(
                    "notification handler",
                    method = %request.method,
                );

                let method = request.method.clone();

                handler
                    .handle(ctx.clone(), request, None)
                    .instrument(handler_span)
                    .await;

                let deferred = mem::take(&mut (*ctx.deferred.lock().await));

                for d in deferred {
                    let deferred_span = tracing::trace_span!(
                        "deferred task",
                        %method,
                    );

                    d.instrument(deferred_span).await
                }
            } else {
                tracing::warn!(
                    method = %request.method,
                    "no notification handler registered"
                );
            }

            Ok(())
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn handle_request(
        inner: Arc<AsyncMutex<Inner<W>>>,
        data: W,
        request: rpc::Request<serde_json::Value>,
        mut writer: impl MessageWriter + Clone + 'static,
    ) -> Result<(), io::Error> {
        if &request.jsonrpc != "2.0" {
            tracing::error!("JSON-RPC message version is not 2.0");
            return writer
                .send(
                    rpc::Response::error(
                        rpc::Error::invalid_request()
                            .with_data("only JSON-RPC version 2.0 is accepted"),
                    )
                    .into_message(),
                )
                .await;
        }

        if request.id.is_some() {
            tracing::debug!(
                id = ?request.id.as_ref().unwrap(),
                method = %request.method,
                "request received"
            );
            let mut s = inner.lock().await;

            if s.shutting_down {
                tracing::warn!(
                    id = ?request.id.as_ref().unwrap(),
                    method = %request.method,
                    "received request while shutting down"
                );

                writer
                    .send(
                        rpc::Response::error(
                            rpc::Error::invalid_request().with_data("server is shutting down"),
                        )
                        .with_request_id(request.id.clone().unwrap())
                        .into_message(),
                    )
                    .await?;
                return Ok(());
            }

            if request.method == req::Shutdown::METHOD {
                tracing::info!(
                    id = ?request.id.as_ref().unwrap(),
                    method = %request.method,
                    "received shutdown request"
                );

                s.shutting_down = true;
            }

            let is_initialize = request.method == req::Initialize::METHOD;

            if !s.initialized && !is_initialize {
                tracing::error!(
                    id = ?request.id.as_ref().unwrap(),
                    method = %request.method,
                    "server not yet initialized"
                );

                writer
                    .send(
                        rpc::Response::error(rpc::Error::server_not_initialized())
                            .with_request_id(request.id.clone().unwrap())
                            .into_message(),
                    )
                    .await?;
                return Ok(());
            }

            if s.handlers.contains_key(&request.method) {
                let mut handler = s.handlers.get_mut(&request.method).unwrap().clone();

                let id = request.id.clone().unwrap();

                // We expect the handler to run for a longer time
                drop(s);

                let ctx = Server::create_context(
                    inner.clone(),
                    Arc::new(AsyncMutex::new(Box::new(writer.clone()))),
                    data,
                    &request,
                )
                .await;

                let handler_span = tracing::trace_span!(
                    "request handler",
                    method = %request.method,
                );

                let method = request.method.clone();

                handler
                    .handle(ctx.clone(), request, Some(&mut writer))
... (truncated)
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
