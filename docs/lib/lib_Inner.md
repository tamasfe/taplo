# lib.Inner

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 275-282)
- **Role:** internal
- **Stability:** unknown

## Signature

```
struct Inner<W: Clone> {
    next_request_id: i32,
    initialized: bool,
    shutting_down: bool,
    handlers: HashMap<String, Box<dyn Handler<W>>>,
    tasks: HashMap<rpc::RequestId, Cancellation>,
    requests: HashMap<rpc::RequestId, oneshot::Sender<rpc::Response<serde_json::Value>>>,
}
```

## Docstring

_No docstring provided._

## Source

```
struct Inner<W: Clone> {
    next_request_id: i32,
    initialized: bool,
    shutting_down: bool,
    handlers: HashMap<String, Box<dyn Handler<W>>>,
    tasks: HashMap<rpc::RequestId, Cancellation>,
    requests: HashMap<rpc::RequestId, oneshot::Sender<rpc::Response<serde_json::Value>>>,
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
