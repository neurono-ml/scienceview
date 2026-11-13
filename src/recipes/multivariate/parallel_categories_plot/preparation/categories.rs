//! Counting one categorical axis through the `SVStat::Count` kernel.
//!
//! The recipe never counts categories itself: it hands the column to the
//! kernel with the requested order and reads the categories back. The labels
//! are a presentation concern layered on top of the counted values.

use super::super::configuration::SVParallelCategoriesConfiguration;
use super::super::options::SVParallelCategoriesMissingValuePolicy;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_stat::{SVCountOptions, SVStat, SVStatInput, SVStatResult};

/// Counts one axis' categories in the configured order.
pub(super) fn sv_count_axis(
    configuration: &SVParallelCategoriesConfiguration<'_>,
    view: SVColumnView<'_>,
) -> Result<crate::sv_stat::SVCountResult, SVError> {
    let options = SVCountOptions::new().order(configuration.category_order.to_count_order());
    match SVStat::count(options).apply(SVStatInput::new(view))? {
        SVStatResult::Categories(result) => Ok(result),
        other => Err(crate::sv_error::SVError::InvalidConfiguration {
            message: format!(
                "the parallel categories plot expected the counting stat to produce categories, but it produced `{}`",
                other.kind().as_str()
            ),
        }),
    }
}

/// Counts the optional colour column in first-appearance order.
pub(super) fn sv_count_colour(
    view: SVColumnView<'_>,
) -> Result<crate::sv_stat::SVCountResult, SVError> {
    let options = SVCountOptions::new();
    match SVStat::count(options).apply(SVStatInput::new(view))? {
        SVStatResult::Categories(result) => Ok(result),
        other => Err(crate::sv_error::SVError::InvalidConfiguration {
            message: format!(
                "the parallel categories plot expected the colour stat to produce categories, but it produced `{}`",
                other.kind().as_str()
            ),
        }),
    }
}

/// Builds the display labels of one axis, in its category order.
pub(super) fn sv_category_labels(
    configuration: &SVParallelCategoriesConfiguration<'_>,
    variable: &str,
    categories: &[f64],
) -> Vec<String> {
    let supplied = configuration
        .category_labels
        .iter()
        .find(|(name, _)| name == variable)
        .map(|(_, labels)| labels.as_slice())
        .unwrap_or(&[]);
    let labels: Vec<String> = categories
        .iter()
        .enumerate()
        .map(|(index, value)| {
            supplied
                .get(index)
                .cloned()
                .unwrap_or_else(|| sv_default_label(*value))
        })
        .collect();
    sv_unique_labels(labels)
}

fn sv_default_label(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        format!("{value}")
    }
}

/// Makes the labels unique without changing how they read.
///
/// A nominal scale identifies a slot by its category string, so two categories
/// that share a display label would resolve to the same slot. A zero-width
/// space keeps the rendered text identical while giving each slot a distinct
/// key.
fn sv_unique_labels(labels: Vec<String>) -> Vec<String> {
    let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    labels
        .into_iter()
        .map(|label| {
            let count = seen.entry(label.clone()).or_insert(0);
            let unique = if *count == 0 {
                label
            } else {
                format!("{label}{}", "\u{200B}".repeat(*count))
            };
            *count += 1;
            unique
        })
        .collect()
}

/// Maps a category value onto its exact storage key.
pub(super) fn sv_category_key(value: f64) -> u64 {
    if value == 0.0 { 0 } else { value.to_bits() }
}

/// Maps one observed value onto the axis rank it belongs to.
pub(super) fn sv_category_rank(
    ranks: &std::collections::HashMap<u64, usize>,
    value: f64,
) -> Result<usize, SVError> {
    ranks
        .get(&sv_category_key(value))
        .copied()
        .ok_or_else(|| crate::sv_error::SVError::InvalidConfiguration {
            message: format!(
                "the parallel categories plot met the category {value} that its counting axis did not report"
            ),
        })
}

/// Reports one missing observation according to the configured policy.
pub(super) fn sv_report_missing(
    policy: SVParallelCategoriesMissingValuePolicy,
    position: usize,
) -> Result<(), SVError> {
    match policy {
        SVParallelCategoriesMissingValuePolicy::Omit => Ok(()),
        SVParallelCategoriesMissingValuePolicy::Error => Err(SVError::NonFiniteValue {
            operation: "parallel_categories",
            position,
        }),
    }
}
