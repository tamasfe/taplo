The `default-run` field in the `[package]` section of the manifest can be used
to specify a default binary picked by [`cargo run`](https://doc.rust-lang.org/cargo/commands/cargo-run.html). For example, when there is
both `src/bin/a.rs` and `src/bin/b.rs`:

```toml
[package]
default-run = "a"
```