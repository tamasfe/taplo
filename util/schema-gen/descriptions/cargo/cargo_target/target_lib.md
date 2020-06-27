The library target defines a "library" that can be used and linked by other
libraries and executables. The filename defaults to `src/lib.rs`, and the name
of the library defaults to the name of the package. A package can have only
one library. The settings for the library can be [customized](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#configuring-a-target) in the `[lib]`
table in `Cargo.toml`.

```toml
# Example of customizing the library in Cargo.toml.
[lib]
crate-type = ["cdylib"]
bench = false
```
