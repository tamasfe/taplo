#![allow(clippy::single_match)]
/*!

# About

The main purpose of the library is to provide tools for analyzing TOML data where the
layout must be preserved and the original position of every parsed token must be known. It can
also format TOML documents.

It uses [Rowan](::rowan) for the syntax tree, and every character is preserved from the input,
including all comments and white space.

A [DOM](dom) can be constructed for data-oriented analysis where each node wraps a part of the
syntax tree with additional information and functionality.

Taplo also exposes a [Value](value::Value) type that can be created from [DOM](dom) nodes
and can be serialized with [Serde](serde) allowing for conversion to JSON, YAML or any other format.

TOML serialization is currently not implemented, but it is planned with a low priority.

# Usage

A TOML document has to be parsed with [parse](parser::parse) first, it
will build a syntax tree that can be traversed.

If there were no syntax errors during parsing, then a [DOM root node](dom::RootNode)
can be constructed. It will build a DOM tree and validate the TOML document according
to the specification. A DOM tree can be constructed even with syntax errors present, however
parts of it will be missing.

If any errors occurred, they will be collected in the root node. In that case
the DOM must be analyzed with caution as parts of it might be missing.

```edition2018
# use taplo::parser::parse;
const SOURCE: &str =
"value = 1
value = 2

[table]
string = 'some string'";

let parse_result = parse(SOURCE);

// Check for syntax errors.
// These are not carried over to DOM errors.
assert_eq!(parse_result.errors.len(), 0);

let root_node = parse_result.into_dom();

// Check for semantic errors.
// In this example "value" is a duplicate key.
assert_eq!(root_node.errors().len(), 1);
```
*/

// TODO: time impls for value
// #[cfg(all(feature = "time", feature = "chrono"))]
// compile_error!("time and chrono features are mutually exclusive");

// #[cfg(not(any(feature = "time", feature = "chrono")))]
// compile_error!("either time or chrono feature is required");

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "verify")]
pub mod verify;

pub mod util;
pub mod dom;
pub mod formatter;
pub mod parser;
pub mod syntax;
pub mod value;
pub mod analytics;

#[cfg(test)]
mod tests;
