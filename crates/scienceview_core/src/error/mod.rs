//! Failure paths of the figure surface. Every error is a value, never a panic.

#[cfg(test)]
mod error_tests;
mod error_definition;

pub use error_definition::SVError;
