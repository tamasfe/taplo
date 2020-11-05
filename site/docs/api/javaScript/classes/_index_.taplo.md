**[@taplo/lib](../README.md)**

> [Globals](../globals.md) / ["index"](../modules/_index_.md) / Taplo

# Class: Taplo

This class allows for usage of the library in a synchronous context
after being asynchronously initialized once.

It cannot be constructed with `new`, and instead must be
created by calling `initialize`.

Example usage:

```js
import { Taplo } from "taplo";

// Somewhere at the start of your app.
const taplo = await Taplo.initialize();
// ...
// The other methods will not return promises.
const formatted = taplo.format(tomlDocument);
```

## Hierarchy

* **Taplo**

## Index

### Constructors

* [constructor](_index_.taplo.md#constructor)

### Properties

* [initializing](_index_.taplo.md#initializing)
* [taplo](_index_.taplo.md#taplo)

### Methods

* [format](_index_.taplo.md#format)
* [lint](_index_.taplo.md#lint)
* [initialize](_index_.taplo.md#initialize)

## Constructors

### constructor

\+ `Private`**new Taplo**(): [Taplo](_index_.taplo.md)

*Defined in [index.ts:173](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L173)*

**Returns:** [Taplo](_index_.taplo.md)

## Properties

### initializing

▪ `Static` `Private` **initializing**: boolean = false

*Defined in [index.ts:173](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L173)*

___

### taplo

▪ `Static` `Private` **taplo**: any \| undefined

*Defined in [index.ts:172](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L172)*

## Methods

### format

▸ **format**(`toml`: string, `options?`: [FormatOptions](../interfaces/_index_.formatoptions.md)): Promise\<String>

*Defined in [index.ts:231](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L231)*

Format the given TOML document.

**`throws`** Throws if the document contains syntax errors and the `ignoreErrors` option is false.

#### Parameters:

Name | Type | Description |
------ | ------ | ------ |
`toml` | string | TOML document. |
`options?` | [FormatOptions](../interfaces/_index_.formatoptions.md) | Optional format options.  |

**Returns:** Promise\<String>

___

### lint

▸ **lint**(`toml`: string, `options?`: [LintOptions](../interfaces/_index_.lintoptions.md)): Promise\<[LintResult](../interfaces/_index_.lintresult.md)>

*Defined in [index.ts:207](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L207)*

Lint a TOML document, this function returns
both syntax and semantic (e.g. conflicting keys) errors.

If a JSON schema is given, the TOML document will be validated with it
only if it is syntactically valid.

**`throws`** If the given JSON schema is invalid.

#### Parameters:

Name | Type | Description |
------ | ------ | ------ |
`toml` | string | TOML document. |
`options?` | [LintOptions](../interfaces/_index_.lintoptions.md) | Optional additional options.  |

**Returns:** Promise\<[LintResult](../interfaces/_index_.lintresult.md)>

___

### initialize

▸ `Static`**initialize**(): Promise\<[Taplo](_index_.taplo.md)>

*Defined in [index.ts:183](https://github.com/tamasfe/taplo/blob/b68fd29/node/lib/src/index.ts#L183)*

**Returns:** Promise\<[Taplo](_index_.taplo.md)>
