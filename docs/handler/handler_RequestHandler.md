# handler.RequestHandler

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\handler.rs` (lines 29-37)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>>,
    W: Clone,
{
    f: fn(Context<W>, Params<R::Params>) -> F,
    t: PhantomData<W>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>>,
    W: Clone,
{
    f: fn(Context<W>, Params<R::Params>) -> F,
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
