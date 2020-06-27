Profile settings can be overridden for specific packages and build-time
crates. To override the settings for a specific package, use the `package`
table to change the settings for the named package:

```toml
# The `foo` package will use the -Copt-level=3 flag.
[profile.dev.package.foo]
opt-level = 3
```

The package name is actually a [Package ID Spec](https://doc.rust-lang.org/cargo/reference/pkgid-spec.html), so you can
target individual versions of a package with syntax such as
`[profile.dev.package."foo:2.1.0"]`.

To override the settings for all dependencies (but not any workspace member),
use the `"*"` package name:

```toml
# Set the default for dependencies.
[profile.dev.package."*"]
opt-level = 2
```

To override the settings for build scripts, proc macros, and their
dependencies, use the `build-override` table:

```toml
# Set the settings for build scripts and proc-macros.
[profile.dev.build-override]
opt-level = 3
```

> Note: When a dependency is both a normal dependency and a build dependency,
> Cargo will try to only build it once when `--target` is not specified. When
> using `build-override`, the dependency may need to be built twice, once as a
> normal dependency and once with the overridden build settings. This may
> increase initial build times.
