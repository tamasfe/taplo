#![allow(clippy::single_match)]
//! # About
//!
//! The main purpose of the library is to provide tools for analyzing TOML data where the
//! layout must be preserved and the original position of every parsed token must be known. It can
//! also format TOML documents.
//!
//! It uses [Rowan](::rowan) for the syntax tree, and every character is preserved from the input,
//! including all comments and white space.
//!
//! A [DOM](dom) can be constructed for data-oriented analysis where each node wraps a part of the
//! syntax tree with additional information and functionality.
//!
//! # Features
//!
//! - **time**: Use [time](https://github.com/time-rs/time) for TOML dates and times
//!
//! - **serde**: Support for [serde](https://serde.rs) serialization of the DOM nodes.
//! - **schema**: Enable JSON-schema generation for formatter configuration.
//!
//! # Usage
//!
//! A TOML document has to be parsed with [parse](parser::parse) first, it
//! will build a syntax tree that can be traversed.
//!
//! If there were no syntax errors during parsing, then a [`dom::Node`]
//! can be constructed. It will build a DOM tree and validate the TOML document according
//! to the specification. A DOM tree can be constructed even with syntax errors present, however
//! parts of it might be missing.
//!
//! ```
//! use taplo::parser::parse;
//! const SOURCE: &str =
//! "value = 1
//! value = 2
//!
//! [table]
//! string = 'some string'";
//!
//! let parse_result = parse(SOURCE);
//!
//! // Check for syntax errors.
//! // These are not carried over to DOM errors.
//! assert!(parse_result.errors.is_empty());
//!
//! let root_node = parse_result.into_dom();
//!
//! // Check for semantic errors.
//! // In this example "value" is a duplicate key.
//! assert!(root_node.validate().is_err());
//! ```

pub mod dom;
pub mod formatter;
pub mod parser;
pub mod syntax;
pub mod util;

pub use rowan;

pub type HashMap<K, V> = ahash::AHashMap<K, V>;
pub type HashSet<V> = ahash::AHashSet<V>;

#[cfg(test)]
mod tests;

mod private {
    pub trait Sealed {}
}
