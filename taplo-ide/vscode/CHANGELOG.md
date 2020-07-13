# Change Log

## next

#### New Features
- Added allowed blank lines formatter option. (default 2)

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