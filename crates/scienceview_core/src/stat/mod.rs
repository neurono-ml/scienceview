//! Statistical transforms: the derived values a chart draws (FDR-002).
//!
//! `SVStat` turns observations into the values a mark consumes — bin edges and
//! counts, category counts, group aggregates, density samples, cumulative
//! probabilities, percentiles and fitted curves. It is backend-independent by
//! construction: it produces data space only, never geometry, so the scale
//! kernel and the marks stay the only place a value becomes a visual property.

mod bin;
mod bin_options;
mod bin_result;
mod count;
mod count_options;
mod count_result;
mod input;
mod numeric;
mod observations;
mod stat_definition;

pub use bin_options::{SVBinOptions, SVNormalisation};
pub use bin_result::SVBinResult;
pub use count_options::{SVCountOptions, SVCountOrder};
pub use count_result::SVCountResult;
pub use input::SVStatInput;
pub use stat_definition::{SVStat, SVStatKind, SVStatResult};

#[cfg(test)]
#[path = "bin_tests.rs"]
mod bin_tests;
#[cfg(test)]
#[path = "count_tests.rs"]
mod count_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
