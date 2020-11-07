# Change Log

###

## 0.9.3

- Fixed syntax highlights (again...)

## 0.9.2
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