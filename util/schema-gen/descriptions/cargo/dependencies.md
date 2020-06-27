Cargo is configured to look for dependencies on [crates.io](https://crates.io) by default. Only
the name and a version string are required in this case. In [the cargo
guide](https://doc.rust-lang.org/cargo/guide/index.html), we specified a dependency on the `time` crate:

```toml
[dependencies]
time = "0.1.12"
```

The string `"0.1.12"` is a [semver](https://github.com/steveklabnik/semver#requirements) version requirement. Since this
string does not have any operators in it, it is interpreted the same way as
if we had specified `"^0.1.12"`, which is called a caret requirement.

A dependency can also be defined by a table with additional options:

```toml
[dependencies]
time = { path = "../time", version = "0.1.12" }
```