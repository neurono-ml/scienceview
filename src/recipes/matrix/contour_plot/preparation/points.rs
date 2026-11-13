//! Binning a point cloud onto a regular lattice.

use super::super::configuration::SVContourConfiguration;
use super::super::field::SVContourField;
use super::super::options::SVContourMissingPolicy;
use super::missing::sv_fill_missing;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_stat::{
    SVAggregateMethod, SVAggregateOptions, SVBinOptions, SVStat, SVStatInput, SVStatResult,
};

/// One finite `(x, y, value)` observation.
struct SVPointObservation {
    x: f64,
    y: f64,
    value: f64,
}

/// Builds a lattice from equal-length `x`, `y` and `value` observations.
///
/// The two axes are binned through [`SVStat::Bin`], so the recipe owns no
/// binning rule of its own. Every cell holds the mean of the values that fall
/// in it, computed by [`SVStat::Aggregate`]; an empty cell is a missing sample
/// and follows the configured policy.
pub(super) fn sv_points_field(
    configuration: &SVContourConfiguration<'_>,
    x: SVColumnView<'_>,
    y: SVColumnView<'_>,
    value: SVColumnView<'_>,
) -> Result<SVContourField, SVError> {
    let observations = sv_collect_observations(x, y, value, configuration.missing_policy)?;
    if observations.is_empty() {
        return Err(SVError::EmptyInput {
            operation: "contour_points",
        });
    }
    let horizontal: Vec<f64> = observations
        .iter()
        .map(|observation| observation.x)
        .collect();
    let vertical: Vec<f64> = observations
        .iter()
        .map(|observation| observation.y)
        .collect();
    let x_edges = sv_bin_edges(&horizontal, configuration.grid_count)?;
    let y_edges = sv_bin_edges(&vertical, configuration.grid_count)?;
    let columns = x_edges.len() - 1;
    let rows = y_edges.len() - 1;
    let mut cell_codes = Vec::with_capacity(observations.len());
    let mut observed_values = Vec::with_capacity(observations.len());
    for observation in &observations {
        let column = sv_bin_index(&x_edges, observation.x);
        let row = sv_bin_index(&y_edges, observation.y);
        cell_codes.push((row * columns + column) as f64);
        observed_values.push(observation.value);
    }
    let samples = sv_cell_means(&observed_values, &cell_codes, columns * rows)?;
    let values = sv_fill_missing(
        &samples,
        columns,
        rows,
        configuration.missing_policy,
        "contour_points",
    )?;
    Ok(SVContourField::new(
        sv_centres(&x_edges),
        sv_centres(&y_edges),
        values,
    ))
}

fn sv_collect_observations(
    x: SVColumnView<'_>,
    y: SVColumnView<'_>,
    value: SVColumnView<'_>,
    policy: SVContourMissingPolicy,
) -> Result<Vec<SVPointObservation>, SVError> {
    let mut observations = Vec::new();
    for index in 0..value.length() {
        let (Some(x_value), Some(y_value), Some(sample)) =
            (x.value_at(index), y.value_at(index), value.value_at(index))
        else {
            if policy == SVContourMissingPolicy::Error {
                return Err(SVError::NonFiniteValue {
                    operation: "contour_points",
                    position: index,
                });
            }
            continue;
        };
        if !x_value.is_finite() || !y_value.is_finite() || !sample.is_finite() {
            if policy == SVContourMissingPolicy::Error {
                return Err(SVError::NonFiniteValue {
                    operation: "contour_points",
                    position: index,
                });
            }
            continue;
        }
        observations.push(SVPointObservation {
            x: x_value,
            y: y_value,
            value: sample,
        });
    }
    Ok(observations)
}

fn sv_bin_edges(values: &[f64], grid_count: usize) -> Result<Vec<f64>, SVError> {
    let stat = SVStat::bin(SVBinOptions::new().bin_count(grid_count));
    match stat.apply(SVStatInput::new(SVColumnView::Dense(values)))? {
        SVStatResult::Bins(bins) => Ok(bins.edges().to_vec()),
        _ => Err(SVError::InvalidConfiguration {
            message: "the contour composition expects the bin kernel".to_string(),
        }),
    }
}

/// Computes the mean of the observations that fall in every cell.
///
/// The cell index is the group code handed to [`SVStat::Aggregate`], so the
/// recipe composes the kernel instead of owning a mean of its own. A cell with
/// no observation stays `NaN` and follows the missing-value policy.
fn sv_cell_means(
    values: &[f64],
    cell_codes: &[f64],
    number_of_cells: usize,
) -> Result<Vec<f64>, SVError> {
    let stat = SVStat::aggregate(SVAggregateOptions::new().method(SVAggregateMethod::Mean));
    let input =
        SVStatInput::new(SVColumnView::Dense(values)).with_groups(SVColumnView::Dense(cell_codes));
    let SVStatResult::Groups(groups) = stat.apply(input)? else {
        return Err(SVError::InvalidConfiguration {
            message: "the contour composition expects the aggregate kernel".to_string(),
        });
    };
    let mut samples = vec![f64::NAN; number_of_cells];
    for group in groups.groups() {
        let Some(code) = group.group() else {
            continue;
        };
        let Ok(position) = usize::try_from(code.round() as i64) else {
            continue;
        };
        if let Some(sample) = samples.get_mut(position) {
            *sample = group.value();
        }
    }
    Ok(samples)
}

fn sv_bin_index(edges: &[f64], value: f64) -> usize {
    let last = edges.len().saturating_sub(2);
    edges
        .partition_point(|edge| *edge <= value)
        .saturating_sub(1)
        .min(last)
}

fn sv_centres(edges: &[f64]) -> Vec<f64> {
    edges
        .windows(2)
        .map(|pair| (pair[0] + pair[1]) / 2.0)
        .collect()
}
