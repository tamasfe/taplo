# Even Better TOML

A TOML language support extension backed by [Taplo](https://github.com/tamasfe/taplo).

It is currently a **preview extension**, it might contain bugs, and might even crash. It is also not yet optimized. If you encounter any issues, please report them [on github](https://github.com/tamasfe/taplo/issues).

- [Even Better TOML](#even-better-toml)
  - [Features](#features)
    - [TOML version 1.0.0-rc.1 support](#toml-version-100-rc1-support)
    - [Semantic highlighting](#semantic-highlighting)
    - [Validation](#validation)
    - [Folding](#folding)
    - [Symbol tree and navigation (even for tables out of order)](#symbol-tree-and-navigation-even-for-tables-out-of-order)
    - [Formatting](#formatting)
  - [Planned Features](#planned-features)
    - [Autocompletion and validation based on JSON Schema](#autocompletion-and-validation-based-on-json-schema)
    - [TextMate syntax highlighting](#textmate-syntax-highlighting)

## Features

### TOML version [1.0.0-rc.1](https://github.com/toml-lang/toml/blob/master/versions/en/toml-v1.0.0-rc.1.md) support

This extension will try to support all the TOML versions in the future.

### Semantic highlighting

Value highlighting based on their types, and semantic key highlighting for tables and arrays.

![Semantic Highlighting](highlight.png)

### Validation

![Validation](validation.gif)

### Folding

Arrays, and top level tables and comments can be folded.

![Folding](folding.gif)

### Symbol tree and navigation (even for tables out of order)

![Symbols](symbols.gif)

### Formatting

Most of the formatting features are enabled by default, but almost everything can be tweaked in the settings. If you're missing a setting, feel free to open an issue about it!

![Formatting](formatting.gif)

## Planned Features

### Autocompletion and validation based on [JSON Schema](https://json-schema.org/)

This is a larger task, and it might not come in the near future, however I definitely want to include this feature.

### TextMate syntax highlighting

The colours would be pretty much remain the same, but highlighting would probably be a lot faster.

This has rather low priority for me, and I haven't written TextMate grammars before, so pull requests are welcome.