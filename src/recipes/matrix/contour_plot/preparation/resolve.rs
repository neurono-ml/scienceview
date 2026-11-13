//! Resolving the configured layout once per operation.

use super::super::configuration::{SVContourConfiguration, SVContourLayout};
use super::super::field::SVContourField;
use crate::sv_data::{SVColumnSnapshot, SVDataSource};
use crate::sv_error::SVError;

/// Builds the sampled field the configured layout describes.
pub(crate) fn sv_prepare_field(
    configuration: &SVContourConfiguration<'_>,
) -> Result<SVContourField, SVError> {
    let mut value_snapshot = configuration.data_source.column(&configuration.value)?;
    let value = value_snapshot.column_view();
    match configuration.layout {
        SVContourLayout::Matrix { rows, columns } => {
            super::matrix::sv_matrix_field(configuration, value, rows, columns)
        }
        SVContourLayout::Points => {
            let mut x = sv_column(configuration.data_source, configuration.x.as_deref())?;
            let mut y = sv_column(configuration.data_source, configuration.y.as_deref())?;
            let x_view = x.column_view();
            let y_view = y.column_view();
            super::points::sv_points_field(configuration, x_view, y_view, value)
        }
    }
}

/// Resolves one named column of the point layout.
pub(crate) fn sv_column<'source>(
    data_source: &'source dyn SVDataSource,
    name: Option<&str>,
) -> Result<SVColumnSnapshot<'source>, SVError> {
    match name {
        Some(name) => data_source.column(name),
        None => Err(SVError::InvalidConfiguration {
            message: "the contour layout needs both `x` and `y` columns".to_string(),
        }),
    }
}
