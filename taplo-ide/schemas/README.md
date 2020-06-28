# Schemas

Completion, validation and other features are supported based on [JSON schemas](https://json-schema.org/).

- [Schemas](#schemas)
  - [Schema-related Configuration](#schema-related-configuration)
    - [Visual Studio Code](#visual-studio-code)
      - [Associations](#associations)
  - [Custom Schemas](#custom-schemas)
    - [Limitations and Unimplemented Features](#limitations-and-unimplemented-features)
    - [Additional Features](#additional-features)


## Schema-related Configuration

### Visual Studio Code

#### Associations

In order to associate files with their correct schemas a table of regex patterns and schema URIs is used.

The key must be a regex pattern for an absolute document URI.

The value can be either an absolute URI to the schema, or a path relative to the root of the workspace.

For example:

```javascript
{
    // The following will match all documents ending with "test.toml", and use the "test.json" schema
    // in the root of the workspace.
    //
    // The relative path must start with "./" and it cannot be omitted.
    ".*test\\.toml": "./test.json",

    // The following will only match a single specific file with a specific schema:
    "file:///path/to/the/file.toml": "file:///path/to/the/schema.json"
}
```

Currently supported schemes for the schema URIs:
- `file`: a local file
- `toml_builtin`: a built-in schema bundled with the extension

In the future further schemes will be supported (such as `http`).

## Custom Schemas

**Only JSON is supported**, schemas written in YAML and other formats must be converted.
Examples of currently working schemas are found in this directory.

### Limitations and Unimplemented Features

Only a subset of JSON schema is supported for now with the following rules:

- No external references are allowed in a schema, meaning that a single `json` file must contain the entire schema with all of its subschemas.
- Parsing JSON pointers is not implemented, and they will not work.
- The only allowed references must point to subschemas in the `definitions` of the root of the schema.

For more information about bugs and missing features check the [issues](https://github.com/tamasfe/taplo/labels/schema).

### Additional Features

Every schema can have an extension with the key `evenBetterToml` that will be parsed by Taplo to provide additional hints and features:

```javascript
{
    "type": "object",
    "title": "My Schema",
    "enum": ["one", "two", "three"],
    "default": "one",
    // ...
    // All the keys in the extension are optional.
    "evenBetterToml": { 
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
            "enumValues": [                 
                "https://example.com/one",
                "https://example.com/two",
            ]
        }
    } 
}
```

The extension field can also appear alongside `$ref`s, in that case it is carried over to the referenced schema overriding its extension if there's any during parsing. This is useful if the same schema has different meanings in different contexts.
