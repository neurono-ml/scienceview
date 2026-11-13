//! The option setters of [`SVSplomPlotBuilder`](super::SVSplomPlotBuilder).
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVSplomPlotBuilder;
use super::{SVSplomDiagonal, SVSplomMissingValuePolicy, SVSplomScaleMode};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::SVDotShape;

impl<'data> SVSplomPlotBuilder<'data> {
    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the variable columns, in matrix order.
    ///
    /// Every variable becomes one row and one column of the grid; at least two
    /// distinct names are required.
    #[must_use]
    pub fn variables(mut self, variables: &[&str]) -> Self {
        self.variables = variables
            .iter()
            .map(|variable| (*variable).to_string())
            .collect();
        self
    }

    /// Names the optional column that groups the observations by colour.
    #[must_use]
    pub fn colour(mut self, colour: impl Into<String>) -> Self {
        self.colour = Some(colour.into());
        self
    }

    /// Selects what the diagonal cells draw.
    #[must_use]
    pub fn diagonal(mut self, diagonal: SVSplomDiagonal) -> Self {
        self.diagonal = diagonal;
        self
    }

    /// Selects the marker shape of the off-diagonal cells.
    #[must_use]
    pub fn marker(mut self, marker: SVDotShape) -> Self {
        self.marker = marker;
        self
    }

    /// Sets the marker size; defaults to the `mark.size` token.
    #[must_use]
    pub fn size(mut self, size: SVLength) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the constant opacity, from `0.0` to `1.0`.
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Selects whether the axes are per-variable or common to every variable.
    #[must_use]
    pub fn scale_mode(mut self, scale_mode: SVSplomScaleMode) -> Self {
        self.scale_mode = scale_mode;
        self
    }

    /// Sets the width/height ratio of one cell; defaults to `1.0`.
    #[must_use]
    pub fn cell_aspect(mut self, cell_aspect: f64) -> Self {
        self.cell_aspect = cell_aspect;
        self
    }

    /// Enables or disables the upper triangle of the grid.
    #[must_use]
    pub fn show_upper(mut self, show_upper: bool) -> Self {
        self.show_upper = show_upper;
        self
    }

    /// Enables or disables the lower triangle of the grid.
    #[must_use]
    pub fn show_lower(mut self, show_lower: bool) -> Self {
        self.show_lower = show_lower;
        self
    }

    /// Enables or disables the diagonal cells.
    #[must_use]
    pub fn show_diagonal(mut self, show_diagonal: bool) -> Self {
        self.show_diagonal = show_diagonal;
        self
    }

    /// Selects what happens to an observation that is not finite.
    #[must_use]
    pub fn missing_value_policy(mut self, policy: SVSplomMissingValuePolicy) -> Self {
        self.missing_value_policy = policy;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Selects the rendering backend. Required.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Selects the execution mode. Defaults to `Automatic`.
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}
