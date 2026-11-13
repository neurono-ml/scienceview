//! The built line plot: configuration in, composed scene out.

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind, SVRenderOutput};
use scienceview_core::sv_data::{SVDataSource, SVRepresentation};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::SVDashPattern;
use scienceview_core::sv_observability::sv_run_operation;
use scienceview_core::sv_scale::{SVScaleKind, SVTemporalUnit};
use scienceview_core::sv_stat::SVAggregateOptions;
use std::path::Path;

use super::SVLineInterpolation;
use super::SVLinePlotBuilder;
use super::composition::{SVLinePlotComposition, sv_compose_line_plot};
use super::options::SVLegendPlacement;
use super::series::{sv_build_series, sv_number_of_drawn_marks};

/// A line plot: one line per series over an ordered axis.
///
/// Built only by [`SVLinePlotBuilder`]. The chart borrows its data source and
/// the names of its columns, and it resolves the source on each operation, so
/// a streaming source is read from its current window without blocking the
/// render thread.
pub struct SVLinePlot<'values> {
    pub(super) data_source: &'values dyn SVDataSource,
    pub(super) x_name: &'values str,
    pub(super) y_name: &'values str,
    pub(super) series_name: Option<&'values str>,
    pub(super) group_name: Option<&'values str>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) title: Option<String>,
    pub(super) x_axis_title: Option<String>,
    pub(super) y_axis_title: Option<String>,
    pub(super) line_width: SVLength,
    pub(super) dash: Option<SVDashPattern>,
    pub(super) markers: bool,
    pub(super) interpolation: SVLineInterpolation,
    pub(super) sort_by_x: bool,
    pub(super) bridge_missing: bool,
    pub(super) aggregate: Option<SVAggregateOptions>,
    pub(super) legend: SVLegendPlacement,
    pub(super) x_scale_kind: SVScaleKind,
    pub(super) temporal_unit: SVTemporalUnit,
    pub(super) representation: SVRepresentation,
}

impl<'values> SVLinePlot<'values> {
    pub(super) fn from_builder(
        builder: SVLinePlotBuilder<'values>,
        data_source: &'values dyn SVDataSource,
        backend_kind: SVBackendKind,
        x_name: &'values str,
        y_name: &'values str,
        representation: SVRepresentation,
    ) -> Self {
        Self {
            data_source,
            x_name,
            y_name,
            series_name: builder.series,
            group_name: builder.group,
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
            x_scale_kind: builder.x_scale_kind,
            temporal_unit: builder.temporal_unit,
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

    /// Returns the number of logical observations in the `x` column.
    pub fn point_count(&self) -> usize {
        self.data_source
            .column(self.x_name)
            .map(|mut snapshot| snapshot.column_view().length())
            .unwrap_or(0)
    }

    /// Returns `true` when markers are drawn on the observations.
    pub fn markers_enabled(&self) -> bool {
        self.markers
    }

    /// Returns the configured line width.
    pub fn line_width(&self) -> SVLength {
        self.line_width
    }

    /// Returns the explicitly configured dash pattern, when there is one.
    pub fn dash(&self) -> Option<&SVDashPattern> {
        self.dash.as_ref()
    }

    /// Returns the configured interpolation.
    pub fn interpolation(&self) -> SVLineInterpolation {
        self.interpolation
    }

    /// Returns where the legend is placed.
    pub fn legend_placement(&self) -> SVLegendPlacement {
        self.legend
    }

    /// Returns the kind of the horizontal scale.
    pub fn x_scale_kind(&self) -> SVScaleKind {
        self.x_scale_kind
    }

    /// Composes the chart: series, scales, mark geometry and decorations.
    ///
    /// The operation runs inside one observability span carrying the backend
    /// kind, the representation and the number of drawn marks — not the
    /// logical input length, which aggregation would overstate.
    ///
    /// # Errors
    ///
    /// Returns the kernel's [`SVError`] when a column is rejected or a mark
    /// cannot be built.
    pub fn compose(&self) -> Result<SVLinePlotComposition, SVError> {
        let series = sv_build_series(self)?;
        let drawn_marks = sv_number_of_drawn_marks(&series);
        sv_run_operation(
            "compose",
            self.backend_kind,
            self.representation,
            drawn_marks,
            || sv_compose_line_plot(self, series),
        )
    }

    /// Renders the chart with the configured backend.
    ///
    /// The SVG and native backends are the recipe's rendering targets. The
    /// composition is backend-agnostic: the SVG path serializes it as a
    /// document and the native path rasterizes the same scene into a frame.
    /// Without the matching Cargo feature the operation reports the backend as
    /// unavailable instead of degrading silently.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnsupportedBackendCapability`] for a backend the
    /// recipe does not render yet and [`SVError::BackendUnavailable`] when the
    /// backend's feature is disabled.
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        let composition = sv_compose_without_span(self)?;
        let drawn_marks = composition.number_of_drawn_marks();
        sv_run_operation(
            "render",
            self.backend_kind,
            self.representation,
            drawn_marks,
            || sv_render_composition(self, &composition),
        )
    }

    /// Rasterizes the same composed scene with the native backend.
    ///
    /// This is the comparable-frame surface: the SVG document and this frame
    /// come from one composition, so a test can compare them without
    /// re-deriving any position.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnsupportedBackendCapability`] when the configured
    /// backend is not native, and [`SVError::BackendUnavailable`] when the
    /// `render-native` feature is disabled.
    #[cfg(feature = "render-native")]
    pub fn render_frame(&self) -> Result<scienceview_native::SVNativeFrame, SVError> {
        if self.backend_kind != SVBackendKind::Native {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind: self.backend_kind,
                capability: SVBackendCapability::Export,
            });
        }
        let composition = sv_compose_without_span(self)?;
        let drawn_marks = composition.number_of_drawn_marks();
        sv_run_operation(
            "render",
            self.backend_kind,
            self.representation,
            drawn_marks,
            || composition.to_native_frame(),
        )
    }

    /// Writes the rendered chart to `destination`.
    ///
    /// Only the SVG backend exports a document; the native backend is a frame
    /// renderer without a file format, so it reports the missing capability.
    ///
    /// # Errors
    ///
    /// Returns the error of [`SVLinePlot::render`] and
    /// [`SVError::BackendFailure`] when the destination cannot be written.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        if self.backend_kind != SVBackendKind::Svg {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind: self.backend_kind,
                capability: SVBackendCapability::Export,
            });
        }
        let output = self.render()?;
        std::fs::write(destination, output.bytes).map_err(|error| SVError::BackendFailure {
            backend_kind: self.backend_kind,
            operation: "save",
            message: format!("cannot write {}: {error}", destination.display()),
        })
    }
}

fn sv_compose_without_span(plot: &SVLinePlot<'_>) -> Result<SVLinePlotComposition, SVError> {
    let series = sv_build_series(plot)?;
    sv_compose_line_plot(plot, series)
}

fn sv_render_composition(
    plot: &SVLinePlot<'_>,
    composition: &SVLinePlotComposition,
) -> Result<SVRenderOutput, SVError> {
    match plot.backend_kind() {
        SVBackendKind::Svg => sv_render_svg(plot, composition),
        SVBackendKind::Native => sv_render_native(plot, composition),
        backend_kind => Err(SVError::UnsupportedBackendCapability {
            backend_kind,
            capability: SVBackendCapability::Export,
        }),
    }
}

#[cfg(feature = "render-svg")]
fn sv_render_svg(
    plot: &SVLinePlot<'_>,
    composition: &SVLinePlotComposition,
) -> Result<SVRenderOutput, SVError> {
    Ok(SVRenderOutput {
        backend_kind: plot.backend_kind(),
        bytes: composition.to_svg()?.into_bytes(),
    })
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(
    plot: &SVLinePlot<'_>,
    _composition: &SVLinePlotComposition,
) -> Result<SVRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: plot.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(
    plot: &SVLinePlot<'_>,
    composition: &SVLinePlotComposition,
) -> Result<SVRenderOutput, SVError> {
    Ok(SVRenderOutput {
        backend_kind: plot.backend_kind(),
        bytes: composition.to_native_frame()?.into_pixels(),
    })
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(
    plot: &SVLinePlot<'_>,
    _composition: &SVLinePlotComposition,
) -> Result<SVRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: plot.backend_kind(),
    })
}
