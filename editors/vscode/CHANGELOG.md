# Change Log

## 0.19.0

### Features

- Added `reorder_arrays` (`evenBetterToml.formatter.reorderArrays`) formatter option that allows for sorting values within arrays. ([#343](https://github.com/tamasfe/taplo/pull/343))

## 0.18.3

### Fixed

- Fixed crash caused by setting environment variables in WASM. Unfortunately this reverted `HTTP_PROXY` support.

## 0.18.2

### Fixes

- Fixed TOML parser issues.
- Fixed rules being ignored.
- `HTTP(S)_PROXY` environment variables are now respected.

## 0.18.1

### Fixes

- Fixed the language server crashing on startup.

## 0.18.0

### Features

- Added `evenBetterToml.formatter.inlineTableExpand` option.

### Fixes

- Fixed some filepath-related issues on windows.
- Fixed completion crashes on `null` default values.

## 0.17.1

### Fixes

- The bundled language server was not updated in `0.17.0`.

## 0.17.0

### Features

- The language server now accepts Taplo rules in the `evenBetterToml.rules` setting
- Show schema title in hover
- Added builtin schema for `taplo.toml`

### Fixes

- Fixed path handling on Windows
- Completion of literal strings now works
- Schema directives now should work more reliably

### Other

- Schemas now will only be fetched from the [schema store](https://schemastore.org) by default

## 0.16.5

### Fixes

- Fixed the language server ignoring the given path to the configuration file in `evenBetterToml.taplo.configFile.path`
- Environment variables are now correctly passed to the bundled language server

## 0.16.4

### Fixes

- Fixed incorrect default formatting options
- Changed auto-closing behaviour ([#273](https://github.com/tamasfe/taplo/pull/273))

## 0.16.3

### Fixes

- Fixed wrong configuration for multiple workspaces

## 0.16.2

### Other

- Web extension compatibility

## 0.16.1

### Fixes

- Even more syntax highlight fixes
- Configuration parameter fixes

## 0.16.0

### Changes

- Changed syntax highlight scopes for table and array of tables keys

### Fixes

- Syntax highlight enhancements and bug fixes

## 0.15.2

### Fixes

- Extension readme images

## 0.15.1

### Fixes

- Fixed default formatting options

## 0.15.0

### Features

- Wide-range JSON schema Draft 4 support
- Support for schema store schemas
- Support for multiple workspaces
- It is now possible to specify schemas with either `#:schema <URL>` or `"$schema" = "<URL>"` in TOML files
- Added rename feature that lets you rename keys that appear at multiple locations
- Added support for `tomlValidation` in extensions

### Other

- Almost a complete internal rewrite with various fixes and improvements. ([#211](https://github.com/tamasfe/taplo/pull/211))

## 0.14.3

### Fixes

- Formatter fixes

## 0.14.2

### Fixes

- Respect exclude rules in Taplo config.

## 0.14.1

### Fixes

- Formatter fixes

## 0.14.0

### Features

- Syntax highlight for comment directives (comments starting with `#:`)
- Syntax highlight in markdown 

### Fixes

- Formatter fixes regarding comments

## 0.13.1

### Features

- Added `indentEntries` formatter option that was supposed to be in `0.13.0`

### Fixes

- Formatter indentation fixes

## 0.13.0

### Features

- New formatting options: `alignComments` and `compactEntries`

### Fixes

- Completely reworked formatting code
- Fixed various false errors
- The `columnWidth` formatter option is more closely enforced

## 0.12.3

### Fixes

- Fixed formatting comments inside arrays
- Relative paths inside comments are relative to the document ([#119](https://github.com/tamasfe/taplo/pull/119))

## 0.12.2

### Fixes

- Fixed handling Windows-specific paths ([#114](https://github.com/tamasfe/taplo/pull/114))
- Fixed LSP crash on invalid schema URLs.

## 0.12.1

### Fixes
- Fixed LSP crash when editing table keys

## 0.12.0

### Features
- Improved refactoring code actions
- Schema directive support

## 0.11.1

### Fixes
- Fixed previous broken release

## 0.11.0

### Features
- Basic refactoring actions
  - inline array of tables -> array of tables
  - inline table -> table

### Fixes
- Formatting fixes
- Fixed semantic highlights

## 0.10.0

### Features
- Conversion commands from JSON to TOML
- Remote schema repository

### Fixes
- Better comment formatting
- Several schema fixes

## 0.9.4

### Features
- Added `rustfmt.toml` schema ([#65](https://github.com/tamasfe/taplo/pull/65))
## 0.9.3
### Fixes
- Fixed syntax highlights (again...)

## 0.9.2
### Fixes

- Fixed unusable config files

## 0.9.1

### Fixes
- Fixed missing syntax highlights

## 0.9.0

### Features
- Support for [Taplo CLI's](https://taplo.tamasfe.dev/configuration/#configuration-file) configuration file
- Support for remote (http/https) schemas

### Fixes
- Several incorrect parsing errors, mainly concerning arrays of tables.
- Rewritten parsing for increased performance
- A lot more accurate completions

## 0.8.7

### Fixes
- Python schema fixes

## 0.8.6

### Fixes
- Major Python schema improvements ([PR](https://github.com/tamasfe/taplo/pull/45))
- Quoted and unquoted keys are properly compared

## 0.8.5

### Fixes
- Python schema fix

## 0.8.4

### Fixes
- Comments inside arrays should be handled a bit better

## 0.8.3

### Fixes
- Fixed false errors in arrays of tables
- Fixed some incorrect schemas

## 0.8.2

### Fixes
- Syntax highlighting fixes
- Some completion fixes

## 0.8.1

#### Fixes
- Fixed comment highlighting ([#30](https://github.com/tamasfe/taplo/issues/30))
- Fixed validation for `pyproject.toml` schema

## 0.8.0

#### New Features
- Schemas are enabled by default
- pyproject.toml schema with Poetry

#### Fixes
- Better schema key completions
- Fixed excessive links in schemas

#### Other
- Links have to be enabled explicitly for schemas

## 0.7.2

#### Fixes
- Extension config should be properly applied now
- Removed separate language for `Cargo.lock`

## 0.7.1

#### Fixes
- Fixed incorrectly accepted arrays
- Fixed infinite loop caused by invalid arrays
- Better formatter indentation

#### Other
- Disabled formatter `alignEntries` and `formatKeys` by default ([#29](https://github.com/tamasfe/taplo/issues/29))

## 0.7.0

#### New Features
- Added allowed blank lines formatter option. (default 2)
- TextMate grammar
- Semantic highlighting option

#### Other
- Automatic indentation for the formatter is disabled by default

#### Fixes
- Document close events are not ignored anymore

## 0.6.2
- Invalid inline tables don't cause an infinite loop anymore
- Hover and links are now better handled in dotted keys

## 0.6.1

#### Fixes
- Fixed false negative error regarding dotted key and top level table conflicts
- Added another check for dotted key and top level table conflicts
- Fixed an error when enabling schemas

#### Other
- More user-friendly error messages coming from the LSP

## 0.6.0

#### New Features
- Added compact arrays formatter option
- Added compact inline tables formatter option
- Completion and validation based on JSON schema (disabled by default)

#### Fixes
- Fixed broken document symbols

## 0.5.1

### Fixes
- Fixed incorrect parsing of floats
- Fixed some incorrectly accepted table headers
- Incorrect underscores in numbers are no longer accepted
- Fixed wrong internal DOM representation (relevant to JSON conversions)
- Array and table keys are different colors again in the default theme (with a workaround until there are proper textmate scopes)
- CR (`\r`) characters are now allowed in multi-line strings

### Other
- Foundations of JSON schema support (cannot be enabled without manually building yet)
- Added DOM tree debug command

## 0.5.0

#### New Features
- Added formatter entry alignment setting

## 0.4.2

### Fixes
- Restored accidentally deleted README

## 0.4.1

### Fixes
- Use the VSCode clipboard API instead of clipboardy
- Remote workspaces should be supported
- Fixed incorrect detection of top level table conflicts with dotted keys

## 0.4.0

#### New Features
- Activation message in the status bar
- Paste as JSON command

#### Fixes
- Tables that conflict with dotted keys are no longer accepted
- Fixed too many new lines at the end when formatting
- Fixed incorrect indentation

#### Other
- Non-debug commands are always visible in the palette unless disabled
- Better error notifications

## 0.3.0

#### New Features
- Multi-line strings and string literals can be folded
- Arrays of tables have the same color as arrays. They should be easier to immediately distinguish from regular tables this way.

#### Fixes
- Non-ASCII characters should now work correctly
- Table folding ranges now start on the correct line
- Fixed parsing error that ocurred when there was no new line after multi-line strings or string literals
- Potentially misleading semantic errors are not shown anymore if there are any syntax errors

#### Other
- Added debug commands

## 0.2.0

#### New Features
- Copy selection as JSON command (enabled in TOML documents only)

#### Fixes
- Fixed incorrect array parsing

#### Other
- Bundling with webpack, and stricter packaging

## 0.1.1

#### Fixes
- Symbol tree fixes, values inside arrays are now also part of the tree

## 0.1.0
- Initial preview release