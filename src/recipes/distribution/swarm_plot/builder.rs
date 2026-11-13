//! The `SVSwarmPlot` builder: option surface, defaults and data binding.

use super::missing_value_policy::SVSwarmMissingValuePolicy;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::{SVBarOrientation, SVDotShape};

/// Builds a [`SVSwarmPlot`](super::SVSwarmPlot).
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVSwarmPlotBuilder::build), so a swarm that exists can render.
#[derive(Clone)]
pub struct SVSwarmPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) value: Option<String>,
    pub(super) category: Option<String>,
    pub(super) series: Option<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) marker: SVDotShape,
    pub(super) size: Option<SVLength>,
    pub(super) orientation: SVBarOrientation,
    pub(super) swarm_width: Option<f32>,
    pub(super) alpha: f32,
    pub(super) missing_value_policy: SVSwarmMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl<'data> SVSwarmPlotBuilder<'data> {
    /// Starts a swarm with its documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            value: None,
            category: None,
            series: None,
            series_labels: Vec::new(),
            marker: SVDotShape::Circle,
            size: None,
            orientation: SVBarOrientation::Vertical,
            swarm_width: None,
            alpha: 1.0,
            missing_value_policy: SVSwarmMissingValuePolicy::default(),
            title: None,
            x_label: None,
            y_label: None,
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

    /// Names the required column of categories.
    #[must_use]
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Names the optional column that groups the dots by colour.
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

    /// Selects the marker shape.
    #[must_use]
    pub fn marker(mut self, marker: SVDotShape) -> Self {
        self.marker = marker;
        self
    }

    /// Sets the marker diameter; `None` keeps the `mark.size` token.
    #[must_use]
    pub fn size(mut self, size: SVLength) -> Self {
        self.size = Some(size);
        self
    }

    /// Chooses the axis the categories occupy.
    ///
    /// `Vertical` places the categories on the horizontal axis and swarms
    /// horizontally; `Horizontal` places them on the vertical axis and swarms
    /// vertically. This is the FDR's `orient` option, spelled out per patterns
    /// P11.
    #[must_use]
    pub fn orientation(mut self, orientation: SVBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Bounds the swarm to a fraction of the category slot, from `0.0` to `1.0`.
    ///
    /// `None` (the default) lets the swarm use the whole category slot; the
    /// fraction is validated at render time against the move kernel's minimum of
    /// two marker extents.
    #[must_use]
    pub fn swarm_width(mut self, swarm_width: f32) -> Self {
        self.swarm_width = Some(swarm_width);
        self
    }

    /// Sets the constant opacity of every dot, from `0.0` to `1.0`.
    #[must_use]
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Chooses what a non-finite observation does: dropped or rejected.
    #[must_use]
    pub fn missing_value_policy(mut self, missing_value_policy: SVSwarmMissingValuePolicy) -> Self {
        self.missing_value_policy = missing_value_policy;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the horizontal axis title.
    #[must_use]
    pub fn x_label(mut self, x_label: impl Into<String>) -> Self {
        self.x_label = Some(x_label.into());
        self
    }

    /// Sets the vertical axis title.
    #[must_use]
    pub fn y_label(mut self, y_label: impl Into<String>) -> Self {
        self.y_label = Some(y_label.into());
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

impl Default for SVSwarmPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
