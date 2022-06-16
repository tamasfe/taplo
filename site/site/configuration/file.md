# Configuration File

Taplo supports configuration via file, unsurprisingly it uses the TOML format.

By default, every tool looks for one in the working directory or the root of the workspace by the following names (in precedence order):

- `.taplo.toml`
- `taplo.toml`

## Include

The `include` property is an array of [glob](<https://en.wikipedia.org/wiki/Glob_(programming)>) path strings that are relative to the working directory (or root of the workspace),
the matched files are included in the operations by the tools unless explicitly overwritten. The pattern supports globstars (`**`) for recursive search.

If this property is omitted, `TOML` files will be searched in the entire child directory tree from the root.

::: warning

If `include` is present but empty, **no files will be included**.

:::

```toml
include = ["Cargo.toml", "some_directory/**/*.toml"]
```

## Exclude

The `exclude` property has the same semantics as `include` and takes precedence over it.

The following will exclude `Cargo.toml` from the includes written above, so files matching `some_directory/**/*.toml` will be included only.

```toml
exclude = ["Cargo.toml"]
```

## Formatting Options

The `formatting` table contains optional [formatting options](./formatter-options.md) for the formatter:

```toml
[formatting]
align_entries = false
```

## Schema

The `schema` table consists of only two keys:

- `path`: the path of the schema, this can be either path to a local file or an URL with the schemes `taplo`, `http` or `https`. (`file` scheme is also accepted, it is the same as specifying a local path)
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
path = "taplo://taplo.toml"
```

Or a remote one:

```toml
[schema]
path = "https://example.com/my_schema.json"
```

## Rules

The `rule` array of tables consist of rules that overwrite the above configuration based on some conditions.
Thus it has the same `formatting` and `schema` settings, and the `include` and `exclude` with the same semantics as their [global variants](#include), however this time they are used to determine whether the rule applies.

Additionally, `keys` of a rule is an array of dotted keys that scope the rule to a specific part within a single document.
The keys also support glob patterns.

::: tip

In case of overlapping rules, the last defined rule always takes precedence.

:::

Let's say we want to sort our `Cargo` dependencies, but nothing else, here is how we would do that:

```toml
[formatting]
reorder_keys = false

[[rule]]
include = ["**/Cargo.toml"]
keys = ["dependencies"]

[rule.formatting]
reorder_keys = true
```
