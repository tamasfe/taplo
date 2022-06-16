# Conversion and Extraction

Taplo makes it easy to convert TOML files to JSON and also extract specific values via the `get` command.

## Examples

The following examples use the TOML file below:

```toml
# foo.toml
[foo]
bar = true

[[baz]]
baz_field = "baz 1"

[[baz]]
baz_field = "baz 2"
```

## Converting to JSON

```sh
taplo get -f foo.toml -o json
```

This will yield the following output:

```json
{
  "foo": {
    "bar": true
  },
  "baz": [
    {
      "baz_field": "baz 1"
    },
    {
      "baz_field": "baz 2"
    }
  ]
}
```

## Extracting Specific Values

It is possible to extract specific values via a simple query expressions.

```sh
taplo get -f foo.toml 'baz[1].baz_field'
```

This will yield:

```
baz 2
```

::: warning

Expressions with array patterns `[]` might need to be put between quotation marks for some shells.

:::

::: info

List indexing is 0-based.

:::

By default the value will be "parsed", so that no strings will contain escape sequences or quotation marks.

However it is also possible to change the output format to JSON:

```sh
taplo get -f foo.toml -o json baz
```

will yield:

```json
[
  {
    "baz_field": "baz 1"
  },
  {
    "baz_field": "baz 2"
  }
]
```

### More Examples

More examples for query expressions using the same document above:

|    expression     | output format |         output         |
| :---------------: | :-----------: | :--------------------: |
| `baz.0.baz_field` |     json      |       `"baz 1"`        |
| `baz.*.baz_field` |     json      | `[ "baz 1", "baz 2" ]` |
| `baz.1.baz_field` |     value     |        `baz 2`         |
|      `baz.1`      |     toml      | `baz_field = "baz 2"`  |


::: info

The outputs have been slightly modified to fit into the table format.

:::