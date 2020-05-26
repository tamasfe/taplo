<div style="text-align:left"><img src="taplo-icon.png" width="128"></div>

[![Continuous integration](https://github.com/tamasfe/taplo/workflows/Continuous%20integration/badge.svg)](https://github.com/tamasfe/taplo/actions?query=workflow%3A%22Continuous+integration%22)
[![Latest Version](https://img.shields.io/crates/v/taplo.svg)](https://crates.io/crates/taplo)
[![Documentation](https://docs.rs/taplo/badge.svg)](https://docs.rs/taplo)

# Taplo

Taplo is a fault-tolerant [TOML](https://github.com/toml-lang/toml) parser, analyzer and formatter library written in Rust. It currently supports [TOML v1.0.0-rc.1](https://github.com/toml-lang/toml/blob/master/versions/en/toml-v1.0.0-rc.1.md).

It uses [Rowan](https://github.com/rust-analyzer/rowan) for the syntax tree.

- [Taplo](#taplo)
  - [Goals](#goals)
  - [Status](#status)
  - [Performance](#performance)
  - [Contributing](#contributing)

## Goals

The main purpose of the library is to provide tools for analyzing TOML data where the layout must be preserved and the original position of every parsed token must be known. Good examples of target applications would be linters, IDE extensions, or language servers.

Taplo also exposes a `Value` type that can be serialized with [Serde](https://github.com/serde-rs/serde) and allows converting it to JSON, YAML or any other format.

## Status

The library is **WIP**, it might have bugs, and the API can change anytime. It is currently used and tested in the [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) Visual Studio Code extension.

The correctness of the decoding is not yet entirely guaranteed (as there is no 1.0.0-rc.1 compliant test suite), and the performance is not the best ([see below](#performance)). Until the library reaches 1.0.0, you should probably use a more mature TOML parser.

The documentation is lacking, and probably won't be any good until the library becomes stable. However the API surface is not too large and should be more or less straightforward to use.

If you need something that is not exposed by the library feel free to open an issue or submit a pull request!

## Performance

The lexing is mostly done with [Logos](https://github.com/maciejhirsz/logos), so that should be pretty fast. The overall performance is _good enough_ for now. There have been no optimization efforts made yet.

For those of you who are curious, here's a comparison to [toml-rs](https://github.com/alexcrichton/toml-rs) as of *v1.0.0-alpha.3*:

```
test tests::benches::bench_taplo_parse          ... bench:     224,934 ns/iter (+/- 27,494)
test tests::benches::bench_taplo_parse_validate ... bench:     708,415 ns/iter (+/- 77,386)
test tests::benches::bench_toml_rs              ... bench:     213,743 ns/iter (+/- 31,816)
```

And the memory usage of the `taplo/examples` applications:

```
taplo_parse:  total heap usage: 6,368 allocs, 6,368 frees, 391,941 bytes allocated
toml_rs_parse: total heap usage:   840 allocs,   840 frees,  78,677 bytes allocated
```

Taplo is around 3 times slower compared to toml-rs, and allocates 5 times more memory.
This is not too terrible considering how immature the library is but there's a lot of room to improve.

## Contributing

All kinds of contributions are welcome. As always, please make sure to discuss an issue before you start working on a pull request.