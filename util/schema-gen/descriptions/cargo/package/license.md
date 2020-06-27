The `license` field contains the name of the software license that the package
is released under.

[crates.io](https://crates.io/) interprets the `license` field as an [SPDX 2.1 license
expression](https://spdx.org/spdx-specification-21-web-version#h.jxpfx0ykyb60). The name must be a known license
from the [SPDX license list 3.6](https://github.com/spdx/license-list-data/tree/v3.6). Parentheses are not
currently supported. See the [SPDX site](https://spdx.org/license-list) for more information.

SPDX license expressions support AND and OR operators to combine multiple
licenses.

```toml
[package]
# ...
license = "MIT OR Apache-2.0"
```

Using `OR` indicates the user may choose either license. Using `AND` indicates
the user must comply with both licenses simultaneously. The `WITH` operator
indicates a license with a special exception. Some examples:

* `MIT OR Apache-2.0`
* `LGPL-2.1 AND MIT AND BSD-2-Clause`
* `GPL-2.0+ WITH Bison-exception-2.2`

If a package is using a nonstandard license, then the `license-file` field may
be specified in lieu of the `license` field.