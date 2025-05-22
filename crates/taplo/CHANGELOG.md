# Change Log

## 0.14.0

### Fixes

- Pass `include_children = true` to `text_ranges` in formatting (#684)

### Features

- Improve error locations for unexpected properties (#664)
- Sort inline tables (#671)

### Breaking Changes

- Bump MSRV to 1.74
- `text_ranges` now accepts `include_children: bool` parameter

## 0.13.2

### Fixes

- Revert "Account for trailing comments in span handling (#527)" to fix sorting regression

## 0.13.1

### Fixes

- Account for trailing comments in span handling ([#527](https://github.com/tamasfe/taplo/pull/527))

## 0.13.0

### Features

- Bump `toml` dependency to `0.7` ([#502](https://github.com/tamasfe/taplo/pull/502))

### Fixes

- Simplify and fix table indentation logic ([#450](https://github.com/tamasfe/taplo/pull/450))
- Fix comments with brackets ([#456](https://github.com/tamasfe/taplo/pull/456))
- Sort by crate lexicographical order ([#513](https://github.com/tamasfe/taplo/pull/513))

## 0.12.1

### Features

- Added `align_single_comments` formatter setting ([#409](https://github.com/tamasfe/taplo/pull/409))

## 0.12.0

### Features

- Added `reorder_arrays` formatter setting ([#343](https://github.com/tamasfe/taplo/pull/343))

## 0.11.1

### Fixes

- Fixed incorrectly disallowing comments after table headers.

## 0.11.0

### Features

- `inline_table_expand` formatting option can be used to control whether to expand values in inline tables.

### Fixes

- Time values were sometimes incorrectly parsed as integers.

## 0.10.0

### Features

- Basic escaping of strings is now supported.

### Breaking Changes

- DOM nodes can be printed with literal strings, and a preference option must be passed to `to_toml` and `to_toml_fmt`.

## 0.9.0

TODO

## 0.8.0

TODO

## 0.7.0

### Breaking Changes

- Bumped Rowan version to `0.14.1`

### Fixes

- Formatter fixes

## 0.6.3

### Fixes

- Formatter fixes

## 0.6.2

### Fixes

- Several comment formatting fixes

## 0.6.1

This is a quick followup version containing a feature that was forgotten in `0.6.0`.

### Features

- Indent entries formatter option

### Fixes

- Formatter indentation fixes

## 0.6.0

### Breaking Changes

- Bumped various dependency versions, most importantly Rowan
- `wasm-bindgen` dependency was made into a feature that can be disabled ([#133](https://github.com/tamasfe/taplo/pull/133))
- Added new formatter options, and formatting results might not always match the existing behaviour

### Fixes

- Fixed false parser and DOM errors
- Fixed some formatter incostencies

## 0.5.2

### Fixes

- Formatting fixes

## 0.5.1

### Fixes

- Fixed comment formatting

## 0.5.0

### Breaking Changes

- Removed `lsp-types` dependency
- Removed builtin schemas

### Fixes

- Formatting fixes

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
