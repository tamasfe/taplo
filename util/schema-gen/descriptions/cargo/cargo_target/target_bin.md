Binary targets are executables programs that can be run after being compiled.
The default binary filename is `src/main.rs`, which defaults to the name of
the package. Additional binaries are stored in the [`src/bin/`
directory](https://doc.rust-lang.org/cargo/guide/project-layout.html). The settings for each binary can be [customized](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#configuring-a-target) in the `[[bin]]` tables in `Cargo.toml`.

Binaries can use the public API of the package's library. They are also linked
with the [`[dependencies]`](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) defined in `Cargo.toml`.

You can run individual binaries with the [`cargo run`](https://doc.rust-lang.org/cargo/commands/cargo-run.html) command with the `--bin
<bin-name>` option. [`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html) can be used to copy the executable to a
common location.

```toml
# Example of customizing binaries in Cargo.toml.
[[bin]]
name = "cool-tool"
test = false
bench = false

[[bin]]
name = "frobnicator"
required-features = ["frobnicate"]
```