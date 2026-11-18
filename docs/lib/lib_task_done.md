# lib.task_done

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 285-290)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn task_done(&mut self, id: &rpc::RequestId) {
        if let Some(mut t) = self.tasks.remove(id) {
            t.cancel();
            tracing::trace!(?id, "task completed");
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn task_done(&mut self, id: &rpc::RequestId) {
        if let Some(mut t) = self.tasks.remove(id) {
            t.cancel();
            tracing::trace!(?id, "task completed");
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
