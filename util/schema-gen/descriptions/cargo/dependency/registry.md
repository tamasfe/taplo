To specify a dependency from a registry other than [crates.io](https://crates.io), first the
registry must be configured in a `.cargo/config.toml` file. See the [registries
documentation](https://doc.rust-lang.org/cargo/reference/registries.html) for more information. In the dependency, set the `registry` key
to the name of the registry to use.

```toml
[dependencies]
some-crate = { version = "1.0", registry = "my-registry" }
```

> **Note**: [crates.io](https://crates.io) does not allow packages to be published with
> dependencies on other registries.
