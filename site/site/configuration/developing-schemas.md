# Developing Schemas

## Writing Schemas

All features from the [Draft 4](https://json-schema.org/specification-links.html#draft-4) specification are supported, the schemas may contain external and even recursive references as well.

All schemas must be in JSON format, if you use YAML for writing your schemas, they must be converted in order to be used by Taplo.

### Schema Extension

All schema objects might contain an `x-taplo` extension field that allows attaching additional information to the schema.

::: warning

The `x-taplo` field (and any other fields) are ignored if `$ref` is present in an object.

:::

The example below contains all the currently supported extension fields:

```jsonc
{
  "type": "string",
  "title": "My Type",
  "enum": ["one", "two", "three"],
  "default": "one",
  // ...
  // All the keys in the extension are optional.
  "x-taplo": {
    // Hide the schema from completion and similar hints.
    "hidden": true,
    "docs": {
      // Main documentation for the schema, it is expected to be markdown.
      // If this is omitted, the description will be used.
      "main": "This is [My Schema](https://example.com/mySchema)",
      // Documentation of the enum values, these are used when selecting
      // the values completion or hovering over them.
      //
      // These are selected by matching the indices of the enum values.
      // If a value doesn't have documentation in the middle of the array,
      // null can be used instead of a string.
      "enumValues": [
        "Documentation of 'one'.",
        null,
        "Documentation of 'three'."
      ],
      // The documentation of the default value, same as the enum docs.
      "defaultValue": "Documentation of 'one'."
    },
    "links": {
      // An URL the key will point to if the schema is part of a table.
      "key": "https://example.com/mySchema",
      // Different enum values can also have URLs they will point to.
      // The rules are the same as enum docs.
      "enumValues": ["https://example.com/one", "https://example.com/two"]
    },
    // If the schema is an object, we can hint what
    // fields are typically important.
    //
    // These will be created automatically along with
    // the required properties during autocompletion.
    "initKeys": ["importantKey"]
  }
}
```

## Publishing

Submitting schemas directly to Taplo is not possible anymore, and all JSON schemas should be submitted to the [JSON Schema Store](https://www.schemastore.org/json/).

In earlier versions, schemas had several limitations and had to be specifically written for Taplo. For this reason a separate catalog was used that is still available [here](https://taplo.tamasfe.dev/schema_index.json).

### Visual Studio Code extensions

Similarly to [`jsonValidation`](https://code.visualstudio.com/api/references/contribution-points#contributes.jsonValidation), it is possible for extensions to contribute their own schemas.

Other than `fileMatch`, it is also possible to specify `regexMatch` that is matched against the entire document URI.

```json
{
  "contributes": {
    "tomlValidation": [
      {
        "regexMatch": "^.*foo.toml$",
        "url": "https://json.schemastore.org/foo.json"
      }
    ]
  }
}
```
