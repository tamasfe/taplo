Disable automatic discovery of `bin` targets.

Disabling automatic discovery should only be needed for specialized
situations. For example, if you have a library where you want a *module* named
`bin`, this would present a problem because Cargo would usually attempt to
compile anything in the `bin` directory as an executable. Here is a sample
layout of this scenario:

```
├── Cargo.toml
└── src
    ├── lib.rs
    └── bin
        └── mod.rs
```

To prevent Cargo from inferring `src/bin/mod.rs` as an executable, set
this to `false` to disable auto-discovery.