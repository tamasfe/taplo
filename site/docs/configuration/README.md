---
sidebar: auto
---

# Configuration

## Configuration File

Taplo supports configuration via file, unsurprisingly it uses the TOML format.

By default, every tool looks for one in the working directory or the root of the workspace by the following names (in precedence order):

- `.taplo.toml`
- `taplo.toml`

### Include

The `include` property is an array of [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path strings that are relative to the working directory (or root of the workspace),
the matched files are included in the operations by the tools unless explicitly overwritten. The pattern supports globstars (`**`) for recursive search.

If this property is omitted, `TOML` files will be searched in the entry child directory tree, however if it is present but **empty**, **no files will be included**.

```toml
include = ["Cargo.toml", "some_directory/**/*.toml"]
```

### Exclude

The `exclude` property has the same semantics as `include` and takes precedence over it.

The following will exclude `Cargo.toml` from the includes written above, so files matching `some_directory/**/*.toml` will be included only.

```toml
exclude = ["Cargo.toml"]
```

### Formatting Options

The following options are used by the formatter, all of them are optional. 
The given example values are used by default.

```toml
[formatting]
# Align consecutive entries vertically.
align_entries = true

# Append trailing commas for multi-line arrays.
array_trailing_comma = true

# Expand arrays to multiple lines that exceed the maximum column width.
array_auto_expand = true

# Collapse arrays that don't exceed the maximum column width and don't contain comments.
array_auto_collapse = true

# Omit white space padding from single-line arrays
compact_arrays = true

# Omit white space padding from the start and end of inline tables.
compact_inline_tables = false

# Maximum column width in characters, affects array expansion and collapse, this doesn't take whitespace into account.
# Note that this is not set in stone, and works on a best-effort basis.
column_width = 80

# Indent based on tables and arrays of tables and their subtables, subtables out of order are not indented.
indent_tables = true

# The substring that is used for indentation, should be tabs or spaces (but technically can be anything).
indent_string = '  '

# Add trailing newline at the end of the file if not present.
trailing_newline = true

# Alphabetically reorder keys that are not separated by empty lines.
reorder_keys = true

# Maximum amount of allowed consecutive blank lines. This does not affect the whitespace at the end of the document, as it is always stripped.
allowed_blank_lines = 2

# Use CRLF for line endings.
crlf = false
```

### Schema

The `schema` table consists of only two keys:
- `path`: the path of the schema, this can be either path to a local file path or an URL with the schemes `taplo`, `http` or `https`. (`file` scheme is also accepted, it is the same as specifying a local path)
- `enabled`: whether to enable the schema or not (`true` if omitted).

An example:

```toml
[schema]
path = "./path/to/schema.json"
enabled = true
```

Or a configuration that uses a built-in schema:

```toml
[schema]
path = "taplo://cargo@Cargo.toml"
```
### Rules

The `rule` array of tables consist of rules that overwrite the above configuration based on some conditions.
Thus it has the same `formatting` and `schema` settings, and the `include` and `exclude` with the same semantics as their [global variants](#include), however this time they are used to determine whether the rule applies.

Additionally, `keys` of a rule is an array of dotted keys that scope the rule to a specific part within a single document.
The keys also support glob patterns.

*In case of overlapping rules, the last defined rule is used.*

Let's say we want to sort our `Cargo` dependencies, but nothing else, here is how we would do that:

```toml
[formatting]
reorder_keys = false

[[rule]]
include = ["**/Cargo.toml"]
keys = "dependencies"

[rule.formatting]
reorder_keys = true
```

## Schemas

The CLI and the VSCode extension supports additional completion, validation and other features based on [JSON schemas](https://json-schema.org/).

### Built-in Schemas

Several schemas are baked into the CLI tool and the VSCode extension, you can view them [here](https://github.com/tamasfe/taplo/tree/master/schemas).

#### Cargo

*URI: `taplo://cargo@Cargo.toml`*

[Cargo](https://doc.rust-lang.org/cargo/), the package manager and build tool for Rust uses TOML for configuration, most of `Cargo.toml` is covered with type-information and documentation.

#### Python

*URI: `taplo://python@pyproject.toml`*

The python configuration file `pyproject.toml` described by [PEP 621](https://www.python.org/dev/peps/pep-0621/) is also supported.

The following tool configurations are currently known by the schema:
- [Poetry](https://python-poetry.org/)

#### Taplo

*URI: `taplo://taplo@taplo.toml`*

For the configuration of the tool itself.

### Writing Custom Schemas

You can use your own schemas while working with Taplo. **Only JSON format is supported**, schemas written in YAML and other formats must be converted.

Submitting schemas is highly appreciated, to submit a schema, open an issue [on GitHub](https://github.com/tamasfe/taplo/issues).

#### Limitations and Unimplemented Features

Only a subset of JSON schema is supported for now with the following rules:

- No external references are allowed in a schema, meaning that a single `json` file must contain the entire schema with all of its subschemas.
- Parsing JSON pointers is not implemented, and they will not work.
- The only allowed references must point to subschemas in the `definitions` of the root of the schema.

These limitations are likely to be lifted in the future.

For more information about bugs and missing features check the [issues](https://github.com/tamasfe/taplo/labels/schema).

#### Schema Extension

Every schema object can have an extension with the key `evenBetterToml` that will be parsed by Taplo to provide additional hints and features.

All the properties are optional. Here's an example object that has all the properties of the schema extension:

```json5
{
    "type": "string",
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

The extension field can also appear alongside `$ref`s, in that case it is carried over to the referenced schema overriding its extension if there's any during parsing. This is useful if the same schema has different meanings in different contexts.

### Using Schemas

#### Visual Studio Code

Schemas can be associated to files in the `evenBetterToml.schema.associations` configuration object.

In order to associate files with their correct schemas a table of regex patterns and schema URIs is used.

- The key must be a regex pattern for an absolute document URI.
- The value can be either an absolute URI to the schema, or a path relative to the root of the workspace.

For example:

```json5
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
- `taplo`: a built-in schema bundled with the extension
- `http`, `https`: a remote schema that can be fetched with a `GET` request without authentication supported

#### Taplo CLI

The CLI supports the `-s` or `--schema` flag for linting, it also supports schemas in the [configuration file](#configuration-file). For more information, visit [the cli docs](/cli/#linting).
