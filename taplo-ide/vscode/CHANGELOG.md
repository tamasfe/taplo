# Change Log

## next

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