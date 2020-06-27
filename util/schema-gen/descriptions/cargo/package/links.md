The `links` field specifies the name of a native library that is being linked
to. More information can be found in the [`links`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key) section of the build
script guide.

```toml
[package]
# ...
links = "foo"
```