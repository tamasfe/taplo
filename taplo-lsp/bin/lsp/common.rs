use lsp_async_stub::rpc;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub(crate) async fn write_message<T: AsyncWrite + Unpin>(
    out: &mut T,
    msg: rpc::Message,
) -> anyhow::Result<()> {
    let msg = serde_json::to_vec(&msg)?;

    out.write_all(format!("Content-Length: {}\r\n\r\n", msg.len()).as_bytes())
        .await?;
    out.write_all(&msg).await?;
    out.write_all("\r\n".as_bytes()).await?;

    out.flush().await?;

    Ok(())
}
