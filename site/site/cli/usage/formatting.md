# Formatting

It is possible to format files in-place or via standard i/o.

```sh
taplo fmt foo.toml
```

Or

```sh
cat foo.toml | taplo fmt -
```

::: info

By default Taplo will bail on documents that contain syntax errors to avoid destructive edits, you can use the `--force` flag to suppress this and try to format the invalid document(s) anyway.

:::

## Options

Formatter options are read from the [configuration file](./configuration.md#configuration-file), on top of that it is possible to specify overrides via the `--option` flag:

```sh
taplo fmt --option indent_tables=false foo.toml
```

## Check

It is possible to check whether the given files are properly formatted via the `--check` flag. When this flag is supplied, no formatting will be done.
