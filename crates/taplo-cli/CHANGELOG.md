# Change Log

## 0.10.0

### Fixes

- Fix incorrect error locations for unexpected entries ([#680](https://github.com/tamasfe/taplo/pull/664))

### Features

- Add `completions` subcommand for generating shell completions (enabled via `completions` feature)
- Move linter command behind `lint` feature (enabled by default)
- Move LSP command behind `lsp` feature (enabled by default)
- Improve error locations for unexpected properties ([#664](https://github.com/tamasfe/taplo/pull/664))

## 0.9.3

### Fixes

- Regression: `[rule.formatting]` completely overrides the default outside of the specified sections ([#634](https://github.com/tamasfe/taplo/issues/634))

## 0.9.2

This is a re-release of 0.9.1

## 0.9.1

### Fixes

- Do not enable default-tls unconditionally ([#554](https://github.com/tamasfe/taplo/pull/554))

## 0.9.0

### Features

- add ENV variable support ([#427](https://github.com/tamasfe/taplo/pull/427))
- Support adding custom CA ([#454](https://github.com/tamasfe/taplo/pull/454))
- Toggle off default clap dependency for prettydiff ([#521](https://github.com/tamasfe/taplo/pull/521))

## 0.8.0

### Features

- additional pre-built architectures ([#330](https://github.com/tamasfe/taplo/pull/330))
- reorder_arrays formatting option ([#343](https://github.com/tamasfe/taplo/pull/343))

## Changes

- Removed OpenSSL dependency ([#302](https://github.com/tamasfe/taplo/pull/302))

## 0.7.0

### Features

- `--diff` flag for displaying differences between formatted and unformatted toml ([#301](https://github.com/tamasfe/taplo/pull/301))

## Changes

- Removed OpenSSL dependency ([#302](https://github.com/tamasfe/taplo/pull/302))

## 0.6.8

### Changes

- Appropriate error when the LSP feature is not available

## 0.6.3

### Fixes

- Fixed schema directive handling in the CLI

## 0.6.2

### Features

- Add `--stdin-filepath` option to format cmd ([#250](https://github.com/tamasfe/taplo/pull/250))

## 0.6.1

### Fixes

- Match config when using format cmd `--stdin-filepath` ([#418](https://github.com/tamasfe/taplo/pull/418))
- Use a single stdout object to avoid interleaving ([#424](https://github.com/tamasfe/taplo/pull/424))
