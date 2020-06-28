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

Taplo also exposes a `Value` type that can be serialized with [Serde](https://github.com/serde-rs/serde) and allows converting it to JSON, YAML or any other format. TOML Serialization is not the main goal of the library, and is not supported yet.

## Status

The library is **WIP**, it might have bugs, and the API can change anytime, and I don't yet guarantee seamless usage. It is currently used and tested in the [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) Visual Studio Code extension.

The correctness of the decoding is not yet entirely guaranteed (as there is no official 1.0.0-rc.1 compliance test suite yet), and the performance is not yet up to par with other parsers ([see below](#performance)), however it has deemed to be fast enough so far.

## Performance

The lexing is mostly done with [Logos](https://github.com/maciejhirsz/logos), so that should be pretty fast. The overall performance is _good enough_. There have been no major optimization efforts made yet.

It is rather tricky to compare Taplo with other parser implementations because despite being a TOML parser, its use-case is slightly different compared to what one would expect.

A workflow with Taplo usually involves two stages:
- **Parsing into a green tree**: This just parses the tokens into a syntax tree that can be traversed, checking for disallowed characters also happens at this stage right now, but nothing else.
- **Constructing a DOM**: This part is much more compute-intensive, it merges all the dotted keys, tables, etc. found in the document and creates a JSON-like DOM. For this sometimes it might traverse the entire document multiple times in order to set the correct overlapping spans and so on.

For those of you who are curious, here's a comparison to [toml-rs](https://github.com/alexcrichton/toml-rs) as of *1.0.0-alpha.6*:

```
test bench_taplo_parse          ... bench:     200,305 ns/iter (+/- 15,342)
test bench_taplo_parse_validate ... bench:     834,096 ns/iter (+/- 249,718)
test bench_toml_rs              ... bench:     231,208 ns/iter (+/- 69,628)
```

And the memory usage of the `taplo/examples` applications:

```
taplo_parse:   total heap usage: 2,597 allocs, 2,597 frees, 324,388 bytes allocated
toml_rs_parse: total heap usage:   840 allocs,   840 frees,  78,750 bytes allocated
```

When doing a full parse with the DOM involved Taplo is around 4 times slower compared to toml-rs, and allocates about 4 times as much memory.
This is not too terrible, but probably there's a lot of room to improve.

## Contributing

All kinds of contributions are welcome. As always, please make sure to discuss an issue before you start working on a pull request.