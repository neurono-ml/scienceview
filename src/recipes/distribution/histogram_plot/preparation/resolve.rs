//! Resolving the source columns into the prepared histogram.

use super::SVPreparedHistogram;
use super::binning;
use super::moves;
use super::observations;
use crate::sv_error::SVError;

use super::super::configuration::SVHistogramConfiguration;

/// Resolves the source and returns the prepared histogram.
pub(crate) fn sv_prepare(
    configuration: &SVHistogramConfiguration<'_>,
    plot_extent: f64,
) -> Result<SVPreparedHistogram, SVError> {
    let mut value_snapshot = configuration.data_source.column(&configuration.value)?;
    let mut series_snapshot = match configuration.series.as_deref() {
        Some(name) => Some(configuration.data_source.column(name)?),
        None => None,
    };
    let mut weight_snapshot = match configuration.weight.as_deref() {
        Some(name) => Some(configuration.data_source.column(name)?),
        None => None,
    };

    let value = value_snapshot.column_view();
    let series = series_snapshot
        .as_mut()
        .map(crate::sv_data::SVColumnSnapshot::column_view);
    let weight = weight_snapshot
        .as_mut()
        .map(crate::sv_data::SVColumnSnapshot::column_view);

    if let Some(series) = series {
        if series.length() != value.length() {
            return Err(SVError::MismatchedLengths {
                name: configuration
                    .series
                    .clone()
                    .unwrap_or_else(|| "series".to_string()),
                expected: value.length(),
                actual: series.length(),
            });
        }
    }
    if let Some(weight) = weight {
        if weight.length() != value.length() {
            return Err(SVError::MismatchedLengths {
                name: configuration
                    .weight
                    .clone()
                    .unwrap_or_else(|| "weight".to_string()),
                expected: value.length(),
                actual: weight.length(),
            });
        }
    }

    let groups = observations::sv_observation_groups(
        value,
        weight,
        series,
        configuration.missing_value_policy,
        &configuration.series_labels,
    )?;

    let binned = binning::sv_bin_groups(configuration, &groups, plot_extent)?;
    let series = moves::sv_prepare_series(configuration, binned)?;

    Ok(SVPreparedHistogram { series })
}
