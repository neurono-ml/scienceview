//! The figure model: the builder state machine, the built figure and the data
//! binding it carries.

mod data_binding;
mod execution_mode;
#[cfg(test)]
mod figure_builder_tests;
#[cfg(test)]
mod figure_operation_tests;

pub use data_binding::SVDataRepresentation;
pub use execution_mode::SVExecutionMode;
