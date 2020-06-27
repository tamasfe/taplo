The package name is an identifier used to refer to the package. It is used
when listed as a dependency in another package, and as the default name of
inferred lib and bin targets.

The name must use only [alphanumeric](https://doc.rust-lang.org/std/primitive.char.html#method.is_alphanumeric) characters or `-` or `_`, and cannot be empty.
Note that [`cargo new`](https://doc.rust-lang.org/cargo/commands/cargo-new.html) and [`cargo init`](https://doc.rust-lang.org/cargo/commands/cargo-init.html) impose some additional restrictions on
the package name, such as enforcing that it is a valid Rust identifier and not
a keyword. [crates.io](https://crates.io) imposes even more restrictions, such as
enforcing only ASCII characters, not a reserved name, not a special Windows
name such as "nul", is not too long, etc.