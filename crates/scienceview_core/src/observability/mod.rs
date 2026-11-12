//! One span per public operation, carrying the configured backend kind, the
//! data representation, the number of drawn marks and the elapsed time.
//!
//! The span shape is fixed here once, so that a new operation cannot invent
//! its own fields.

mod operation_span;
#[cfg(test)]
mod operation_span_subscriber_tests;
#[cfg(test)]
mod operation_span_tests;
#[cfg(test)]
pub(crate) mod span_test_support;
mod stat_span;

pub use operation_span::SV_OPERATION_SPAN_NAME;
pub use operation_span::sv_run_operation;
pub use operation_span::sv_start_operation_span;
pub use stat_span::SV_STAT_SPAN_NAME;
pub use stat_span::sv_run_stat_operation;
pub use stat_span::sv_start_stat_span;
