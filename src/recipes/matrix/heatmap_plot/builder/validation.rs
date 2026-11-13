//! Validation and assembly of the built heatmap.

use super::builder_state::SVHeatmapPlotBuilder;
use crate::sv_backend::sv_verify_mark_supported;
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkKind;
use crate::sv_recipes::sv_matrix::sv_heatmap_plot::SVHeatmapMatrix;
use crate::sv_recipes::sv_matrix::sv_heatmap_plot::configuration::{
    SVHeatmapConfiguration, SVHeatmapInput,
};
use crate::sv_recipes::sv_matrix::sv_heatmap_plot::core_implementation::SVHeatmapPlot;

impl<'data> SVHeatmapPlotBuilder<'data> {
    /// Validates the configuration and builds the heatmap.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingBackendSelection`] without a backend,
    /// [`SVError::InvalidConfiguration`] for an incomplete or contradictory
    /// input, a non-finite option or mismatched labels, and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw cells.
    pub fn build(self) -> Result<SVHeatmapPlot<'data>, SVError> {
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        sv_verify_mark_supported(SVMarkKind::Image, backend_kind)?;
        sv_validate_line_width(self.line_width)?;
        sv_validate_centre(self.centre)?;

        let (input, representation, number_of_cells) = match self.matrix {
            Some(matrix) => {
                sv_validate_matrix_labels(matrix, self.row_labels, self.column_labels)?;
                (
                    SVHeatmapInput::Matrix(matrix),
                    crate::sv_data::SVRepresentation::Dense,
                    matrix.number_of_rows() * matrix.number_of_columns(),
                )
            }
            None => {
                let data_source = self.data_source.ok_or_else(|| SVError::InvalidConfiguration {
                    message: "SVHeatmapPlot requires either `matrix(...)` or `data_source(...)` with `row(...)`, `column(...)` and `value(...)`".to_string(),
                })?;
                let row = sv_required_column(self.row, "row")?;
                let column = sv_required_column(self.column, "column")?;
                let value = sv_required_column(self.value, "value")?;
                let (number_of_cells, representation) =
                    crate::sv_recipes::sv_matrix::sv_heatmap_plot::core_implementation::sv_probe_columns(
                        data_source,
                        &[row, column, value],
                    )?;
                (
                    SVHeatmapInput::Table {
                        data_source,
                        row,
                        column,
                        value,
                    },
                    representation,
                    number_of_cells,
                )
            }
        };

        Ok(SVHeatmapPlot {
            configuration: SVHeatmapConfiguration {
                input,
                row_labels: self.row_labels,
                column_labels: self.column_labels,
                colour_map: self.colour_map,
                centre: self.centre,
                robust: self.robust,
                annot: self.annot,
                annot_format: self.annot_format,
                line_width: self.line_width,
                square: self.square,
                colour_bar: self.colour_bar,
                missing: self.missing,
                orientation: self.orientation,
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
            "SVHeatmapPlot requires the `{role}` column of a long table; pass it with `{role}(...)`"
        ),
    })
}

fn sv_validate_line_width(line_width: f64) -> Result<(), SVError> {
    if !line_width.is_finite() || line_width < 0.0 {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "the cell border width must be a finite non-negative number, received {line_width}"
            ),
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
    matrix: SVHeatmapMatrix<'_>,
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
