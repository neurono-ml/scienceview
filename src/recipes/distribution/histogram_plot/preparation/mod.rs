//! Turning the source columns into the prepared, data-space histogram.
//!
//! The preparation resolves the source once per operation, groups the
//! observations by `series`, bins every group through `SVStat::Bin`, and
//! applies the configured stack or dodge through `SVMove`. Every intermediate
//! value is in data space; the composition is the only place a scale turns one
//! into pixels.

mod binning;
mod moves;
mod observations;
mod prepared;

pub(crate) use prepared::{SVPreparedHistogram, SVPreparedSeries};

use super::configuration::SVHistogramConfiguration;
use crate::sv_error::SVError;

/// Resolves the source and returns the prepared histogram.
pub(super) fn sv_prepare(
    configuration: &SVHistogramConfiguration<'_>,
    plot_extent: f64,
) -> Result<SVPreparedHistogram, SVError> {
    let mut value_snapshot = configuration.data_source.column(&configuration.value)?;
    let mut series_snapshot = match configuration.series.as_deref() {
        Some(name) => Some(configuration.data_source.column(name)?),
        None => None,
    };

    let value = value_snapshot.column_view();
    let series = series_snapshot
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

    let groups = observations::sv_observation_groups(
        value,
        series,
        configuration.missing_value_policy,
        &configuration.series_labels,
    )?;

    let binned = binning::sv_bin_groups(configuration, &groups, plot_extent)?;
    let series = moves::sv_prepare_series(configuration, binned)?;

    Ok(SVPreparedHistogram { series })
}
