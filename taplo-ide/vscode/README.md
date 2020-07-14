# Even Better TOML

A TOML language support extension backed by [Taplo](https://github.com/tamasfe/taplo).

It is currently a **preview extension**, it might contain bugs, or might even crash. If you encounter any issues, please report them [on github](https://github.com/tamasfe/taplo/issues).

- [Even Better TOML](#even-better-toml)
  - [Features](#features)
    - [TOML version 1.0.0-rc.1 support](#toml-version-100-rc1-support)
    - [Syntax highlighting](#syntax-highlighting)
      - [Extended Colors](#extended-colors)
    - [Semantic highlighting](#semantic-highlighting)
    - [Validation](#validation)
    - [Folding](#folding)
    - [Symbol tree and navigation](#symbol-tree-and-navigation)
    - [Formatting](#formatting)
    - [Commands](#commands)
      - [Copy Selection as JSON](#copy-selection-as-json)
      - [Paste as JSON](#paste-as-json)
  - [Incomplete Features](#incomplete-features)
    - [JSON schema support](#json-schema-support)
      - [Built-in schemas](#built-in-schemas)
        - [Cargo.toml](#cargotoml)
  - [Planned Features](#planned-features)
    - [Configuration File](#configuration-file)
  - [Special Thanks](#special-thanks)

## Features

### TOML version [1.0.0-rc.1](https://toml.io/en/v1.0.0-rc.1) support

This extension will try to support all the TOML versions in the future.

### Syntax highlighting

Syntax highlighting for TOML documents with TextMate grammar.

![Syntax Highlighting](highlight.png)

#### Extended Colors

The extension defines custom scopes for array headers and arrays of tables.

In order to differentiate them from regular keys, you can set your own colors for them. Unfortunately this [has to be done manually](https://github.com/Microsoft/vscode/issues/32813).

You might also want to set a color for dates and times, as they don't have have one in most themes.

<details>
<summary>Custom color settings for the Dark+ theme</summary>

```json
{
  "editor.tokenColorCustomizations": {
      "textMateRules": [
          {
              "scope": "variable.key.table",
              "settings": {
                  "foreground": "#4EC9B0",
              },
          },
          {
              "scope": "variable.key.array",
              "settings": {
                  "foreground": "#569CD6",
              }
          },
          {
              "scope": "constant.other.time",
              "settings": {
                  "foreground": "#DCDCAA",
              }
          }
      ]
  },
}
```
</details>

![Extended Color Highlighting](extended_colors.png)

### Semantic highlighting

Semantic key highlighting for inline tables and arrays can be enabled in the settings.

**You need to set extended colors in order for this to have any practical effect.**

![Semantic Highlighting](semantic_colors.png)

### Validation

![Validation](validation.gif)

### Folding

Arrays, multi-line strings and top level tables and comments can be folded.

![Folding](folding.gif)

### Symbol tree and navigation

Works even for tables not in order.

![Symbols](symbols.gif)

### Formatting

The formatter is rather conservative by default, additional features can be enabled in the settings. If you're missing a configuration option, feel free to open an issue about it!

![Formatting](formatting.gif)

### Commands

Some commands are always in the palette, these can be hidden in the settings.

#### Copy Selection as JSON

Copies the selected TOML text converted to JSON to the system clipboard.

#### Paste as JSON

Parses the TOML text from the system clipboard and pastes it converted into JSON in the active selection.

## Incomplete Features

These features are not yet complete and are disabled by default.
You can enable them, but expect them to either be full of bugs or very limited in features.

### [JSON schema](https://json-schema.org/) support

[*completion status*](https://github.com/tamasfe/taplo/issues?q=is%3Aopen+is%3Aissue+milestone%3A%22VSCode+Schemas%22)

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
The current focus of the extension is polishing (and completing) the current feature set.

You can see the future feature requests [here](https://github.com/tamasfe/taplo/issues?q=is%3Aissue+is%3Aopen+label%3Afeature).

### Configuration File

A configuration file for formatter and validation behaviour.

## Special Thanks

- To [@GalAster](https://github.com/GalAster) and [@be5invis](https://github.com/be5invis) for letting me use their TextMate grammar.
- To every contributor.
- And to everyone else using this extension.
