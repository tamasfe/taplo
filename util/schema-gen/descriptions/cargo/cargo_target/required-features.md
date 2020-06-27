The `required-features` field specifies which [features](https://doc.rust-lang.org/cargo/reference/features.html) the target needs in
order to be built. If any of the required features are not enabled, the
target will be skipped. This is only relevant for the `[[bin]]`, `[[bench]]`,
`[[test]]`, and `[[example]]` sections, it has no effect on `[lib]`.

```toml
[features]
# ...
postgres = []
sqlite = []
tools = []

[[bin]]
name = "my-pg-tool"
required-features = ["postgres", "tools"]
```
