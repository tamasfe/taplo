# lib.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 299-310)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn new() -> ServerBuilder<W> {
        ServerBuilder {
            inner: Inner {
                next_request_id: 0,
                initialized: false,
                shutting_down: false,
                handlers: HashMap::new(),
                tasks: HashMap::new(),
                requests: HashMap::new(),
            },
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn new() -> ServerBuilder<W> {
        ServerBuilder {
            inner: Inner {
                next_request_id: 0,
                initialized: false,
                shutting_down: false,
                handlers: HashMap::new(),
                tasks: HashMap::new(),
                requests: HashMap::new(),
            },
        }
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
