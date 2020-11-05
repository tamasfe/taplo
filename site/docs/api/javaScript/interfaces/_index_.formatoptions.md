**[@taplo/lib](../README.md)**

> [Globals](../globals.md) / ["index"](../modules/_index_.md) / FormatOptions

# Interface: FormatOptions

Options for the format function.

## Hierarchy

* **FormatOptions**

## Index

### Properties

* [ignoreErrors](_index_.formatoptions.md#ignoreerrors)
* [options](_index_.formatoptions.md#options)

## Properties

### ignoreErrors

• `Optional` **ignoreErrors**: boolean

*Defined in [index.ts:85](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L85)*

Ignore syntax errors, and format anyway.

Setting this can be potentially destructive,
if the TOML document is invalid.

___

### options

• `Optional` **options**: [FormatterOptions](_index_.formatteroptions.md)

*Defined in [index.ts:89](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L89)*

Options to pass to the formatter. (https://tamasfe.dev/taplo/configuration/#formatting-options)
