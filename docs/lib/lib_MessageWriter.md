# lib.MessageWriter

- **Type:** trait
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 272-272)
- **Role:** protocol
- **Stability:** unknown

## Signature

```
pub trait MessageWriter: Sink<rpc::Message, Error = io::Error> + Unpin {}
```

## Docstring

_No docstring provided._

## Source

```
pub trait MessageWriter: Sink<rpc::Message, Error = io::Error> + Unpin {}
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
