//! Building the sampled field a contour plot draws.
//!
//! `mod.rs` is a pure dispatcher. The matrix module folds a row-major matrix
//! column onto a lattice with configured axis ranges; the points module bins
//! an `x`, `y`, `value` cloud through the bin kernel and averages every cell;
//! the missing module applies the configured policy to a non-finite sample.

mod matrix;
mod missing;
mod points;

use super::configuration::{SVContourConfiguration, SVContourLayout};
use super::field::SVContourField;
use crate::sv_data::{SVColumnSnapshot, SVDataSource};
use crate::sv_error::SVError;

pub(crate) fn sv_prepare_field(
    configuration: &SVContourConfiguration<'_>,
) -> Result<SVContourField, SVError> {
    let mut value_snapshot = configuration.data_source.column(&configuration.value)?;
    let value = value_snapshot.column_view();
    match configuration.layout {
        SVContourLayout::Matrix { rows, columns } => {
            matrix::sv_matrix_field(configuration, value, rows, columns)
        }
        SVContourLayout::Points => {
            let mut x = sv_column(configuration.data_source, configuration.x.as_deref())?;
            let mut y = sv_column(configuration.data_source, configuration.y.as_deref())?;
            let x_view = x.column_view();
            let y_view = y.column_view();
            points::sv_points_field(configuration, x_view, y_view, value)
        }
    }
}

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
