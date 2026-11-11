//! Failure paths of the figure surface. Every error is a value, never a panic.

mod error_definition;
#[cfg(test)]
mod error_tests;

pub use error_definition::SVError;
