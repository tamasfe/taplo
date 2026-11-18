# listen.write_message

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\listen.rs` (lines 120-132)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn write_message<T: AsyncWrite + Unpin>(
    out: &mut T,
    msg: rpc::Message,
) -> anyhow::Result<()> {
    let msg = serde_json::to_vec(&msg)?;

    out.write_all(format!("Content-Length: {}\r\n\r\n", msg.len()).as_bytes())
        .await?;
    out.write_all(&msg).await?;
    out.flush().await?;

    Ok(())
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) async fn write_message<T: AsyncWrite + Unpin>(
    out: &mut T,
    msg: rpc::Message,
) -> anyhow::Result<()> {
    let msg = serde_json::to_vec(&msg)?;

    out.write_all(format!("Content-Length: {}\r\n\r\n", msg.len()).as_bytes())
        .await?;
    out.write_all(&msg).await?;
    out.flush().await?;

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
