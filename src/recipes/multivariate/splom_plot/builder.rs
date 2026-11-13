//! The `SVSplomPlot` builder and its validating state machine.

use super::configuration::SVSplomConfiguration;
use super::core_implementation::{SVSplomPlot, sv_probe_columns};
use super::{SVSplomDiagonal, SVSplomMissingValuePolicy, SVSplomScaleMode};
use crate::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use crate::sv_data::SVDataSource;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::{SVDotShape, SVMark, SVMarkKind, SVMarkTheme};
use std::marker::PhantomData;

/// The absolute marker size the mark token declares.
pub(super) const SV_SPLOM_MINIMUM_VARIABLES: usize = 2;

/// Builds a [`SVSplomPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVSplomPlotBuilder::build), so a matrix that exists can render.
#[derive(Clone)]
pub struct SVSplomPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) variables: Vec<String>,
    pub(super) colour: Option<String>,
    pub(super) diagonal: SVSplomDiagonal,
    pub(super) marker: SVDotShape,
    pub(super) size: Option<SVLength>,
    pub(super) opacity: f32,
    pub(super) scale_mode: SVSplomScaleMode,
    pub(super) cell_aspect: f64,
    pub(super) show_upper: bool,
    pub(super) show_lower: bool,
    pub(super) show_diagonal: bool,
    pub(super) missing_value_policy: SVSplomMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) states: PhantomData<&'data ()>,
}

impl<'data> SVSplomPlotBuilder<'data> {
    /// Starts a matrix with no data source, no variables and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            variables: Vec::new(),
            colour: None,
            diagonal: SVSplomDiagonal::Histogram,
            marker: SVDotShape::Circle,
            size: None,
            opacity: 1.0,
            scale_mode: SVSplomScaleMode::Independent,
            cell_aspect: 1.0,
            show_upper: true,
            show_lower: true,
            show_diagonal: true,
            missing_value_policy: SVSplomMissingValuePolicy::Omit,
            title: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
            states: PhantomData,
        }
    }

    /// Validates the configuration and builds the matrix.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] for a required step,
    /// [`SVError::InvalidConfiguration`] for too few, duplicated or malformed
    /// variables and for an impossible marker size, opacity or cell aspect,
    /// [`SVError::ColumnNotFound`] for a column the source does not expose,
    /// [`SVError::MismatchedLengths`] when the columns disagree and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw the marks the
    /// configured diagonal needs.
    pub fn build(self) -> Result<SVSplomPlot<'data>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;

        sv_validate_variables(&self.variables)?;
        sv_validate_cell_aspect(self.cell_aspect)?;
        sv_validate_opacity(self.opacity)?;

        let base_marker_size = SVMarkTheme::default().mark_size();
        let size = self
            .size
            .unwrap_or_else(|| SVLength::absolute(base_marker_size));
        let dot: SVMark = SVMark::dot()
            .shape(self.marker)
            .size(size)
            .opacity(self.opacity)
            .into();
        dot.validate()?;
        sv_verify_mark_supported(SVMarkKind::Dot, backend_kind)?;
        sv_verify_diagonal_supported(self.diagonal, self.show_diagonal, backend_kind)?;

        let mut names: Vec<&str> = self.variables.iter().map(String::as_str).collect();
        names.extend(self.colour.as_deref());
        let (number_of_observations, representation) = sv_probe_columns(data_source, &names)?;

        Ok(SVSplomPlot {
            configuration: SVSplomConfiguration {
                data_source,
                variables: self.variables,
                colour: self.colour,
                diagonal: self.diagonal,
                marker: self.marker,
                size,
                opacity: self.opacity,
                scale_mode: self.scale_mode,
                cell_aspect: self.cell_aspect,
                show_upper: self.show_upper,
                show_lower: self.show_lower,
                show_diagonal: self.show_diagonal,
                missing_value_policy: self.missing_value_policy,
                title: self.title,
                backend_kind,
                execution_mode: self.execution_mode,
                number_of_observations,
                representation,
            },
        })
    }
}

impl Default for SVSplomPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_validate_variables(variables: &[String]) -> Result<(), SVError> {
    if variables.len() < SV_SPLOM_MINIMUM_VARIABLES {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "SVSplomPlot needs at least {SV_SPLOM_MINIMUM_VARIABLES} variables; received {}",
                variables.len()
            ),
        });
    }
    for (index, variable) in variables.iter().enumerate() {
        if variable.is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: format!("the variable at position {index} has an empty column name"),
            });
        }
        if variables[..index].contains(variable) {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the variable `{variable}` is listed twice; a matrix plots each pair once"
                ),
            });
        }
    }
    Ok(())
}

fn sv_validate_cell_aspect(cell_aspect: f64) -> Result<(), SVError> {
    if cell_aspect.is_finite() && cell_aspect > 0.0 {
        return Ok(());
    }
    Err(SVError::InvalidConfiguration {
        message: format!(
            "the cell aspect must be a finite positive width/height ratio; received {cell_aspect}"
        ),
    })
}

fn sv_validate_opacity(opacity: f32) -> Result<(), SVError> {
    if opacity.is_finite() && (0.0..=1.0).contains(&opacity) {
        return Ok(());
    }
    Err(SVError::InvalidConfiguration {
        message: format!("the opacity must be between 0.0 and 1.0; received {opacity}"),
    })
}

fn sv_verify_diagonal_supported(
    diagonal: SVSplomDiagonal,
    show_diagonal: bool,
    backend_kind: SVBackendKind,
) -> Result<(), SVError> {
    if !show_diagonal {
        return Ok(());
    }
    match diagonal {
        SVSplomDiagonal::Histogram => sv_verify_mark_supported(SVMarkKind::Bar, backend_kind),
        SVSplomDiagonal::Density => sv_verify_mark_supported(SVMarkKind::Area, backend_kind),
        SVSplomDiagonal::Empty => Ok(()),
    }
}
