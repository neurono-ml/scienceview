//! Statistical transforms: the derived values a chart draws (FDR-002).
//!
//! `SVStat` turns observations into the values a mark consumes — bin edges and
//! counts, category counts, group aggregates, density samples, cumulative
//! probabilities, percentiles and fitted curves. It is backend-independent by
//! construction: it produces data space only, never geometry, so the scale
//! kernel and the marks stay the only place a value becomes a visual property.
//!
//! # Example
//!
//! ```
//! use scienceview_core::sv_data::SVColumnView;
//! use scienceview_core::sv_stat::{
//!     SVBinOptions, SVNormalisation, SVStat, SVStatInput, SVStatResult,
//! };
//!
//! let values = [1.0, 2.0, 2.0, 3.0, 4.0];
//! let stat = SVStat::bin(
//!     SVBinOptions::new()
//!         .bin_count(2)
//!         .normalise(SVNormalisation::Probability),
//! );
//!
//! match stat.apply(SVStatInput::new(SVColumnView::Dense(&values))) {
//!     Ok(SVStatResult::Bins(bins)) => {
//!         assert_eq!(bins.edges(), &[1.0, 2.5, 4.0]);
//!         assert_eq!(bins.counts(), &[0.6, 0.4]);
//!     }
//!     other => panic!("the bin stat produces bins: {other:?}"),
//! }
//! ```

mod aggregate;
mod aggregate_options;
mod aggregate_result;
mod bin;
mod bin_options;
mod bin_result;
mod count;
mod count_options;
mod count_result;
mod density;
mod density_options;
mod density_result;
mod empirical_cdf;
mod empirical_cdf_options;
mod empirical_cdf_result;
mod estimator;
mod fit;
mod fit_options;
mod fit_result;
mod input;
mod numeric;
mod observations;
mod percentile;
mod percentile_options;
mod percentile_result;
mod sorted_observations;
mod stat_definition;

pub use aggregate_options::{SVAggregateErrorKind, SVAggregateMethod, SVAggregateOptions};
pub use aggregate_result::{SVAggregateErrorInterval, SVAggregateGroup, SVAggregateResult};
pub use bin_options::{SVBinOptions, SVNormalisation};
pub use bin_result::SVBinResult;
pub use count_options::{SVCountOptions, SVCountOrder};
pub use count_result::SVCountResult;
pub use density_options::{SVKernelDensityOptions, SVKernelShape};
pub use density_result::SVKernelDensityResult;
pub use empirical_cdf_options::SVEmpiricalCdfOptions;
pub use empirical_cdf_result::SVEmpiricalCdfResult;
pub use estimator::{SVFitCurve, SVFitEstimator, sv_fit_with_estimator};
pub use fit_options::{SVFitModel, SVFitOptions};
pub use fit_result::SVFitResult;
pub use input::SVStatInput;
pub use percentile_options::SVPercentileOptions;
pub use percentile_result::SVPercentileResult;
pub use stat_definition::{SVStat, SVStatKind, SVStatResult};

#[cfg(test)]
#[path = "aggregate_tests.rs"]
mod aggregate_tests;
#[cfg(test)]
#[path = "bin_tests.rs"]
mod bin_tests;
#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "count_tests.rs"]
mod count_tests;
#[cfg(test)]
#[path = "density_tests.rs"]
mod density_tests;
#[cfg(test)]
#[path = "empirical_cdf_tests.rs"]
mod empirical_cdf_tests;
#[cfg(test)]
#[path = "estimator_tests.rs"]
mod estimator_tests;
#[cfg(test)]
#[path = "fit_tests.rs"]
mod fit_tests;
#[cfg(test)]
#[path = "input_tests.rs"]
mod input_tests;
#[cfg(test)]
#[path = "numeric_tests.rs"]
mod numeric_tests;
#[cfg(test)]
#[path = "observations_tests.rs"]
mod observations_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "percentile_tests.rs"]
mod percentile_tests;
#[cfg(test)]
#[path = "sorted_observations_tests.rs"]
mod sorted_observations_tests;
#[cfg(test)]
#[path = "stat_span_tests.rs"]
mod stat_span_tests;
