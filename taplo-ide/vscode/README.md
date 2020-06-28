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
    - [Commands](#commands)
      - [Copy Selection as JSON](#copy-selection-as-json)
      - [Paste as JSON](#paste-as-json)
  - [Incomplete Features](#incomplete-features)
    - [JSON schema support](#json-schema-support)
      - [Built-in schemas](#built-in-schemas)
        - [Cargo.toml](#cargotoml)
  - [Planned Features](#planned-features)
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

Arrays, multi-line strings and top level tables and comments can be folded.

![Folding](folding.gif)

### Symbol tree and navigation (even for tables out of order)

![Symbols](symbols.gif)

### Formatting

Most of the formatting features are enabled by default, but almost everything can be tweaked in the settings. If you're missing a setting, feel free to open an issue about it!

![Formatting](formatting.gif)

### Commands

Some commands are always in the palette, these can be hidden in the settings.

#### Copy Selection as JSON

Copies the selected TOML text converted to JSON to the system clipboard.

#### Paste as JSON

Parses the TOML text from the system clipboard and pastes it converted into JSON in the active selection.

## Incomplete Features

These features are not yet complete and are disabled by default.

### [JSON schema](https://json-schema.org/) support

*Can be enabled with the `evenBetterToml.schema.enabled` configuration.*

There is experimental support for completion, hover text, links and validation.

Schemas can be associated with document URIs with the `evenBetterToml.schema.associations` configuration.

You can provide your own schemas or use the ones provided with the extension. More details [here](https://github.com/tamasfe/taplo/tree/master/taplo-ide/schemas).

![Schema](schema.gif)

#### Built-in schemas

Several schemas are bundled with the extension, these can be turned off individually by removing their associations.

##### Cargo.toml

Most of `Cargo.toml` is covered with documentation and enum values, but not everything yet.

## Planned Features

Features that are definitely planned but no work has been done.

### TextMate syntax highlighting

The colours would be pretty much remain the same, but highlighting would probably be a lot faster.

This has rather low priority for me, and I haven't written TextMate grammars before, so pull requests are welcome.