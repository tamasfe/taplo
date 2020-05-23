#[macro_use]
mod macros;

mod serde;

pub mod dom;
pub mod formatter;
pub mod parser;
pub mod syntax;
pub mod util;
pub mod value;

#[cfg(test)]
mod tests;
