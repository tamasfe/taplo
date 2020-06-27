The `categories` field is an array of strings of the categories this package
belongs to.

```toml
categories = ["command-line-utilities", "development-tools::cargo-plugins"]
```

> **Note**: [crates.io](https://crates.io) has a maximum of 5 categories. Each category should
> match one of the strings available at https://crates.io/category_slugs, and
> must match exactly.