//! The built line plot: configuration in, composed chart out.

use scienceview_core::sv_backend::{SVBackendKind, SVRenderOutput};
use scienceview_core::sv_data::{SVColumnView, SVRepresentation};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::SVDashPattern;
use scienceview_core::sv_observability::sv_run_operation;
use scienceview_core::sv_stat::SVAggregateOptions;
use std::path::Path;

use super::SVLineInterpolation;
use super::SVLinePlotBuilder;
use super::composition::{SVLinePlotComposition, sv_compose_line_plot};

/// A line plot: one line per series over an ordered axis.
///
/// Built only by [`SVLinePlotBuilder`]. The chart carries its data columns, its
/// validated options and the configured backend kind, and it resolves the
/// composition on each operation, so a streaming source is read from its
/// current window without blocking the render thread.
pub struct SVLinePlot<'values> {
    pub(super) x: SVColumnView<'values>,
    pub(super) y: SVColumnView<'values>,
    pub(super) series: Option<SVColumnView<'values>>,
    pub(super) group: Option<SVColumnView<'values>>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) title: Option<String>,
    pub(super) x_axis_title: Option<String>,
    pub(super) y_axis_title: Option<String>,
    pub(super) line_width: SVLength,
    pub(super) dash: SVDashPattern,
    pub(super) markers: bool,
    pub(super) interpolation: SVLineInterpolation,
    pub(super) sort_by_x: bool,
    pub(super) bridge_missing: bool,
    pub(super) aggregate: Option<SVAggregateOptions>,
    pub(super) legend: bool,
    pub(super) representation: SVRepresentation,
}

impl<'values> SVLinePlot<'values> {
    pub(super) fn from_builder(
        builder: SVLinePlotBuilder<'values>,
        backend_kind: SVBackendKind,
        x: SVColumnView<'values>,
        y: SVColumnView<'values>,
        representation: SVRepresentation,
    ) -> Self {
        Self {
            x,
            y,
            series: builder.series,
            group: builder.group,
            backend_kind,
            execution_mode: builder.execution_mode,
            title: builder.title,
            x_axis_title: builder.x_axis_title,
            y_axis_title: builder.y_axis_title,
            line_width: builder.line_width,
            dash: builder.dash,
            markers: builder.markers,
            interpolation: builder.interpolation,
            sort_by_x: builder.sort_by_x,
            bridge_missing: builder.bridge_missing,
            aggregate: builder.aggregate,
            legend: builder.legend,
            representation,
        }
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the execution mode the builder selected.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.execution_mode
    }

    /// Returns the dominant representation of the bound columns.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// Returns the number of logical observations across all columns.
    pub fn point_count(&self) -> usize {
        self.x.length()
    }

    /// Returns `true` when markers are drawn on the observations.
    pub fn markers_enabled(&self) -> bool {
        self.markers
    }

    /// Returns the configured line width.
    pub fn line_width(&self) -> SVLength {
        self.line_width
    }

    /// Returns the configured dash pattern.
    pub fn dash(&self) -> &SVDashPattern {
        &self.dash
    }

    /// Returns the configured interpolation.
    pub fn interpolation(&self) -> SVLineInterpolation {
        self.interpolation
    }

    /// Composes the chart: series, scales, mark geometry and decorations.
    ///
    /// The operation runs inside one observability span carrying the backend
    /// kind, the representation and the point count.
    ///
    /// # Errors
    ///
    /// Returns the kernel's [`SVError`] when a column is rejected or a mark
    /// cannot be built.
    pub fn compose(&self) -> Result<SVLinePlotComposition, SVError> {
        sv_run_operation(
            "compose",
            self.backend_kind,
            self.representation,
            self.point_count(),
            || sv_compose_line_plot(self),
        )
    }

    /// Renders the chart with the configured backend.
    ///
    /// The SVG backend is the recipe's first rendering target. Without the
    /// `render-svg` feature the operation reports the backend as unavailable
    /// instead of degrading silently.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnsupportedBackendCapability`] for a backend the
    /// recipe does not render yet and [`SVError::BackendUnavailable`] when the
    /// SVG feature is disabled.
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        sv_run_operation(
            "render",
            self.backend_kind,
            self.representation,
            self.point_count(),
            || {
                if self.backend_kind != SVBackendKind::Svg {
                    return Err(SVError::UnsupportedBackendCapability {
                        backend_kind: self.backend_kind,
                        capability: scienceview_core::sv_backend::SVBackendCapability::Export,
                    });
                }
                sv_render_line_plot(self)
            },
        )
    }

    /// Writes the rendered chart to `destination`.
    ///
    /// # Errors
    ///
    /// Returns the error of [`SVLinePlot::render`] and
    /// [`SVError::BackendFailure`] when the destination cannot be written.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        let output = self.render()?;
        std::fs::write(destination, output.bytes).map_err(|error| SVError::BackendFailure {
            backend_kind: self.backend_kind,
            operation: "save",
            message: format!("cannot write {}: {error}", destination.display()),
        })
    }
}

#[cfg(feature = "render-svg")]
fn sv_render_line_plot(plot: &SVLinePlot<'_>) -> Result<SVRenderOutput, SVError> {
    let document = super::svg_output::sv_line_plot_document(plot)?;
    Ok(SVRenderOutput {
        backend_kind: plot.backend_kind(),
        bytes: document.into_bytes(),
    })
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_line_plot(plot: &SVLinePlot<'_>) -> Result<SVRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: plot.backend_kind(),
    })
}
