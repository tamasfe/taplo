# lib.create_lsp

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lib.rs` (lines 215-232)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn create_lsp(env: JsValue, lsp_interface: JsValue) -> lsp::TaploWasmLsp {
    
    use taplo_common::log::setup_stderr_logging;

    let env = WasmEnvironment::from(env);

    // for (key, value) in env.env_vars() {
    //     std::env::set_var(key, value);
    // }

    setup_stderr_logging(env.clone(), false, false, None);

    lsp::TaploWasmLsp {
        server: taplo_lsp::create_server(),
        world: taplo_lsp::create_world(env),
        lsp_interface: lsp::WasmLspInterface::from(lsp_interface),
    }
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn create_lsp(env: JsValue, lsp_interface: JsValue) -> lsp::TaploWasmLsp {
    
    use taplo_common::log::setup_stderr_logging;

    let env = WasmEnvironment::from(env);

    // for (key, value) in env.env_vars() {
    //     std::env::set_var(key, value);
    // }

    setup_stderr_logging(env.clone(), false, false, None);

    lsp::TaploWasmLsp {
        server: taplo_lsp::create_server(),
        world: taplo_lsp::create_world(env),
        lsp_interface: lsp::WasmLspInterface::from(lsp_interface),
    }
}
```

## Git History

- Commits: 8
- Authors: 2
- Bug fixes: 5
- Features: 2

## Callers

None

## Callees

None
