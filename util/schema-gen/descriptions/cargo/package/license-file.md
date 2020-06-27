The `license-file` field contains the path to a file
containing the text of the license (relative to this `Cargo.toml`).

```toml
[package]
# ...
license-file = "LICENSE.txt"
```

> **Note**: [crates.io](https://crates.io) requires either `license` or `license-file` to be set.