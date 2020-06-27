The `incremental` setting controls the [`-C incremental` flag](https://doc.rust-lang.org/rustc/codegen-options/index.html#incremental) which controls
whether or not incremental compilation is enabled. Incremental compilation
causes `rustc` to to save additional information to disk which will be reused
when recompiling the crate, improving re-compile times. The additional
information is stored in the `target` directory.

The valid options are:

* `true`: enabled
* `false`: disabled

Incremental compilation is only used for workspace members and "path"
dependencies.

The incremental value can be overridden globally with the `CARGO_INCREMENTAL`
[environment variable](https://doc.rust-lang.org/cargo/reference/environment-variables.html) or the [`build.incremental`](https://doc.rust-lang.org/cargo/reference/config.html#buildincremental) config variable.