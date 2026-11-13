//! The built ECDF plot value and its read-only surface.

use super::super::builder::SVECdfPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::SVBarOrientation;

/// An empirical cumulative distribution plot ready to render.
///
/// Built only by [`SVECdfPlotBuilder`]; the direct constructor is private, so
/// the "backend is explicit" rule cannot be bypassed.
pub struct SVECdfPlot<'source> {
    pub(crate) data_source: &'source dyn SVDataSource,
    pub(crate) value: String,
    pub(crate) series: Option<String>,
    pub(crate) normalise: bool,
    pub(crate) orient: SVBarOrientation,
    pub(crate) complementary: bool,
    pub(crate) line_width: Option<SVLength>,
    pub(crate) viewport: SVViewport,
    pub(crate) title: Option<String>,
    pub(crate) backend_kind: SVBackendKind,
    pub(crate) execution_mode: SVExecutionMode,
}

impl SVECdfPlot<'_> {
    /// Starts building an ECDF plot.
    pub fn builder() -> SVECdfPlotBuilder<'static> {
        SVECdfPlotBuilder::new()
    }
}

impl core::fmt::Debug for SVECdfPlot<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SVECdfPlot")
            .field("value", &self.value)
            .field("series", &self.series)
            .field("normalise", &self.normalise)
            .field("orient", &self.orient)
            .field("complementary", &self.complementary)
            .field("backend_kind", &self.backend_kind)
            .field("execution_mode", &self.execution_mode)
            .finish_non_exhaustive()
    }
}

impl SVECdfPlot<'_> {
    /// The configured backend kind, as a runtime value.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// The execution mode the builder selected.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.execution_mode
    }

    /// The observed column the curve describes.
    pub fn value_column(&self) -> &str {
        &self.value
    }

    /// The series column, when the chart draws one curve per group.
    pub fn series_column(&self) -> Option<&str> {
        self.series.as_deref()
    }

    /// Whether the curve is normalised to `[0, 1]` (the default).
    pub fn is_normalised(&self) -> bool {
        self.normalise
    }

    /// The configured orientation.
    pub fn orientation(&self) -> SVBarOrientation {
        self.orient
    }

    /// Whether the complementary (survival) curve is drawn.
    pub fn is_complementary(&self) -> bool {
        self.complementary
    }

    /// The explicit line width, or `None` when the `mark.line_width` token
    /// applies.
    pub fn line_width(&self) -> Option<SVLength> {
        self.line_width
    }

    /// The logical canvas size.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// The figure title, when one was configured.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}
