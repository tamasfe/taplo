The `[workspace]` table in `Cargo.toml` defines which packages are members of
the workspace:

```toml
[workspace]
members = ["member1", "path/to/member2", "crates/*"]
exclude = ["crates/foo", "path/to/other"]
```

An empty `[workspace]` table can be used with a `[package]` to conveniently
create a workspace with the package and all of its path dependencies.

All [`path` dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies) residing in the workspace directory automatically
become members. Additional members can be listed with the `members` key, which
should be an array of strings containing directories with `Cargo.toml` files.

The `members` list also supports [globs](https://docs.rs/glob/0.3.0/glob/struct.Pattern.html) to match multiple paths, using
typical filename glob patterns like `*` and `?`.

The `exclude` key can be used to prevent paths from being included in a
workspace. This can be useful if some path dependencies aren't desired to be
in the workspace at all, or using a glob pattern and you want to remove a
directory.

An empty `[workspace]` table can be used with a `[package]` to conveniently
create a workspace with the package and all of its path dependencies.