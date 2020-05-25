// Uncomment this to run benchmarks
// #![feature(test)]

#[macro_use]
mod macros;

#[cfg(feature = "serialize")]
mod serde;

pub mod dom;
pub mod formatter;
pub mod parser;
pub mod syntax;
pub mod util;
pub mod value;

#[cfg(test)]
mod tests;
