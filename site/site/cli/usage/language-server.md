# Language Server

The TOML language server can be used via the CLI and it supports communication via standard i/o or TCP.

::: tip

The language server is not part of the default builds, and is not available if Taplo was installed via NPM.

Consult the build or installation documentation on how to enable the functionality.

:::

## Via Standard i/o

```
taplo lsp stdio
```

In this mode Taplo expects messages from the standard input, and will print messages intended for the client to the standard output.

## Via TCP

```
taplo lsp tcp --address 0.0.0.0:9181
```

The server will listen on the given TCP address.

Multiple clients are not supported.

## Debugging

### CLI

Taplo uses the Rust `tracing` library for logging. Set `RUST_LOG` to choose
which events are logged, use `--verbose` for the expanded log format, and use
`--log-spans` to include span creation and closure events. For example:

```sh
RUST_LOG="info,taplo=debug" taplo --verbose --log-spans lsp stdio
```

`RUST_LOG=trace` enables the most detailed output, but can be very noisy. See
the [logging configuration](configuration.md#log-level) for the available log
levels.

Logs are written to standard error. In standard I/O mode, standard output is
reserved for Language Server Protocol messages, so do not combine standard
error with standard output. If the client does not capture the logs, redirect
only standard error to a file:

```sh
RUST_LOG=trace taplo --verbose --log-spans lsp stdio 2>taplo-lsp.log
```

### Visual Studio Code

In the desktop extension, open the **Output** panel and select
**Even Better TOML LSP** from the channel list.

The bundled language server supports environment variables, so logging can be
enabled in `settings.json` without installing the Taplo CLI:

```jsonc
"evenBetterToml.taplo.environment": {
  "RUST_LOG": "info,taplo=debug"
}
```

For the full trace format and span events, use an external Taplo executable.
The `path` and `extraArgs` settings only apply when the bundled server is
disabled:

```jsonc
"evenBetterToml.taplo.bundled": false,
"evenBetterToml.taplo.path": "/absolute/path/to/taplo",
"evenBetterToml.taplo.environment": {
  "RUST_LOG": "trace"
},
"evenBetterToml.taplo.extraArgs": ["--verbose", "--log-spans"]
```

After changing these launch settings, run **Developer: Reload Window** so that
the language server restarts with the new environment and arguments.

The web extension always uses its browser worker. Inspect the browser developer
console for its log output. The worker does not have access to environment
variables, and `taplo.path` and `taplo.extraArgs` do not apply, so `RUST_LOG`
cannot currently be configured there.

### Reporting a problem

When reporting a problem, include:

- the Taplo CLI or Even Better TOML extension version;
- the operating system and whether the extension is desktop or web;
- whether the desktop extension uses the bundled or an external server;
- a minimal reproduction and the relevant Taplo and editor settings; and
- logs covering the reproduction, preferably with a focused `RUST_LOG` filter
  before resorting to `trace`.

Review the complete log and configuration before sharing them. Redact tokens,
credentials, private URLs, and other secrets. Replace sensitive user names and
local path components consistently while preserving details that matter to the
problem, such as path separators, drive letters, and the workspace layout.
