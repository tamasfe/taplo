# listen.read_message

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\listen.rs` (lines 162-211)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn read_message<R: tokio::io::AsyncBufRead + Unpin>(
    mut input: R,
) -> Result<Option<rpc::Message>, anyhow::Error> {
    let mut size = 0;
    let mut buf = String::new();

    // Parse headers
    loop {
        buf.clear();

        if input.read_line(&mut buf).await? == 0 {
            return Ok(None);
        };

        if !buf.ends_with("\r\n") {
            return Err(anyhow!("malformed header: {:?}", buf));
        }

        let buf = &buf[..buf.len() - 2];
        if buf.is_empty() {
            break;
        }
        let mut parts = buf.splitn(2, ": ");
        let header_name = parts
            .next()
            .ok_or_else(|| anyhow!("invalid header: {}", buf))?;
        let header_value = match parts.next() {
            Some(h) => h,
            None => {
                return Err(anyhow!("malformed header: {:?}", buf));
            }
        };
        if header_name == "Content-Length" {
            size = match header_value.parse::<usize>() {
                Ok(s) => s,
                Err(err) => {
                    return Err(anyhow!("invalid content-length: {}", err));
                }
            }
        }
    }

    // Parse the message itself.
    let mut buf = buf.into_bytes();
    buf.resize(size, 0);

    input.read_exact(&mut buf).await?;

    Ok(Some(serde_json::from_slice(&buf)?))
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) async fn read_message<R: tokio::io::AsyncBufRead + Unpin>(
    mut input: R,
) -> Result<Option<rpc::Message>, anyhow::Error> {
    let mut size = 0;
    let mut buf = String::new();

    // Parse headers
    loop {
        buf.clear();

        if input.read_line(&mut buf).await? == 0 {
            return Ok(None);
        };

        if !buf.ends_with("\r\n") {
            return Err(anyhow!("malformed header: {:?}", buf));
        }

        let buf = &buf[..buf.len() - 2];
        if buf.is_empty() {
            break;
        }
        let mut parts = buf.splitn(2, ": ");
        let header_name = parts
            .next()
            .ok_or_else(|| anyhow!("invalid header: {}", buf))?;
        let header_value = match parts.next() {
            Some(h) => h,
            None => {
                return Err(anyhow!("malformed header: {:?}", buf));
            }
        };
        if header_name == "Content-Length" {
            size = match header_value.parse::<usize>() {
                Ok(s) => s,
                Err(err) => {
                    return Err(anyhow!("invalid content-length: {}", err));
                }
            }
        }
    }

    // Parse the message itself.
    let mut buf = buf.into_bytes();
    buf.resize(size, 0);

    input.read_exact(&mut buf).await?;

    Ok(Some(serde_json::from_slice(&buf)?))
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
