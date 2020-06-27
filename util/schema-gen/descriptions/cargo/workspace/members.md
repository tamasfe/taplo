All [`path` dependencies] residing in the workspace directory automatically
become members. Additional members can be listed with the `members` key, which
should be an array of strings containing directories with `Cargo.toml` files.

The `members` list also supports [globs] to match multiple paths, using
typical filename glob patterns like `*` and `?`.