**[@taplo/lib](../README.md)**

> [Globals](../globals.md) / ["index"](../modules/_index_.md) / FormatterOptions

# Interface: FormatterOptions

Taplo formatter options. (https://tamasfe.dev/taplo/configuration/#formatting-options)

## Hierarchy

* **FormatterOptions**

## Index

### Properties

* [alignEntries](_index_.formatteroptions.md#alignentries)
* [allowed\_blank\_lines](_index_.formatteroptions.md#allowed_blank_lines)
* [array\_auto\_collapse](_index_.formatteroptions.md#array_auto_collapse)
* [array\_auto\_expand](_index_.formatteroptions.md#array_auto_expand)
* [array\_trailing\_comma](_index_.formatteroptions.md#array_trailing_comma)
* [column\_width](_index_.formatteroptions.md#column_width)
* [compact\_arrays](_index_.formatteroptions.md#compact_arrays)
* [compact\_inline\_tables](_index_.formatteroptions.md#compact_inline_tables)
* [crlf](_index_.formatteroptions.md#crlf)
* [indent\_string](_index_.formatteroptions.md#indent_string)
* [indent\_tables](_index_.formatteroptions.md#indent_tables)
* [reorder\_keys](_index_.formatteroptions.md#reorder_keys)
* [trailing\_newline](_index_.formatteroptions.md#trailing_newline)

## Properties

### alignEntries

• `Optional` **alignEntries**: boolean

*Defined in [index.ts:11](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L11)*

Align consecutive entries vertically.

___

### allowed\_blank\_lines

• `Optional` **allowed\_blank\_lines**: number

*Defined in [index.ts:67](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L67)*

Maximum amount of allowed consecutive blank lines. This does not affect the whitespace at the end of the document, as it is always stripped.

___

### array\_auto\_collapse

• `Optional` **array\_auto\_collapse**: boolean

*Defined in [index.ts:26](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L26)*

Collapse arrays that don't exceed the maximum column width and don't contain comments.

___

### array\_auto\_expand

• `Optional` **array\_auto\_expand**: boolean

*Defined in [index.ts:21](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L21)*

Expand arrays to multiple lines that exceed the maximum column width.

___

### array\_trailing\_comma

• `Optional` **array\_trailing\_comma**: boolean

*Defined in [index.ts:16](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L16)*

Append trailing commas for multi-line arrays.

___

### column\_width

• `Optional` **column\_width**: number

*Defined in [index.ts:42](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L42)*

Maximum column width in characters, affects array expansion and collapse, this doesn't take whitespace into account.
Note that this is not set in stone, and works on a best-effort basis.

___

### compact\_arrays

• `Optional` **compact\_arrays**: boolean

*Defined in [index.ts:31](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L31)*

Omit white space padding from single-line arrays

___

### compact\_inline\_tables

• `Optional` **compact\_inline\_tables**: boolean

*Defined in [index.ts:36](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L36)*

Omit white space padding from the start and end of inline tables.

___

### crlf

• `Optional` **crlf**: boolean

*Defined in [index.ts:72](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L72)*

Use CRLF for line endings.

___

### indent\_string

• `Optional` **indent\_string**: string

*Defined in [index.ts:52](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L52)*

The substring that is used for indentation, should be tabs or spaces (but technically can be anything).

___

### indent\_tables

• `Optional` **indent\_tables**: boolean

*Defined in [index.ts:47](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L47)*

Indent based on tables and arrays of tables and their subtables, subtables out of order are not indented.

___

### reorder\_keys

• `Optional` **reorder\_keys**: boolean

*Defined in [index.ts:62](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L62)*

Alphabetically reorder keys that are not separated by empty lines.

___

### trailing\_newline

• `Optional` **trailing\_newline**: boolean

*Defined in [index.ts:57](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L57)*

Add trailing newline at the end of the file if not present.
