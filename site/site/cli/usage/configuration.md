# Configuration

## Log Level

Taplo uses the Rust `tracing` library for configurable logging features and respects the `RUST_LOG` environment variable. All logs regardless of log level are printed to the standard error output.

In most cases you might wish to disable logging below a certain log level.
As an example if you wish to only see error messages, you can do the following:

```sh
RUST_LOG=error taplo lint foo.toml
```

The available log levels:

- `trace`
- `debug`
- `info`
- `warn`
- `error`

## Configuration File

<!-- TODO: config link -->

Taplo CLI by default searches for a Taplo config file in the current working directory, this behaviour can be disabled by either supplying `--no-auto-config` or `--config <path>` flags.

If a config file is not provided and no file is found in the current working directory, Taplo CLI will try to retrieve the configuration file path from the `TAPLO_CONFIG` environment variable.
