<div style="text-align:left"><img src="taplo-icon.png" width="128"></div>

[![Continuous integration](https://github.com/tamasfe/taplo/workflows/Continuous%20integration/badge.svg)](https://github.com/tamasfe/taplo/actions?query=workflow%3A%22Continuous+integration%22)
[![Latest Version](https://img.shields.io/crates/v/taplo.svg)](https://crates.io/crates/taplo)
[![Documentation](https://docs.rs/taplo/badge.svg)](https://docs.rs/taplo)

[**Website**](https://taplo.tamasfe.dev)

# Taplo

This is the repository for Taplo, a TOML v1.0.0 toolkit, more details on the [website](https://taplo.tamasfe.dev).


- [Taplo](#taplo)
  - [Status](#status)
  - [Contributing](#contributing)

## Status

The project is very young, so bugs and incomplete features are expected, so [any help is welcome](CONTRIBUTING.md)!

The correctness of the TOML parsing and decoding is not yet entirely guaranteed (as there is no official 1.0.0 compliance test suite yet).

## Contributing

All kinds of contributions are welcome. Make sure to read the [CONTRIBUTING.md](CONTRIBUTING.md) first!

## Use it in Github Workflow

To use taplo in your Github workflow, you can add a step :
```yaml
      uses: tamasfe/taplo
```
Setup the configuration in a file on your repository: [Documentation](https://taplo.tamasfe.dev/configuration/file.html).
And you can configure what running with action inputs: `format`, `format_write_changes`, `lint` and `version`.
`format_write_changes` don't update the commits, should be used with additional step if changes should be integrated (tool like autofix.ci)