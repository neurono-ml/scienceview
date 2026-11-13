//! Tests for `SVStepPlot`, split by concern.

mod concurrency_tests;
mod data_contract_tests;
mod observability_tests;
mod option_tests;
mod parity_tests;
mod resolution_tests;
#[cfg(feature = "render-svg")]
mod svg_tests;
mod test_sources;
