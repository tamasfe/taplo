**[@taplo/lib](../README.md)**

> [Globals](../globals.md) / ["index"](../modules/_index_.md) / LintOptions

# Interface: LintOptions

Options for the lint function.

## Hierarchy

* **LintOptions**

## Index

### Properties

* [schema](_index_.lintoptions.md#schema)
* [schemaRanges](_index_.lintoptions.md#schemaranges)

## Properties

### schema

• `Optional` **schema**: string \| any

*Defined in [index.ts:140](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L140)*

Optional JSON schema for validation, can be a JSON string or an object.

___

### schemaRanges

• `Optional` **schemaRanges**: boolean

*Defined in [index.ts:149](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L149)*

By default validation errors based on JSON Schema return a human-readable
path to the invalid part in the error message.

If this property is `true`, the byte range will be returned instead in the error,
just like for syntax errors.
