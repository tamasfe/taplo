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

Taplo also exposes a `Value` type that can be serialized with [Serde](https://github.com/serde-rs/serde) and allows converting it to JSON, YAML or any other format. Serialization is not the main goal of the library, and is not supported yet.

## Status

The library is **WIP**, it might have bugs, and the API can change anytime. It is currently used and tested in the [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) Visual Studio Code extension.

The correctness of the decoding is not yet entirely guaranteed (as there is no official 1.0.0-rc.1 compliance test suite yet), and the performance is not yet up to par with other parsers ([see below](#performance)), however it has deemed to be fast enough so far.

## Performance

The lexing is mostly done with [Logos](https://github.com/maciejhirsz/logos), so that should be pretty fast. The overall performance is _good enough_. There have been no major optimization efforts made yet.

For those of you who are curious, here's a comparison to [toml-rs](https://github.com/alexcrichton/toml-rs) as of *1.0.0-alpha.5*:

```
test bench_taplo_parse          ... bench:     218,674 ns/iter (+/- 12,876)
test bench_taplo_parse_validate ... bench:     576,393 ns/iter (+/- 38,857)
test bench_toml_rs              ... bench:     230,086 ns/iter (+/- 54,954)
```

And the memory usage of the `taplo/examples` applications:

```
taplo_parse:   total heap usage: 2,597 allocs, 2,597 frees, 324,388 bytes allocated
toml_rs_parse: total heap usage:   840 allocs,   840 frees,  78,750 bytes allocated
```

Taplo is around 2.5 times slower compared to toml-rs, and allocates about 4 times as much memory.
This is not too terrible, but there's a lot of room to improve.

## Contributing

All kinds of contributions are welcome. As always, please make sure to discuss an issue before you start working on a pull request.