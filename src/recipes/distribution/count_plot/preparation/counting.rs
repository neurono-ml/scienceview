//! Counting through the `SVStat::Count` kernel and aligning the results.
//!
//! The recipe never counts observations itself: it hands each series' category
//! column to the kernel with the requested order and reads the categories and
//! counts back. It then aligns the category sets the kernel returns on one
//! shared axis, so stacking and dodging can match them by index. The only
//! arithmetic added here is the normalisation the count kernel does not carry.

use std::collections::HashSet;

use super::super::configuration::{SVCountConfiguration, sv_invalid};
use super::observations::{SVCategoryGroup, sv_category_key};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_scale::SVColour;
use crate::sv_stat::{
    SVCountOptions, SVCountOrder, SVNormalisation, SVStat, SVStatInput, SVStatResult,
};

/// One series after counting, in data space, aligned to the shared axis.
pub(super) struct SVCountedSeries {
    pub(super) label: String,
    pub(super) colour: SVColour,
    pub(super) counts: Vec<f64>,
}

/// Builds the shared category axis in the requested order.
///
/// `DataOrder` keeps the order the categories first appear in the whole
/// column, so the axis does not depend on how the series were split.
pub(super) fn sv_category_axis(
    configuration: &SVCountConfiguration<'_>,
    category: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
) -> Result<Vec<f64>, SVError> {
    let number_of_points = category.length();
    let mut seen: HashSet<u64> = HashSet::new();
    let mut categories: Vec<f64> = Vec::new();
    for index in 0..number_of_points {
        let Some(value) = category.value_at(index) else {
            continue;
        };
        if !value.is_finite() {
            continue;
        }
        if let Some(series) = series {
            match series.value_at(index) {
                Some(series_value) if series_value.is_finite() => {}
                _ => continue,
            }
        }
        if seen.insert(sv_category_key(value)) {
            categories.push(value);
        }
    }
    match configuration.order {
        SVCountOrder::DataOrder => {}
        SVCountOrder::Ascending => categories.sort_by(f64::total_cmp),
        SVCountOrder::Descending => {
            categories.sort_by(|left, right| right.total_cmp(left));
        }
    }
    if categories.is_empty() {
        return Err(SVError::EmptyInput { operation: "count" });
    }
    Ok(categories)
}

/// Counts every series through the kernel and aligns it on the shared axis.
pub(super) fn sv_count_groups(
    configuration: &SVCountConfiguration<'_>,
    groups: &[SVCategoryGroup<'_>],
    categories: &[f64],
) -> Result<Vec<SVCountedSeries>, SVError> {
    let positions: Vec<(u64, usize)> = categories
        .iter()
        .enumerate()
        .map(|(index, value)| (sv_category_key(*value), index))
        .collect();

    let mut counted: Vec<SVCountedSeries> = Vec::with_capacity(groups.len());
    let mut grand_total = 0.0f64;
    for group in groups {
        if group.storage.length() == 0 {
            continue;
        }
        let result = sv_apply_count(configuration, group)?;
        let mut counts = vec![0.0f64; categories.len()];
        for (category, count) in result.categories().iter().zip(result.counts().iter()) {
            let key = sv_category_key(*category);
            let Some((_, position)) = positions.iter().find(|(candidate, _)| *candidate == key)
            else {
                return Err(sv_invalid(
                    "the counting kernel returned a category that is not part of the shared axis",
                ));
            };
            if let Some(slot) = counts.get_mut(*position) {
                *slot = *count as f64;
            }
        }
        grand_total += counts.iter().sum::<f64>();
        counted.push(SVCountedSeries {
            label: group.label.clone(),
            colour: group.colour,
            counts,
        });
    }
    if counted.is_empty() {
        return Err(SVError::EmptyInput { operation: "count" });
    }

    let factor = match configuration.normalisation {
        SVNormalisation::Count => 1.0,
        SVNormalisation::Probability | SVNormalisation::Density => {
            if grand_total > 0.0 {
                1.0 / grand_total
            } else {
                0.0
            }
        }
    };
    for series in &mut counted {
        series.counts.iter_mut().for_each(|count| *count *= factor);
    }
    Ok(counted)
}

fn sv_apply_count(
    configuration: &SVCountConfiguration<'_>,
    group: &SVCategoryGroup<'_>,
) -> Result<crate::sv_stat::SVCountResult, SVError> {
    let view = group.storage.view()?;
    let options = SVCountOptions::new().order(configuration.order);
    match SVStat::count(options).apply(SVStatInput::new(view))? {
        SVStatResult::Categories(result) => Ok(result),
        other => Err(sv_invalid(&format!(
            "the count plot expected the counting stat to produce categories, but it produced `{}`",
            other.kind().as_str()
        ))),
    }
}
