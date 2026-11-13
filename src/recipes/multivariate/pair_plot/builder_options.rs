//! The option setters of [`SVPairPlotBuilder`](super::SVPairPlotBuilder).
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVPairPlotBuilder;
use super::{SVPairDiagonalKind, SVPairMissingValuePolicy, SVPairOffDiagonalKind};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVDotShape;
use crate::sv_move::SVMove;

impl<'data> SVPairPlotBuilder<'data> {
    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Sets the same variable list on both axes (the square pair grid).
    #[must_use]
    pub fn variables(mut self, variables: &[&str]) -> Self {
        self.variables = sv_names(variables);
        self
    }

    /// Sets the variables placed along the horizontal axes.
    #[must_use]
    pub fn x_variables(mut self, variables: &[&str]) -> Self {
        self.x_variables = sv_names(variables);
        self
    }

    /// Sets the variables placed along the vertical axes.
    #[must_use]
    pub fn y_variables(mut self, variables: &[&str]) -> Self {
        self.y_variables = sv_names(variables);
        self
    }

    /// Names the optional grouping column painted by colour.
    #[must_use]
    pub fn hue(mut self, hue: impl Into<String>) -> Self {
        self.hue = Some(hue.into());
        self
    }

    /// Selects the diagonal shape.
    #[must_use]
    pub fn diagonal_kind(mut self, diagonal_kind: SVPairDiagonalKind) -> Self {
        self.diagonal_kind = diagonal_kind;
        self
    }

    /// Selects the off-diagonal shape.
    #[must_use]
    pub fn off_diagonal_kind(mut self, off_diagonal_kind: SVPairOffDiagonalKind) -> Self {
        self.off_diagonal_kind = off_diagonal_kind;
        self
    }

    /// Selects the marker shape of the scatter cells.
    #[must_use]
    pub fn marker(mut self, marker: SVDotShape) -> Self {
        self.marker = marker;
        self
    }

    /// Selects what a non-finite observation does.
    #[must_use]
    pub fn missing_value_policy(mut self, missing_value_policy: SVPairMissingValuePolicy) -> Self {
        self.missing_value_policy = missing_value_policy;
        self
    }

    /// Sets the constant opacity of the marks, from `0.0` to `1.0`.
    #[must_use]
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Applies a displacement move inside every off-diagonal cell.
    ///
    /// The grid accepts the displacement moves `Jitter` and `Dodge`, matching
    /// the scatter recipe; the displacement is keyed by the hue group.
    #[must_use]
    pub fn jitter(mut self, jitter: SVMove) -> Self {
        self.jitter = Some(jitter);
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

fn sv_names(variables: &[&str]) -> Vec<String> {
    variables.iter().map(|name| (*name).to_string()).collect()
}
