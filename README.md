<div style="text-align:left"><img src="taplo-icon.png" width="128"></div>

[![Continuous integration](https://github.com/tamasfe/taplo/workflows/Continuous%20integration/badge.svg)](https://github.com/tamasfe/taplo/actions?query=workflow%3A%22Continuous+integration%22)
[![Latest Version](https://img.shields.io/crates/v/taplo.svg)](https://crates.io/crates/taplo)
[![Documentation](https://docs.rs/taplo/badge.svg)](https://docs.rs/taplo)

# Taplo

Taplo is a fault-tolerant [TOML](https://github.com/toml-lang/toml) parser, analyzer and formatter library written in Rust. It currently supports [TOML v1.0.0-rc.3](https://toml.io/en/v1.0.0-rc.3).

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

The correctness of the decoding is not yet entirely guaranteed (as there is no official 1.0.0 compliance test suite yet), and the performance is not yet up to par with other parsers ([see below](#performance)), however it has deemed to be fast enough so far.

## Performance

The lexing is mostly done with [Logos](https://github.com/maciejhirsz/logos), so that should be pretty fast. The overall performance is good enough for most use-cases.

It is rather tricky to compare Taplo with other parser implementations because of its unusual goal.

A workflow with Taplo usually involves two stages:
- **Parsing into a green tree**: This just parses the tokens into a syntax tree that can be traversed, checking for disallowed characters also happens at this stage right now, but nothing else.
- **Constructing a DOM**: This part is much more compute-intensive, it merges all the dotted keys, tables, etc. found in the document and creates a JSON-like DOM.

For those of you who are curious, here's a comparison to [toml-rs](https://github.com/alexcrichton/toml-rs) as of *0.1.0*:

```
test bench_taplo_parse          ... bench:     109,804 ns/iter (+/- 1,187)
test bench_taplo_parse_validate ... bench:     220,236 ns/iter (+/- 6,148)
test bench_toml_rs              ... bench:     140,089 ns/iter (+/- 1,392)
```

When doing a full parse with the DOM involved Taplo is only around 1.5 times slower compared to toml-rs, however it might allocate significantly more by nature.

## Contributing

All kinds of contributions are welcome. As always, please make sure to discuss an issue before you start working on a pull request.
