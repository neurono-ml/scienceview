//! Failure paths of the figure surface. Every error is a value, never a panic.

mod error_tests;
mod sv_error;

pub use sv_error::SVError;
