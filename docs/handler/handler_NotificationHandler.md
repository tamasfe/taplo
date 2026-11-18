# handler.NotificationHandler

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\handler.rs` (lines 117-125)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct NotificationHandler<N, F, W>
where
    N: Notification,
    F: Future,
    W: Clone,
{
    f: fn(Context<W>, Params<N::Params>) -> F,
    t: PhantomData<W>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct NotificationHandler<N, F, W>
where
    N: Notification,
    F: Future,
    W: Clone,
{
    f: fn(Context<W>, Params<N::Params>) -> F,
    t: PhantomData<W>,
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
