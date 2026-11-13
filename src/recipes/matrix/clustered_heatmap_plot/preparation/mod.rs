//! Resolving the matrix from its configured representation.
//!
//! A direct matrix is already row-major. A long table is pivoted here through
//! [`SVStat::Aggregate`](crate::sv_stat::SVStat::Aggregate), the stat kernel the
//! FDR composition names. A sparse value column contributes only its stored
//! entries, so it is never densified. Streaming is rejected in `build`, before
//! this module runs.

mod table;

use self::table::sv_prepare_table;
use super::SVClusterAxis;
use super::configuration::{SVClusteredHeatmapConfiguration, SVClusteredHeatmapInput};
use super::options::sv_validate_cluster_order;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;

/// One prepared cell, in data space and original axis indices.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SVPreparedClusteredCell {
    pub(super) row: usize,
    pub(super) column: usize,
    pub(super) value: f64,
}

/// The data-space matrix one composition draws.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SVPreparedClusteredHeatmap {
    pub(super) row_labels: Vec<String>,
    pub(super) column_labels: Vec<String>,
    pub(super) cells: Vec<SVPreparedClusteredCell>,
    pub(super) representation: SVRepresentation,
}

impl SVPreparedClusteredHeatmap {
    /// Returns the number of rows the matrix has.
    pub(super) fn number_of_rows(&self) -> usize {
        self.row_labels.len()
    }

    /// Returns the number of columns the matrix has.
    pub(super) fn number_of_columns(&self) -> usize {
        self.column_labels.len()
    }
}

/// Resolves the configured input into the prepared matrix.
pub(super) fn sv_prepare(
    configuration: &SVClusteredHeatmapConfiguration<'_>,
) -> Result<SVPreparedClusteredHeatmap, SVError> {
    match configuration.input {
        SVClusteredHeatmapInput::Matrix(matrix) => {
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
                        cells.push(SVPreparedClusteredCell { row, column, value });
                    }
                }
            }
            Ok(SVPreparedClusteredHeatmap {
                row_labels,
                column_labels,
                cells,
                representation: SVRepresentation::Dense,
            })
        }
        SVClusteredHeatmapInput::Table {
            data_source,
            row,
            column,
            value,
        } => {
            let mut prepared = sv_prepare_table(data_source, row, column, value)?;
            if let Some(labels) = configuration.row_labels {
                if labels.len() != prepared.row_labels.len() {
                    return Err(SVError::MismatchedLengths {
                        name: "row labels".to_string(),
                        expected: prepared.row_labels.len(),
                        actual: labels.len(),
                    });
                }
                prepared.row_labels = labels.iter().map(|label| (*label).to_string()).collect();
            }
            if let Some(labels) = configuration.column_labels {
                if labels.len() != prepared.column_labels.len() {
                    return Err(SVError::MismatchedLengths {
                        name: "column labels".to_string(),
                        expected: prepared.column_labels.len(),
                        actual: labels.len(),
                    });
                }
                prepared.column_labels = labels.iter().map(|label| (*label).to_string()).collect();
            }
            Ok(prepared)
        }
    }
}

/// Validates the supplied order and tree against the resolved dimensions.
///
/// A direct matrix is validated in `build`; a long table only knows its axis
/// cardinality after the pivot, so this runs at composition time.
pub(super) fn sv_validate_supplied_clustering(
    configuration: &SVClusteredHeatmapConfiguration<'_>,
    prepared: &SVPreparedClusteredHeatmap,
) -> Result<(), SVError> {
    sv_validate_cluster_order(
        configuration.row_order,
        SVClusterAxis::Row,
        prepared.number_of_rows(),
    )?;
    sv_validate_cluster_order(
        configuration.column_order,
        SVClusterAxis::Column,
        prepared.number_of_columns(),
    )?;
    if let Some(tree) = configuration.row_tree {
        super::options::sv_validate_cluster_tree(
            tree,
            SVClusterAxis::Row,
            prepared.number_of_rows(),
        )?;
    }
    if let Some(tree) = configuration.column_tree {
        super::options::sv_validate_cluster_tree(
            tree,
            SVClusterAxis::Column,
            prepared.number_of_columns(),
        )?;
    }
    if configuration.dendrogram
        && (configuration.row_tree.is_none() || configuration.column_tree.is_none())
    {
        return Err(SVError::InvalidConfiguration {
            message: "the dendrogram needs both `row_linkage(...)` and `column_linkage(...)`; supply the supplied clustering tree or call `dendrogram(false)`".to_string(),
        });
    }
    Ok(())
}

fn sv_explicit_labels(labels: Option<&[&str]>, count: usize, axis: &str) -> Vec<String> {
    match labels {
        Some(labels) => labels.iter().map(|label| (*label).to_string()).collect(),
        None => (0..count).map(|index| format!("{axis} {index}")).collect(),
    }
}
