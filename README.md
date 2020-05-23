<div style="text-align:left"><img src="taplo-icon.png" width="128"></div>

# Taplo

Taplo is a [TOML](https://github.com/toml-lang/toml) parser, analyzer and formatter library written in Rust.

It uses [Rowan](https://github.com/rust-analyzer/rowan) for the syntax tree.

- [Taplo](#taplo)
  - [Goals](#goals)
  - [Status](#status)
  - [Performance](#performance)
  - [Contributing](#contributing)

## Goals

The main purpose of the library is to provide tools for analyzing TOML data where the layout must be preserved and the original position of every parsed token must be known. Good examples are linters, IDE extensions, or language servers.

Even though Taplo exposes a `Value` that can be serialized with [Serde](https://github.com/serde-rs/serde), it should not be used if you are only interested in the actual data, and you should probably use one of the more mature TOML serializer/deserialziers.

## Status

The library is pretty much **WIP**, it has a few bugs that are being fixed, but it is already used in the [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) Visual Studio Code extension.

The documentation is a bit lacking for now, and I'm not planning to put too much effort into it before the library reaches 1.0.0 stable. However the API surface is not too large, and should be more or less straightforward to use.

If you need something that is not exposed by the library, feel free to open an issue or submit a pull request.

## Performance

The overall performance is probably _"good enough"_. There have been no optimization efforts made yet but there will be in the future after the library stabilizes.

## Contributing

All kinds of contributions are welcome. As always, please make sure to discuss an issue before you start working on a pull request.