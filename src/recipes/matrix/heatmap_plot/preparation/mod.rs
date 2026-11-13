//! Resolving the matrix from its configured representation.
//!
//! A direct matrix is already row-major. A long table is pivoted here: the row
//! and column key columns become the two axes and the value column fills the
//! cells. Duplicate (row, column) pairs are aggregated with
//! [`SVStat::Aggregate`](crate::sv_stat::SVStat::Aggregate), which is the stat
//! kernel the FDR composition names. A sparse value column contributes only its
//! stored entries, so it is never densified, and a streaming column contributes
//! its current window.

mod axis_keys;
mod table;

use self::table::sv_prepare_table;
use super::configuration::{SVHeatmapConfiguration, SVHeatmapInput};
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;

/// One prepared cell, in data space.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SVPreparedCell {
    pub(super) row: usize,
    pub(super) column: usize,
    pub(super) value: f64,
}

/// The data-space matrix one composition draws.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SVPreparedHeatmap {
    pub(super) row_labels: Vec<String>,
    pub(super) column_labels: Vec<String>,
    pub(super) cells: Vec<SVPreparedCell>,
    pub(super) representation: SVRepresentation,
}

/// Resolves the configured input into the prepared matrix.
pub(super) fn sv_prepare(
    configuration: &SVHeatmapConfiguration<'_>,
) -> Result<SVPreparedHeatmap, SVError> {
    match configuration.input {
        SVHeatmapInput::Matrix(matrix) => {
            let row_labels =
                sv_explicit_labels(configuration.row_labels, matrix.number_of_rows(), "row");
            let column_labels = sv_explicit_labels(
                configuration.column_labels,
                matrix.number_of_columns(),
                "column",
            );
            let mut cells = Vec::with_capacity(
                matrix
                    .number_of_rows()
                    .saturating_mul(matrix.number_of_columns()),
            );
            for row in 0..matrix.number_of_rows() {
                for column in 0..matrix.number_of_columns() {
                    if let Some(value) = matrix.value_at(row, column) {
                        cells.push(SVPreparedCell { row, column, value });
                    }
                }
            }
            Ok(SVPreparedHeatmap {
                row_labels,
                column_labels,
                cells,
                representation: SVRepresentation::Dense,
            })
        }
        SVHeatmapInput::Table {
            data_source,
            row,
            column,
            value,
        } => sv_prepare_table(data_source, row, column, value),
    }
}

fn sv_explicit_labels(labels: Option<&[&str]>, count: usize, axis: &str) -> Vec<String> {
    match labels {
        Some(labels) => labels.iter().map(|label| (*label).to_string()).collect(),
        None => (0..count).map(|index| format!("{axis} {index}")).collect(),
    }
}
