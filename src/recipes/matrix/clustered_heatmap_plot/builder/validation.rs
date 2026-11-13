//! Validation and assembly of the built clustered heatmap.

use super::builder_state::SVClusteredHeatmapPlotBuilder;
use crate::sv_backend::sv_verify_mark_supported;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkKind;
use crate::sv_recipes::sv_matrix::sv_clustered_heatmap_plot::SVClusterAxis;
use crate::sv_recipes::sv_matrix::sv_clustered_heatmap_plot::configuration::{
    SVClusteredHeatmapConfiguration, SVClusteredHeatmapInput,
};
use crate::sv_recipes::sv_matrix::sv_clustered_heatmap_plot::core_implementation::SVClusteredHeatmapPlot;
use crate::sv_recipes::sv_matrix::sv_clustered_heatmap_plot::options::{
    sv_validate_cluster_order, sv_validate_cluster_tree,
};

impl<'data> SVClusteredHeatmapPlotBuilder<'data> {
    /// Validates the configuration and builds the clustered heatmap.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingBackendSelection`] without a backend,
    /// [`SVError::UnsupportedRepresentation`] for a streaming source,
    /// [`SVError::InvalidClusterTree`] for an inconsistent supplied tree, and
    /// [`SVError::InvalidConfiguration`] for an incomplete configuration, a
    /// non-finite option or a requested dendrogram without its tree.
    pub fn build(self) -> Result<SVClusteredHeatmapPlot<'data>, SVError> {
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        sv_verify_mark_supported(SVMarkKind::Image, backend_kind)?;
        sv_validate_centre(self.centre)?;

        let row_order = self.row_order.ok_or_else(|| SVError::InvalidConfiguration {
            message: "SVClusteredHeatmapPlot requires `row_order(...)`: the clustering is an input, not a computation".to_string(),
        })?;
        let column_order =
            self.column_order
                .ok_or_else(|| SVError::InvalidConfiguration {
                    message: "SVClusteredHeatmapPlot requires `column_order(...)`: the clustering is an input, not a computation".to_string(),
                })?;

        let (input, representation, number_of_cells, row_dimension, column_dimension) = match self
            .matrix
        {
            Some(matrix) => {
                sv_validate_matrix_labels(matrix, self.row_labels, self.column_labels)?;
                (
                    SVClusteredHeatmapInput::Matrix(matrix),
                    SVRepresentation::Dense,
                    matrix
                        .number_of_rows()
                        .saturating_mul(matrix.number_of_columns()),
                    Some(matrix.number_of_rows()),
                    Some(matrix.number_of_columns()),
                )
            }
            None => {
                let data_source =
                        self.data_source
                            .ok_or_else(|| SVError::InvalidConfiguration {
                                message: "SVClusteredHeatmapPlot requires either `matrix(...)` or `data_source(...)` with `row(...)`, `column(...)` and `value(...)`".to_string(),
                            })?;
                let row = sv_required_column(self.row, "row")?;
                let column = sv_required_column(self.column, "column")?;
                let value = sv_required_column(self.value, "value")?;
                let (length, representation) =
                    sv_probe_columns(data_source, &[row, column, value])?;
                sv_reject_streaming(representation)?;
                (
                    SVClusteredHeatmapInput::Table {
                        data_source,
                        row,
                        column,
                        value,
                    },
                    representation,
                    length,
                    None,
                    None,
                )
            }
        };

        if let (Some(rows), Some(columns)) = (row_dimension, column_dimension) {
            sv_validate_cluster_order(row_order, SVClusterAxis::Row, rows)?;
            sv_validate_cluster_order(column_order, SVClusterAxis::Column, columns)?;
            if let Some(tree) = self.row_tree {
                sv_validate_cluster_tree(tree, SVClusterAxis::Row, rows)?;
            }
            if let Some(tree) = self.column_tree {
                sv_validate_cluster_tree(tree, SVClusterAxis::Column, columns)?;
            }
            if self.dendrogram && (self.row_tree.is_none() || self.column_tree.is_none()) {
                return Err(SVError::InvalidConfiguration {
                    message: "the dendrogram needs both `row_linkage(...)` and `column_linkage(...)`; supply the supplied clustering tree or call `dendrogram(false)`".to_string(),
                });
            }
        }

        Ok(SVClusteredHeatmapPlot {
            configuration: SVClusteredHeatmapConfiguration {
                input,
                row_order,
                column_order,
                row_tree: self.row_tree,
                column_tree: self.column_tree,
                dendrogram: self.dendrogram,
                row_labels: self.row_labels,
                column_labels: self.column_labels,
                colour_map: self.colour_map,
                centre: self.centre,
                colour_bar: self.colour_bar,
                missing: self.missing,
                title: self.title,
                x_label: self.x_label,
                y_label: self.y_label,
                backend_kind,
                execution_mode: self.execution_mode,
                representation,
                number_of_cells,
            },
        })
    }
}

fn sv_required_column<'data>(
    column: Option<&'data str>,
    role: &str,
) -> Result<&'data str, SVError> {
    column.ok_or_else(|| SVError::InvalidConfiguration {
        message: format!(
            "SVClusteredHeatmapPlot requires the `{role}` column of a long table; pass it with `{role}(...)`"
        ),
    })
}

/// Rejects a streaming source before any composition starts.
///
/// Clustering is a static-only exception under `data-view-boundary`: the
/// supplied order and tree describe a whole snapshot, so the recipe refuses to
/// render a live stream and names the documented alternative.
pub(super) fn sv_reject_streaming(representation: SVRepresentation) -> Result<(), SVError> {
    if representation == SVRepresentation::Streaming {
        return Err(SVError::UnsupportedRepresentation {
            operation: "SVClusteredHeatmapPlot",
            representation,
            alternative: "take a snapshot of the current window, compute the clustering outside scienceview, then render that snapshot",
        });
    }
    Ok(())
}

fn sv_validate_centre(centre: Option<f64>) -> Result<(), SVError> {
    if centre.is_some_and(|centre| !centre.is_finite()) {
        return Err(SVError::InvalidConfiguration {
            message: "the colour-map centre must be finite".to_string(),
        });
    }
    Ok(())
}

fn sv_validate_matrix_labels(
    matrix: crate::sv_recipes::sv_matrix::sv_clustered_heatmap_plot::SVHeatmapMatrix<'_>,
    row_labels: Option<&[&str]>,
    column_labels: Option<&[&str]>,
) -> Result<(), SVError> {
    if let Some(labels) = row_labels {
        if labels.len() != matrix.number_of_rows() {
            return Err(SVError::MismatchedLengths {
                name: "row labels".to_string(),
                expected: matrix.number_of_rows(),
                actual: labels.len(),
            });
        }
    }
    if let Some(labels) = column_labels {
        if labels.len() != matrix.number_of_columns() {
            return Err(SVError::MismatchedLengths {
                name: "column labels".to_string(),
                expected: matrix.number_of_columns(),
                actual: labels.len(),
            });
        }
    }
    Ok(())
}

/// Probes the named columns once for existence, equal length and one
/// representation.
pub(super) fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    names: &[&str],
) -> Result<(usize, SVRepresentation), SVError> {
    let mut expected: Option<usize> = None;
    let mut representation = SVRepresentation::Dense;
    for name in names {
        let mut snapshot = data_source.column(name)?;
        let view = snapshot.column_view();
        match expected {
            None => expected = Some(view.length()),
            Some(length) if view.length() != length => {
                return Err(SVError::MismatchedLengths {
                    name: (*name).to_string(),
                    expected: length,
                    actual: view.length(),
                });
            }
            Some(_) => {}
        }
        representation = view.representation();
    }
    Ok((expected.unwrap_or(0), representation))
}
