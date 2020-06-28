To depend on a library located in a `git` repository, the minimum information
you need to specify is the location of the repository with the `git` key:

```toml
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand" }
```

Cargo will fetch the `git` repository at this location then look for a
`Cargo.toml` for the requested crate anywhere inside the `git` repository
(not necessarily at the root - for example, specifying a member crate name
of a workspace and setting `git` to the repository containing the workspace).

Since we haven’t specified any other information, Cargo assumes that
we intend to use the latest commit on the main branch to build our package.
You can combine the `git` key with the `rev`, `tag`, or `branch` keys to
specify something else. Here's an example of specifying that you want to use
the latest commit on a branch named `next`:

```toml
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand", branch = "next" }
```

See [Git Authentication](https://doc.rust-lang.org/cargo/appendix/git-authentication.html) for help with git authentication for private repos.

> **Note**: [crates.io](https://crates.io/) does not allow packages to be published with `git`
> dependencies (`git` [dev-dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies) are ignored). See the [Multiple
> locations](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#multiple-locations) section for a fallback alternative.
