The `test` field indicates whether or not the target is tested by default by
[`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html). The default is `true` for lib, bins, and tests.

> **Note**: Examples are built by [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html) by default to ensure they
> continue to compile, but they are not *tested* by default. Setting `test =
> true` for an example will also build it as a test and run any
> [`#[test]`](https://doc.rust-lang.org/reference/attributes/testing.html#the-test-attribute) functions defined in the example.