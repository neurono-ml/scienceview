//! The figure model: the builder state machine, the built figure and the data
//! view it carries.

mod builder;
mod builder_state;
mod core_implementation;
mod execution_mode;

pub use builder::SVFigureBuilder;
pub use builder_state::SVWithBackend;
pub use builder_state::SVWithDataSource;
pub use builder_state::SVWithoutBackend;
pub use builder_state::SVWithoutDataSource;
pub use core_implementation::SVFigure;
pub use execution_mode::SVExecutionMode;

#[cfg(test)]
mod figure_builder_tests;
#[cfg(test)]
mod figure_concurrency_tests;
#[cfg(test)]
mod figure_operation_tests;
