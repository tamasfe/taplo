# lsp.execute_lsp

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\lsp.rs` (lines 9-32)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn execute_lsp(&mut self, cmd: LspCommand) -> Result<(), anyhow::Error> {
        self.schemas
            .cache()
            .set_cache_path(cmd.general.cache_path.clone());

        let config = self.load_config(&cmd.general).await?;

        let server = taplo_lsp::create_server();
        let world = taplo_lsp::create_world(NativeEnvironment::new());
        world.set_default_config(config);

        match cmd.io {
            LspCommandIo::Tcp { address } => {
                server
                    .listen_tcp(world, &address, async_ctrlc::CtrlC::new().unwrap())
                    .await
            }
            LspCommandIo::Stdio {} => {
                server
                    .listen_stdio(world, async_ctrlc::CtrlC::new().unwrap())
                    .await
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn execute_lsp(&mut self, cmd: LspCommand) -> Result<(), anyhow::Error> {
        self.schemas
            .cache()
            .set_cache_path(cmd.general.cache_path.clone());

        let config = self.load_config(&cmd.general).await?;

        let server = taplo_lsp::create_server();
        let world = taplo_lsp::create_world(NativeEnvironment::new());
        world.set_default_config(config);

        match cmd.io {
            LspCommandIo::Tcp { address } => {
                server
                    .listen_tcp(world, &address, async_ctrlc::CtrlC::new().unwrap())
                    .await
            }
            LspCommandIo::Stdio {} => {
                server
                    .listen_stdio(world, async_ctrlc::CtrlC::new().unwrap())
                    .await
            }
        }
    }
```

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
