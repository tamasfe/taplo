# Cargo

If you have a [Rust](https://www.rust-lang.org) toolchain installed, you can install Taplo via the [`taplo-cli`](https://crates.io/crates/taplo-cli) crate from crates.io.


```sh
cargo install taplo-cli --locked
```

::: warning

Taplo depends on [OpenSSL](https://www.openssl.org) in order to fetch schemas via HTTPS, you will most likely need the `openssl` development files to be installed (`openssl-dev` or `openssl-devel` on most Linux-based systems).

:::

::: tip

Make sure to use `--locked` if you run into weird compile errors due to incompatible dependencies.

:::

## Features

The following features are available:

- `lsp`: the language server
- `toml-test`: an interface for [toml-test](https://github.com/BurntSushi/toml-test)
