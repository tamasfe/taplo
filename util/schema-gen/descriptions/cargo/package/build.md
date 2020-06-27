The `build` field specifies a file in the package root which is a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) for building native code. More information can be found in the [build script guide](https://doc.rust-lang.org/cargo/reference/build-scripts.html).


```toml
[package]
# ...
build = "build.rs"
```

The default is `"build.rs"`, which loads the script from a file named
`build.rs` in the root of the package. Use `build = "custom_build_name.rs"` to
specify a path to a different file or `build = false` to disable automatic
detection of the build script.
