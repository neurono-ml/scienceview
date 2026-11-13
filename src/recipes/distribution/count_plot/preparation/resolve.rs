//! Resolving the source columns into the prepared count plot.

use super::super::configuration::SVCountConfiguration;
use super::SVPreparedCount;
use super::counting;
use super::moves;
use super::observations;
use crate::sv_data::SVColumnSnapshot;
use crate::sv_error::SVError;

/// Resolves the source and returns the prepared count plot.
pub(crate) fn sv_prepare(
    configuration: &SVCountConfiguration<'_>,
) -> Result<SVPreparedCount, SVError> {
    let mut category_snapshot = configuration.data_source.column(&configuration.category)?;
    let mut series_snapshot = match configuration.series.as_deref() {
        Some(name) => Some(configuration.data_source.column(name)?),
        None => None,
    };

    let category = category_snapshot.column_view();
    let series = series_snapshot.as_mut().map(SVColumnSnapshot::column_view);

    if let Some(series) = series {
        if series.length() != category.length() {
            return Err(SVError::MismatchedLengths {
                name: configuration
                    .series
                    .clone()
                    .unwrap_or_else(|| "series".to_string()),
                expected: category.length(),
                actual: series.length(),
            });
        }
    }

    let categories = counting::sv_category_axis(configuration, category, series)?;
    let groups = observations::sv_category_groups(
        category,
        series,
        configuration.missing_value_policy,
        &configuration.series_labels,
        configuration.colour,
    )?;
    let counted = counting::sv_count_groups(configuration, &groups, &categories)?;
    moves::sv_prepare_series(configuration, categories, counted)
}
