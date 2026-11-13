//! Resolving the columns, grouping the observations and applying the kernel.

use scienceview_core::sv_data::{SVColumnView, SVRepresentation};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_scale::sv_default_scale_palette;
use scienceview_core::sv_stat::{
    SVKernelDensityOptions, SVKernelDensityResult, SVNormalisation, SVStat, SVStatInput,
    SVStatResult,
};

use super::super::configuration::SVDensityPlotConfiguration;
use super::group_values::SVDensityGroupValues;
use super::prepared_density::{SVPreparedDensity, SVPreparedDensitySeries};

pub(crate) fn sv_prepare_density(
    configuration: &SVDensityPlotConfiguration<'_>,
) -> Result<SVPreparedDensity, SVError> {
    let mut value_snapshot = configuration.data_source.column(configuration.value)?;
    let mut series_snapshot = match configuration.series {
        Some(name) => Some(configuration.data_source.column(name)?),
        None => None,
    };

    let value_view = value_snapshot.column_view();
    let series_view = series_snapshot
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

    let groups = sv_extract_groups(configuration, value_view, series_view)?;
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
    series: Option<SVColumnView<'values>>,
) -> Result<Vec<SVDensityGroupValues<'values>>, SVError> {
    match series {
        None => Ok(vec![sv_single_group(configuration, value)?]),
        Some(series) => sv_grouped_groups(configuration, value, series),
    }
}

/// Keeps the whole column as one group.
///
/// When every stored observation is finite the column is borrowed as it
/// arrived, so a sparse input reaches the kernel without being materialised.
/// Only a column that actually holds a missing value is rebuilt, and even then
/// its implicit zeros stay implicit.
fn sv_single_group<'values>(
    configuration: &SVDensityPlotConfiguration<'_>,
    value: SVColumnView<'values>,
) -> Result<SVDensityGroupValues<'values>, SVError> {
    match value {
        SVColumnView::Dense(values) => {
            if values.iter().all(|value| value.is_finite()) {
                return Ok(SVDensityGroupValues::Borrowed(SVColumnView::Dense(values)));
            }
            let mut finite = Vec::with_capacity(values.len());
            for (index, value) in values.iter().copied().enumerate() {
                if value.is_finite() {
                    finite.push(value);
                } else {
                    configuration
                        .missing_value_policy
                        .report(configuration.value, index)?;
                }
            }
            Ok(SVDensityGroupValues::OwnedDense(finite))
        }
        SVColumnView::Streaming(streaming) => {
            if streaming.values().iter().all(|value| value.is_finite()) {
                return Ok(SVDensityGroupValues::Borrowed(SVColumnView::Streaming(
                    streaming,
                )));
            }
            let mut finite = Vec::with_capacity(streaming.length());
            for (index, value) in streaming.values().iter().copied().enumerate() {
                if value.is_finite() {
                    finite.push(value);
                } else {
                    configuration
                        .missing_value_policy
                        .report(configuration.value, index)?;
                }
            }
            Ok(SVDensityGroupValues::OwnedStreaming {
                values: finite,
                sequence: streaming.sequence(),
                dropped_samples: streaming.dropped_samples(),
            })
        }
        SVColumnView::Sparse(sparse) => {
            if sparse.values().iter().all(|value| value.is_finite()) {
                return Ok(SVDensityGroupValues::Borrowed(SVColumnView::Sparse(sparse)));
            }
            let mut indices = Vec::with_capacity(sparse.number_of_stored_values());
            let mut values = Vec::with_capacity(sparse.number_of_stored_values());
            for (index, value) in sparse.stored_entries() {
                if value.is_finite() {
                    indices.push(index);
                    values.push(value);
                } else {
                    configuration
                        .missing_value_policy
                        .report(configuration.value, index)?;
                }
            }
            Ok(SVDensityGroupValues::OwnedSparse {
                indices,
                values,
                length: sparse.length(),
            })
        }
    }
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
    series: SVColumnView<'values>,
) -> Result<Vec<SVDensityGroupValues<'values>>, SVError> {
    if matches!(value, SVColumnView::Sparse(_)) {
        return Err(SVError::UnsupportedRepresentation {
            operation: "density_plot",
            representation: SVRepresentation::Sparse,
            alternative: "supply the value column as a dense or streaming column, or estimate one density without a `series` column",
        });
    }

    let mut groups: Vec<(f64, Vec<f64>)> = Vec::new();
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
        match groups.iter_mut().find(|(group, _)| *group == code) {
            Some((_, group)) => group.push(observation),
            None => groups.push((code, vec![observation])),
        }
    }

    Ok(groups
        .into_iter()
        .map(|(_, values)| SVDensityGroupValues::OwnedDense(values))
        .collect())
}

/// Applies the configured kernel to one group.
fn sv_evaluate_group(
    configuration: &SVDensityPlotConfiguration<'_>,
    group: &SVDensityGroupValues<'_>,
) -> Result<SVEvaluatedDensity, SVError> {
    let first = group.evaluate(|view| sv_apply_density(configuration, view))?;
    let adjusted_bandwidth = first.bandwidth() * configuration.bandwidth_adjustment;

    let result = if (configuration.bandwidth_adjustment - 1.0).abs() > f64::EPSILON {
        let options = sv_density_options(configuration).bandwidth(adjusted_bandwidth);
        group.evaluate(|view| sv_apply_density_options(&options, view))?
    } else {
        first
    };

    let observation_count = group.observation_count();
    let spacing = result
        .grid()
        .windows(2)
        .next()
        .map(|pair| pair[1] - pair[0])
        .unwrap_or(1.0);
    let factor = match configuration.normalisation {
        SVNormalisation::Density => 1.0,
        SVNormalisation::Probability => spacing,
        SVNormalisation::Count => spacing * observation_count as f64,
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
) -> Result<SVKernelDensityResult, SVError> {
    sv_apply_density_options(&sv_density_options(configuration), view)
}

fn sv_apply_density_options(
    options: &SVKernelDensityOptions,
    view: SVColumnView<'_>,
) -> Result<SVKernelDensityResult, SVError> {
    match SVStat::kernel_density(options.clone()).apply(SVStatInput::new(view))? {
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
