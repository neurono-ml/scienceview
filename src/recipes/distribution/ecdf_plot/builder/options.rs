//! The fluent option surface; validation happens in `build`.

use super::definition::SVECdfPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::SVBarOrientation;

impl SVECdfPlotBuilder<'_> {
    /// Names the observed continuous column the curve describes.
    #[must_use]
    pub fn value(mut self, column: impl Into<String>) -> Self {
        self.value = Some(column.into());
        self
    }

    /// Names the nominal column that splits the chart into one ECDF per group.
    #[must_use]
    pub fn series(mut self, column: impl Into<String>) -> Self {
        self.series = Some(column.into());
        self
    }

    /// Chooses between probabilities (`true`, default) and cumulative counts.
    #[must_use]
    pub fn normalise(mut self, normalise: bool) -> Self {
        self.normalise = normalise;
        self
    }

    /// Chooses the orientation of the two channels.
    ///
    /// Vertical keeps the observed value on the horizontal axis and the
    /// cumulative probability on the vertical one; horizontal swaps them, as
    /// in the seaborn `orient` option.
    #[must_use]
    pub fn orient(mut self, orient: SVBarOrientation) -> Self {
        self.orient = orient;
        self
    }

    /// Draws the complementary (survival) curve `1 - F(x)` instead of `F(x)`.
    ///
    /// With `normalise(false)` the complementary curve descends from the total
    /// observation count. Off by default.
    #[must_use]
    pub fn complementary(mut self, complementary: bool) -> Self {
        self.complementary = complementary;
        self
    }

    /// Sets the stroke width; `None` keeps the `mark.line_width` token.
    #[must_use]
    pub fn line_width(mut self, width: SVLength) -> Self {
        self.line_width = Some(width);
        self
    }

    /// Sets the logical canvas size.
    #[must_use]
    pub fn viewport(mut self, viewport: SVViewport) -> Self {
        self.viewport = viewport;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Selects the rendering backend. Required: there is no automatic backend.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Selects the execution mode. Defaults to [`SVExecutionMode::Automatic`].
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}
