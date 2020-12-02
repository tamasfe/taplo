# Change Log

## Unreleased

### Additions
- Builtin `rustfmt.toml` schema added

## 0.4.0

### Breaking Changes

- `lsp-types` dependency version bump (this dependency is likely to be removed in future versions)

## 0.3.1

### Additions
- Formatter options can be updated with string key=value pairs.

## 0.3.0

**From this release Taplo only guarantees to support the latest stable Rust release**

### Breaking Changes

- Updated the library to use Rust 1.47.0 stable, it will definitely fail to build on versions older than 1.45.0.

### Fixes
- Added features to documentation
- Documentation should actually compile

## 0.2.0

### Features
- Moved analytics and schema utilities into this library
- Optional `time` and `chrono` support

### Fixes

- Fixed offset-position mapping.

## 0.1.0

### Features

- Initial proper release

## 1.0.0-alpha.x

These releases were labelled way too early incorrectly, and were yanked from the registry.
