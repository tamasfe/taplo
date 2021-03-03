use lsp_async_stub::rpc;
use lsp_types::{InitializeParams, NumberOrString};
use std::{process::Stdio, thread, time::Duration};
use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    net::TcpStream,
    process::Command,
    time::timeout,
};

pub(crate) async fn write_message<T: AsyncWrite + Unpin>(
    out: &mut T,
    msg: &rpc::Message,
) -> anyhow::Result<()> {
    let msg = serde_json::to_vec(&msg)?;

    out.write_all(format!("Content-Length: {}\r\n\r\n", msg.len()).as_bytes())
        .await?;
    out.write_all(&msg).await?;
    out.write_all("\r\n".as_bytes()).await?;

    out.flush().await?;

    Ok(())
}

fn client_messages() -> impl Iterator<Item = rpc::Message> {
    #[allow(deprecated)]
    vec![
        rpc::Request::new()
            .with_method("initialize")
            .with_params(Some(InitializeParams {
                process_id: Default::default(),
                root_path: Default::default(),
                root_uri: Default::default(),
                initialization_options: Default::default(),
                capabilities: Default::default(),
                trace: Default::default(),
                workspace_folders: Default::default(),
                client_info: Default::default(),
                locale: Default::default(),
            }))
            .with_id(Some(NumberOrString::Number(1)))
            .into_message(),
        rpc::Request::<()>::new()
            .with_method("shutdown")
            .with_id(Some(NumberOrString::Number(2)))
            .into_message(),
        rpc::Request::<()>::new().with_method("exit").into_message(),
    ]
    .into_iter()
}

#[tokio::test]
async fn test_tcp() {
    let mut lsp = Command::new("cargo")
        .args("run --bin taplo-lsp -- listen".split(' '))
        .spawn()
        .unwrap();

    // Give it some time to start.
    thread::sleep(Duration::from_secs(2));

    let mut stream = TcpStream::connect("localhost:5000").await.unwrap();

    for msg in client_messages() {
        write_message(&mut stream, &msg).await.unwrap();
    }

    if let Err(_) = timeout(Duration::from_secs(2), lsp.wait()).await {
        let _ = lsp.kill().await;
        // TODO
        // panic!("the LSP still running after shutdown");
    }
}

#[tokio::test]
async fn test_stdio() {
    let mut lsp = Command::new("cargo")
        .args("run --bin taplo-lsp -- run".split(' '))
        .stdout(Stdio::null())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    // Give it some time to start.
    thread::sleep(Duration::from_secs(2));

    let mut stdin = lsp.stdin.take().unwrap();

    for msg in client_messages() {
        write_message(&mut stdin, &msg).await.unwrap();
    }

    if let Err(_) = timeout(Duration::from_secs(2), lsp.wait()).await {
        let _ = lsp.kill().await;
        // TODO
        // panic!("the LSP still running after shutdown");
    }
}
