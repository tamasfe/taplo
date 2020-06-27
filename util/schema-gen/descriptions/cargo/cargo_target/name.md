The `name` field specifies the name of the target, which corresponds to the
filename of the artifact that will be generated. For a library, this is the
crate name that dependencies will use to reference it.

For the `[lib]` and the default binary (`src/main.rs`), this defaults to the
name of the package, with any dashes replaced with underscores. For other
[auto discovered](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery) targets, it defaults to the
directory or file name.

This is required for all targets except `[lib]`.