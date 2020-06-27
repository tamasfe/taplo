The optional `default-members` key can be specified to set the members to
operate on when in the workspace root and the package selection flags are not
used:

```toml
[workspace]
members = ["path/to/member1", "path/to/member2", "path/to/member3/*"]
default-members = ["path/to/member2", "path/to/member3/foo"]
```

When specified, `default-members` must expand to a subset of `members`.