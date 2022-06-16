# Directives

The behaviour of Taplo can be further customized by comments in TOML files called `directives`.

All directive comments must follow the following pattern: `#:<name> <content>`.

A `header` directive means that it is at the beginning of the document and can only be preceded by other directives or comments.

## The `schema` Directive

It is possible to override the schema for a specific document by using the `schema` header directive. A relative file path or an URL can be provided.

Example:

```toml
#:schema ./foo-schema.json
foo = "bar"
```

::: tip

Relative paths are relative to the document file, if the file path is not known, Taplo will be unable to find the schema.

:::

::: warning

Multiple schema directives in the same document are not supported and the behaviour is undefined.

:::
