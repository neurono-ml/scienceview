//! The built step plot value and its read-only surface.

use super::super::builder::SVStepPlotBuilder;
use super::super::options::{SVStepDuplicatePolicy, SVStepPosition};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnView, SVDataSource};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::SVDashPattern;
use scienceview_core::sv_scale::SVScaleKind;
use scienceview_core::sv_stat::SVStat;

/// A step plot ready to render.
///
/// Built only by [`SVStepPlotBuilder`]; the direct constructor is private, so
/// the "backend is explicit" rule cannot be bypassed.
pub struct SVStepPlot<'source> {
    pub(crate) data_source: &'source dyn SVDataSource,
    pub(crate) x: String,
    pub(crate) y: String,
    pub(crate) series: Option<String>,
    pub(crate) step_position: SVStepPosition,
    pub(crate) duplicate_policy: SVStepDuplicatePolicy,
    pub(crate) line_width: Option<SVLength>,
    pub(crate) dash: Option<SVDashPattern>,
    pub(crate) markers: bool,
    pub(crate) aggregate: Option<SVStat>,
    pub(crate) viewport: SVViewport,
    pub(crate) x_scale_kind: SVScaleKind,
    pub(crate) title: Option<String>,
    pub(crate) backend_kind: SVBackendKind,
    pub(crate) execution_mode: SVExecutionMode,
}

impl SVStepPlot<'_> {
    /// Starts building a step plot.
    pub fn builder() -> SVStepPlotBuilder<'static> {
        SVStepPlotBuilder::new()
    }
}

impl SVStepPlot<'_> {
    /// The configured backend kind, as a runtime value.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// The execution mode the builder selected.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.execution_mode
    }

    /// The configured step position.
    pub fn step_position(&self) -> SVStepPosition {
        self.step_position
    }

    /// The configured duplicate-`x` policy.
    pub fn duplicate_policy(&self) -> SVStepDuplicatePolicy {
        self.duplicate_policy
    }

    /// The explicit line width, or `None` when the token applies.
    pub fn line_width(&self) -> Option<SVLength> {
        self.line_width
    }

    /// The configured dash pattern, or `None` for a solid stroke.
    pub fn dash(&self) -> Option<&SVDashPattern> {
        self.dash.as_ref()
    }

    /// Whether a marker is drawn at every sample.
    pub fn markers(&self) -> bool {
        self.markers
    }

    /// The configured aggregate stat, when duplicate `x` values are collapsed.
    pub fn aggregate(&self) -> Option<&SVStat> {
        self.aggregate.as_ref()
    }

    /// The logical canvas size.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// The kind of the horizontal scale.
    pub fn x_scale_kind(&self) -> SVScaleKind {
        self.x_scale_kind
    }

    /// The figure title, when one was configured.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Resolves the three source columns once and runs `operation` on them.
    pub(crate) fn sv_resolve<Output>(
        &self,
        operation: impl FnOnce(
            SVColumnView<'_>,
            SVColumnView<'_>,
            Option<SVColumnView<'_>>,
        ) -> Result<Output, SVError>,
    ) -> Result<Output, SVError> {
        let mut x_snapshot = self.data_source.column(&self.x)?;
        let mut y_snapshot = self.data_source.column(&self.y)?;
        let mut series_snapshot = match self.series.as_deref() {
            Some(name) => Some(self.data_source.column(name)?),
            None => None,
        };
        let x_view = x_snapshot.column_view();
        let y_view = y_snapshot.column_view();
        let series_view = series_snapshot
            .as_mut()
            .map(|snapshot| snapshot.column_view());
        operation(x_view, y_view, series_view)
    }
}
