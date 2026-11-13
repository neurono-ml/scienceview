//! Resolving the columns, grouping the observations and applying the kernel.

use scienceview_core::sv_data::{SVColumnView, SVRepresentation};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_scale::sv_default_scale_palette;
use scienceview_core::sv_stat::{
    SVKernelDensityOptions, SVKernelDensityResult, SVNormalisation, SVStat, SVStatInput,
    SVStatResult,
};

use super::super::configuration::SVDensityPlotConfiguration;
use super::group_values::{SVDensityGroupValues, SVDensityStorage};
use super::prepared_density::{SVPreparedDensity, SVPreparedDensitySeries};

pub(crate) fn sv_prepare_density(
    configuration: &SVDensityPlotConfiguration<'_>,
) -> Result<SVPreparedDensity, SVError> {
    let mut value_snapshot = configuration.data_source.column(configuration.value)?;
    let mut series_snapshot = match configuration.series {
        Some(name) => Some(configuration.data_source.column(name)?),
        None => None,
    };
    let mut weight_snapshot = match configuration.weight {
        Some(name) => Some(configuration.data_source.column(name)?),
        None => None,
    };

    let value_view = value_snapshot.column_view();
    let series_view = series_snapshot
        .as_mut()
        .map(|snapshot| snapshot.column_view());
    let weight_view = weight_snapshot
        .as_mut()
        .map(|snapshot| snapshot.column_view());

    let representation = value_view.representation();
    let length = value_view.length();
    if let Some(series) = series_view {
        if series.length() != length {
            return Err(SVError::MismatchedLengths {
                name: configuration.series.unwrap_or("series").to_string(),
                expected: length,
                actual: series.length(),
            });
        }
    }
    if let Some(weight) = weight_view {
        if weight.length() != length {
            return Err(SVError::MismatchedLengths {
                name: configuration.weight.unwrap_or("weight").to_string(),
                expected: length,
                actual: weight.length(),
            });
        }
    }

    let groups = sv_extract_groups(configuration, value_view, weight_view, series_view)?;
    if groups.is_empty() {
        return Err(SVError::EmptyInput {
            operation: "kernel_density",
        });
    }
    let palette = sv_default_scale_palette();

    let mut series = Vec::with_capacity(groups.len());
    let mut point_count = 0;
    for (ordinal, group) in groups.iter().enumerate() {
        let evaluated = sv_evaluate_group(configuration, group)?;
        point_count += evaluated.grid.len();
        series.push(SVPreparedDensitySeries {
            colour: palette.colour_at_index(ordinal),
            grid: evaluated.grid,
            density: evaluated.density,
            bandwidth: evaluated.bandwidth,
        });
    }

    Ok(SVPreparedDensity {
        representation,
        point_count,
        windowed: representation == SVRepresentation::Streaming,
        series,
    })
}

/// Extracts one group per distinct series code, or one group for the whole
/// column when no grouping column was configured.
fn sv_extract_groups<'values>(
    configuration: &SVDensityPlotConfiguration<'_>,
    value: SVColumnView<'values>,
    weight: Option<SVColumnView<'values>>,
    series: Option<SVColumnView<'values>>,
) -> Result<Vec<SVDensityGroupValues<'values>>, SVError> {
    match series {
        None => Ok(vec![sv_single_group(configuration, value, weight)?]),
        Some(series) => sv_grouped_groups(configuration, value, weight, series),
    }
}

/// Keeps the whole column as one group.
///
/// When every stored observation is finite the column is borrowed as it
/// arrived, so a sparse input reaches the kernel without being materialised.
/// Only a column that actually holds a missing value is rebuilt, and even then
/// its implicit zeros stay implicit; the weights are rebuilt with it so the two
/// columns stay aligned.
fn sv_single_group<'values>(
    configuration: &SVDensityPlotConfiguration<'_>,
    value: SVColumnView<'values>,
    weight: Option<SVColumnView<'values>>,
) -> Result<SVDensityGroupValues<'values>, SVError> {
    let storage = match value {
        SVColumnView::Dense(values) => {
            let mut kept = Vec::with_capacity(values.len());
            let mut retained = Vec::with_capacity(values.len());
            for (index, value) in values.iter().copied().enumerate() {
                if value.is_finite() {
                    kept.push(value);
                    retained.push(index);
                } else {
                    configuration
                        .missing_value_policy
                        .report(configuration.value, index)?;
                }
            }
            if retained.len() == values.len() {
                SVDensityStorage::Borrowed(SVColumnView::Dense(values))
            } else {
                return Ok(SVDensityGroupValues {
                    storage: SVDensityStorage::OwnedDense(kept),
                    weights: weight.map(|weight| sv_select_weights(weight, &retained)),
                });
            }
        }
        SVColumnView::Streaming(streaming) => {
            let values = streaming.values();
            let mut kept = Vec::with_capacity(values.len());
            let mut retained = Vec::with_capacity(values.len());
            for (index, value) in values.iter().copied().enumerate() {
                if value.is_finite() {
                    kept.push(value);
                    retained.push(index);
                } else {
                    configuration
                        .missing_value_policy
                        .report(configuration.value, index)?;
                }
            }
            if retained.len() == values.len() {
                SVDensityStorage::Borrowed(SVColumnView::Streaming(streaming))
            } else {
                return Ok(SVDensityGroupValues {
                    storage: SVDensityStorage::OwnedStreaming {
                        values: kept,
                        sequence: streaming.sequence(),
                        dropped_samples: streaming.dropped_samples(),
                    },
                    weights: weight.map(|weight| sv_select_weights(weight, &retained)),
                });
            }
        }
        SVColumnView::Sparse(sparse) => {
            let dropped: Vec<usize> = sparse
                .stored_entries()
                .filter(|(_, value)| !value.is_finite())
                .map(|(index, _)| index)
                .collect();
            if dropped.is_empty() {
                SVDensityStorage::Borrowed(SVColumnView::Sparse(sparse))
            } else {
                if let Some(position) = dropped.first() {
                    configuration
                        .missing_value_policy
                        .report(configuration.value, *position)?;
                }
                let mut indices = Vec::with_capacity(sparse.number_of_stored_values());
                let mut values = Vec::with_capacity(sparse.number_of_stored_values());
                for (index, value) in sparse.stored_entries() {
                    if value.is_finite() {
                        indices.push(index);
                        values.push(value);
                    }
                }
                return Ok(SVDensityGroupValues {
                    storage: SVDensityStorage::OwnedSparse {
                        indices,
                        values,
                        length: sparse.length(),
                    },
                    weights: weight.map(|weight| sv_zero_weights(weight, &dropped)),
                });
            }
        }
    };

    Ok(SVDensityGroupValues {
        storage,
        weights: weight.map(SVDensityStorage::Borrowed),
    })
}

/// Splits a dense or streaming column by its series codes, in first-appearance
/// order.
///
/// A sparse value column paired with a series column cannot be grouped without
/// walking its whole logical lattice to attribute every implicit zero, which
/// would be a densification. The recipe rejects that combination up front and
/// names the alternative, as the data boundary requires.
fn sv_grouped_groups<'values>(
    configuration: &SVDensityPlotConfiguration<'_>,
    value: SVColumnView<'values>,
    weight: Option<SVColumnView<'values>>,
    series: SVColumnView<'values>,
) -> Result<Vec<SVDensityGroupValues<'values>>, SVError> {
    if matches!(value, SVColumnView::Sparse(_)) {
        return Err(SVError::UnsupportedRepresentation {
            operation: "density_plot",
            representation: SVRepresentation::Sparse,
            alternative: "supply the value column as a dense or streaming column, or estimate one density without a `series` column",
        });
    }

    let mut groups: Vec<(f64, Vec<f64>, Vec<f64>)> = Vec::new();
    for index in 0..value.length() {
        let code = series.value_at(index).unwrap_or(f64::NAN);
        if !code.is_finite() {
            configuration
                .missing_value_policy
                .report(configuration.series.unwrap_or("series"), index)?;
            continue;
        }
        let observation = value.value_at(index).unwrap_or(f64::NAN);
        if !observation.is_finite() {
            configuration
                .missing_value_policy
                .report(configuration.value, index)?;
            continue;
        }
        let observation_weight = weight
            .and_then(|weight| weight.value_at(index))
            .unwrap_or(1.0);
        match groups.iter_mut().find(|(group, _, _)| *group == code) {
            Some((_, group, group_weights)) => {
                group.push(observation);
                group_weights.push(observation_weight);
            }
            None => groups.push((code, vec![observation], vec![observation_weight])),
        }
    }

    Ok(groups
        .into_iter()
        .map(|(_, values, weights)| SVDensityGroupValues {
            storage: SVDensityStorage::OwnedDense(values),
            weights: weight.map(|_| SVDensityStorage::OwnedDense(weights)),
        })
        .collect())
}

/// Reads the weights at the given logical positions, preserving sparse storage.
fn sv_select_weights<'values>(
    weight: SVColumnView<'values>,
    retained: &[usize],
) -> SVDensityStorage<'values> {
    match weight {
        SVColumnView::Dense(values) => SVDensityStorage::OwnedDense(
            retained
                .iter()
                .map(|position| values.get(*position).copied().unwrap_or(0.0))
                .collect(),
        ),
        SVColumnView::Streaming(streaming) => {
            let values = streaming.values();
            SVDensityStorage::OwnedDense(
                retained
                    .iter()
                    .map(|position| values.get(*position).copied().unwrap_or(0.0))
                    .collect(),
            )
        }
        SVColumnView::Sparse(sparse) => {
            let mut indices = Vec::new();
            let mut values = Vec::new();
            for (stored_index, stored_value) in sparse.stored_entries() {
                if let Ok(position) = retained.binary_search(&stored_index) {
                    indices.push(position);
                    values.push(stored_value);
                }
            }
            SVDensityStorage::OwnedSparse {
                indices,
                values,
                length: retained.len(),
            }
        }
    }
}

/// Sets the weight of every given position to zero.
///
/// A sparse value column turns a dropped stored entry into an implicit zero
/// without shortening its logical length; the weight at that position must
/// contribute nothing, so it is zeroed in place instead of being removed.
fn sv_zero_weights<'values>(
    weight: SVColumnView<'values>,
    dropped: &[usize],
) -> SVDensityStorage<'values> {
    match weight {
        SVColumnView::Dense(values) => {
            let mut values = values.to_vec();
            for position in dropped {
                if let Some(value) = values.get_mut(*position) {
                    *value = 0.0;
                }
            }
            SVDensityStorage::OwnedDense(values)
        }
        SVColumnView::Streaming(streaming) => {
            let mut values = streaming.values().to_vec();
            for position in dropped {
                if let Some(value) = values.get_mut(*position) {
                    *value = 0.0;
                }
            }
            SVDensityStorage::OwnedStreaming {
                values,
                sequence: streaming.sequence(),
                dropped_samples: streaming.dropped_samples(),
            }
        }
        SVColumnView::Sparse(sparse) => {
            let mut indices = Vec::with_capacity(sparse.number_of_stored_values() + dropped.len());
            let mut values = Vec::with_capacity(sparse.number_of_stored_values() + dropped.len());
            let mut dropped_positions = dropped.iter().copied().peekable();
            for (position, value) in sparse.stored_entries() {
                while dropped_positions
                    .peek()
                    .is_some_and(|next| *next < position)
                {
                    indices.push(dropped_positions.next().unwrap_or(position));
                    values.push(0.0);
                }
                if dropped_positions
                    .peek()
                    .is_some_and(|next| *next == position)
                {
                    dropped_positions.next();
                    indices.push(position);
                    values.push(0.0);
                } else {
                    indices.push(position);
                    values.push(value);
                }
            }
            for position in dropped_positions {
                indices.push(position);
                values.push(0.0);
            }
            SVDensityStorage::OwnedSparse {
                indices,
                values,
                length: sparse.length(),
            }
        }
    }
}

/// Applies the configured kernel to one group.
fn sv_evaluate_group(
    configuration: &SVDensityPlotConfiguration<'_>,
    group: &SVDensityGroupValues<'_>,
) -> Result<SVEvaluatedDensity, SVError> {
    let first = group.evaluate(|view, weights| sv_apply_density(configuration, view, weights))?;
    let adjusted_bandwidth = first.bandwidth() * configuration.bandwidth_adjustment;

    let result = if (configuration.bandwidth_adjustment - 1.0).abs() > f64::EPSILON {
        let options = sv_density_options(configuration).bandwidth(adjusted_bandwidth);
        group.evaluate(|view, weights| sv_apply_density_options(&options, view, weights))?
    } else {
        first
    };

    let total_weight = group.total_weight();
    let spacing = result
        .grid()
        .windows(2)
        .next()
        .map(|pair| pair[1] - pair[0])
        .unwrap_or(1.0);
    let factor = match configuration.normalisation {
        SVNormalisation::Density => 1.0,
        SVNormalisation::Probability => spacing,
        SVNormalisation::Count => spacing * total_weight,
    };

    Ok(SVEvaluatedDensity {
        grid: result.grid().to_vec(),
        density: result
            .density()
            .iter()
            .map(|density| density * factor)
            .collect(),
        bandwidth: result.bandwidth(),
    })
}

fn sv_density_options(configuration: &SVDensityPlotConfiguration<'_>) -> SVKernelDensityOptions {
    let mut options = SVKernelDensityOptions::new()
        .kernel(configuration.kernel)
        .grid_count(configuration.grid_count)
        .cut(configuration.cut);
    if let Some(bandwidth) = configuration.bandwidth {
        options = options.bandwidth(bandwidth);
    }
    options
}

fn sv_apply_density(
    configuration: &SVDensityPlotConfiguration<'_>,
    view: SVColumnView<'_>,
    weights: Option<SVColumnView<'_>>,
) -> Result<SVKernelDensityResult, SVError> {
    sv_apply_density_options(&sv_density_options(configuration), view, weights)
}

fn sv_apply_density_options(
    options: &SVKernelDensityOptions,
    view: SVColumnView<'_>,
    weights: Option<SVColumnView<'_>>,
) -> Result<SVKernelDensityResult, SVError> {
    let mut input = SVStatInput::new(view);
    if let Some(weights) = weights {
        input = input.with_weights(weights);
    }
    match SVStat::kernel_density(options.clone()).apply(input)? {
        SVStatResult::Density(result) => Ok(result),
        _ => Err(SVError::InvalidConfiguration {
            message: "the kernel-density stat returned a result that is not a density".to_string(),
        }),
    }
}

/// The owned result of one kernel-density application.
struct SVEvaluatedDensity {
    grid: Vec<f64>,
    density: Vec<f64>,
    bandwidth: f64,
}
