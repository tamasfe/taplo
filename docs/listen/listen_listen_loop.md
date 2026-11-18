# listen.listen_loop

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\listen.rs` (lines 23-96)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn listen_loop(
        self,
        world: W,
        mut input: UnboundedReceiver<Message>,
        output: UnboundedSender<Message>,
        mut shutdown_signals: impl Stream<Item = ()> + Unpin,
    ) -> Result<(), anyhow::Error> {
        let ls = tokio::task::LocalSet::new();

        ls.run_until(async move {
            'l: loop {
                tokio::select! {
                    _ = shutdown_signals.next() => {
                        tracing::info!("shutdown signal received, shutting down...");

                        let task_fut = self.handle_message(
                            world.clone(),
                            Request::<()>::new()
                                .with_id(Some(NumberOrString::String(
                                    "external_shutdown".to_string(),
                                )))
                                .with_method("shutdown")
                                .into_message(),
                            output.clone().sink_map_err(|e| panic!("{}", e)),
                        );

                        select!{
                            _ = shutdown_signals.next() => {
                                tracing::warn!("shut down forcibly");
                            },
                            _ = task_fut => {}
                        }

                        drop(output);
                        drop(input);
                        break 'l;
                    },
                    msg = input.next() => {
                        match msg {
                            Some(msg) => {
                                if msg.method.as_ref().map(|m| m == "exit").unwrap_or(false) {
                                    drop(output);
                                    drop(input);
                                    break;
                                }

                                let task_fut = self.handle_message(
                                    world.clone(),
                                    msg,
                                    output.clone().sink_map_err(|e| panic!("{}", e)),
                                );

                                tokio::task::spawn_local(async move {
                                    if let Err(e) = task_fut.await {
                                        tracing::error!(error = %e, "handler returned error");
                                    }
                                });
                            }
                            None => break 'l,
                        }
                    }
                };
            }

            if self.is_shutting_down().await {
                Ok(())
            } else {
                Err(anyhow!("got exit message without shutdown notice"))
            }
        })
        .await?;

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) async fn listen_loop(
        self,
        world: W,
        mut input: UnboundedReceiver<Message>,
        output: UnboundedSender<Message>,
        mut shutdown_signals: impl Stream<Item = ()> + Unpin,
    ) -> Result<(), anyhow::Error> {
        let ls = tokio::task::LocalSet::new();

        ls.run_until(async move {
            'l: loop {
                tokio::select! {
                    _ = shutdown_signals.next() => {
                        tracing::info!("shutdown signal received, shutting down...");

                        let task_fut = self.handle_message(
                            world.clone(),
                            Request::<()>::new()
                                .with_id(Some(NumberOrString::String(
                                    "external_shutdown".to_string(),
                                )))
                                .with_method("shutdown")
                                .into_message(),
                            output.clone().sink_map_err(|e| panic!("{}", e)),
                        );

                        select!{
                            _ = shutdown_signals.next() => {
                                tracing::warn!("shut down forcibly");
                            },
                            _ = task_fut => {}
                        }

                        drop(output);
                        drop(input);
                        break 'l;
                    },
                    msg = input.next() => {
                        match msg {
                            Some(msg) => {
                                if msg.method.as_ref().map(|m| m == "exit").unwrap_or(false) {
                                    drop(output);
                                    drop(input);
                                    break;
                                }

                                let task_fut = self.handle_message(
                                    world.clone(),
                                    msg,
                                    output.clone().sink_map_err(|e| panic!("{}", e)),
                                );

                                tokio::task::spawn_local(async move {
                                    if let Err(e) = task_fut.await {
                                        tracing::error!(error = %e, "handler returned error");
                                    }
                                });
                            }
                            None => break 'l,
                        }
                    }
                };
            }

            if self.is_shutting_down().await {
                Ok(())
            } else {
                Err(anyhow!("got exit message without shutdown notice"))
            }
        })
        .await?;

        Ok(())
    }
```

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
