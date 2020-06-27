Files located under the [examples directory](https://doc.rust-lang.org/cargo/guide/project-layout.html) are example uses of the functionality provided by the library. When compiled, they are placed in the[ target/debug/examples directory](https://doc.rust-lang.org/cargo/guide/build-cache.html).

Examples can use the public API of the package's library. They are also linked with the [dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) and [dev-dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies) defined in Cargo.toml.

By default, examples are executable binaries (with a `main()` function). You
can specify the [`crate-type` field](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field) to make an example
be compiled as a library:

```toml
[[example]]
name = "foo"
crate-type = ["staticlib"]
```

You can run individual executable examples with the [`cargo run`](https://doc.rust-lang.org/cargo/commands/cargo-run.html) command with
the `--example <example-name>` option. Library examples can be built with
[`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) with the `--example <example-name>` option. [`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html)
with the `--example <example-name>` option can be used to copy executable
binaries to a common location. Examples are compiled by [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html) by
default to protect them from bit-rotting. Set [the `test`
field](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-test-field) to `true` if you have `#[test]` functions in the
example that you want to run with [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html).
