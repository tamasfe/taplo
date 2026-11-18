# handler.Handler

- **Type:** trait
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\handler.rs` (lines 10-21)
- **Role:** protocol
- **Stability:** unknown

## Signature

```
pub(crate) trait Handler<W: Clone> {
    fn method(&self) -> &'static str;

    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        writer: Option<&mut dyn MessageWriter>,
    );

    fn box_clone(&self) -> Box<dyn Handler<W>>;
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) trait Handler<W: Clone> {
    fn method(&self) -> &'static str;

    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        writer: Option<&mut dyn MessageWriter>,
    );

    fn box_clone(&self) -> Box<dyn Handler<W>>;
}
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
