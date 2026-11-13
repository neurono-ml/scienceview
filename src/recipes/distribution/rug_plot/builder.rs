//! The `SVRugPlot` builder: option surface, defaults and data binding.

use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::SVBarOrientation;

/// Builds a [`SVRugPlot`](super::SVRugPlot).
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVRugPlotBuilder::build), so a rug that exists can render.
#[derive(Clone)]
pub struct SVRugPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) value: Option<String>,
    pub(super) series: Option<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) orientation: SVBarOrientation,
    pub(super) height: f32,
    pub(super) alpha: f32,
    pub(super) line_width: Option<SVLength>,
    pub(super) title: Option<String>,
    pub(super) value_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl<'data> SVRugPlotBuilder<'data> {
    /// Starts a rug with its documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            value: None,
            series: None,
            series_labels: Vec::new(),
            orientation: SVBarOrientation::Vertical,
            height: 0.05,
            alpha: 1.0,
            line_width: None,
            title: None,
            value_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }

    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the required column of observed values.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Names the optional column that groups the ticks by colour.
    #[must_use]
    pub fn series(mut self, series: impl Into<String>) -> Self {
        self.series = Some(series.into());
        self
    }

    /// Supplies human labels for the series values, in first-appearance order.
    #[must_use]
    pub fn series_labels(mut self, labels: &[&str]) -> Self {
        self.series_labels = labels.iter().map(|label| (*label).to_string()).collect();
        self
    }

    /// Chooses the axis the values occupy and therefore the margin of the ticks.
    ///
    /// `Vertical` places the values on the horizontal axis and draws vertical
    /// ticks in the bottom margin; `Horizontal` places them on the vertical
    /// axis and draws horizontal ticks in the left margin. This is the FDR's
    /// `orient` option, spelled out per patterns P11.
    #[must_use]
    pub fn orientation(mut self, orientation: SVBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the tick length as a fraction of the plot's perpendicular extent.
    ///
    /// The length is capped at the depth of the axis margin, so a tick never
    /// enters the plot rectangle even at `height = 1.0`.
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Sets the constant opacity of every tick, from `0.0` to `1.0`.
    #[must_use]
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Sets the tick stroke width; `None` keeps the `mark.line_width` token.
    #[must_use]
    pub fn line_width(mut self, line_width: SVLength) -> Self {
        self.line_width = Some(line_width);
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the title of the axis the values occupy.
    #[must_use]
    pub fn value_label(mut self, value_label: impl Into<String>) -> Self {
        self.value_label = Some(value_label.into());
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

impl Default for SVRugPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
