//! The built area plot and the operations it performs.
//!
//! The built value carries its configured backend kind and never dispatches on
//! it in `build` (patterns.md P2). Every public operation dispatches once,
//! records its span and composes the figure from the core kernels.

use std::path::Path;

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind, SVRenderOutput};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_mark::{SVBaselineMode, SVMarkTheme};
use scienceview_core::sv_observability::sv_run_operation;

use super::builder::SVAreaPlotBuilder;
use super::configuration::{SVAreaInterpolation, SVAreaPlotConfiguration};
use super::missing_value_policy::SVAreaMissingValuePolicy;
use super::preparation::sv_prepare_area;
use super::rendering::{SVAreaRendering, sv_compose_area};

/// A relational area chart: the region between a line and a baseline.
///
/// Built only by [`SVAreaPlotBuilder`]; the direct constructor is private, so
/// the explicit backend rule cannot be bypassed.
pub struct SVAreaPlot<'values> {
    configuration: SVAreaPlotConfiguration<'values>,
    backend_kind: SVBackendKind,
    execution_mode: SVExecutionMode,
}

impl<'values> SVAreaPlot<'values> {
    /// Starts building an area plot.
    ///
    /// The builder type is the only constructor; the method is named `new`
    /// because it starts the chart's own builder, not because it returns the
    /// built chart.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> SVAreaPlotBuilder<'values, scienceview_core::sv_figure::SVWithoutBackend> {
        SVAreaPlotBuilder::new()
    }

    pub(super) fn from_configuration(
        configuration: SVAreaPlotConfiguration<'values>,
        backend_kind: SVBackendKind,
        execution_mode: SVExecutionMode,
    ) -> Self {
        Self {
            configuration,
            backend_kind,
            execution_mode,
        }
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the selected execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.execution_mode
    }

    /// Returns the horizontal column name.
    pub fn x_column_name(&self) -> &str {
        self.configuration.x
    }

    /// Returns the vertical column name.
    pub fn y_column_name(&self) -> &str {
        self.configuration.y
    }

    /// Returns the optional series column name.
    pub fn series_column_name(&self) -> Option<&str> {
        self.configuration.series
    }

    /// Returns whether the areas are stacked.
    pub fn stack_enabled(&self) -> bool {
        self.configuration.stack
    }

    /// Returns the configured baseline of a non-stacked area.
    pub fn baseline_value(&self) -> SVBaselineMode {
        self.configuration.baseline
    }

    /// Returns the explicit opacity, or `None` when the token applies.
    pub fn opacity_value(&self) -> Option<f32> {
        self.configuration.opacity
    }

    /// Returns whether the boundary line is drawn.
    pub fn line_enabled(&self) -> bool {
        self.configuration.line
    }

    /// Returns the interpolation of the ordered boundary.
    pub fn interpolation_value(&self) -> SVAreaInterpolation {
        self.configuration.interpolation
    }

    /// Returns the missing-value policy in force.
    pub fn missing_value_policy_value(&self) -> SVAreaMissingValuePolicy {
        self.configuration.missing_value_policy
    }

    /// Returns the theme the composed marks use, when one was set.
    pub fn theme_value(&self) -> Option<&SVMarkTheme> {
        self.configuration.theme.as_ref()
    }

    /// Composes the figure into backend-agnostic primitives.
    ///
    /// This is the public surface a backend serializer consumes: the result
    /// carries the ordered primitives and needs no backend dispatch. The call
    /// emits the same `render` span as [`SVAreaPlot::render`], recording the
    /// configured backend kind, the representation and the number of points.
    ///
    /// # Errors
    ///
    /// Returns the data-boundary, move, scale, decoration or mark error that
    /// prevented the figure from being composed.
    pub fn rendering(&self) -> Result<SVAreaRendering, SVError> {
        let prepared = sv_prepare_area(&self.configuration)?;
        sv_run_operation(
            "render",
            self.backend_kind,
            prepared.representation,
            prepared.point_count,
            || sv_compose_area(&self.configuration, prepared),
        )
    }

    /// Renders the figure with the configured backend.
    ///
    /// The operation composes the scene once and hands it to the serializer
    /// selected by the configured kind: the SVG document or the native raster
    /// frame. The kind is never ignored, and a kind the recipe has no
    /// serializer for reports the missing capability instead of receiving
    /// bytes another backend produced.
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::UnsupportedBackendCapability`]
    /// for a kind without a recipe serializer, and
    /// [`SVError::BackendUnavailable`] when the selected serializer's feature
    /// is not compiled.
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        let rendering = self.rendering()?;
        sv_serialize(self.backend_kind, &rendering)
    }

    /// Writes the figure to `destination`.
    ///
    /// Only the SVG backend exports a document. The native backend presents a
    /// frame instead, so it reports the missing export capability rather than
    /// writing raw pixels under a document extension.
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::UnsupportedBackendCapability`]
    /// for a backend without an export format, and [`SVError::BackendFailure`]
    /// when the file cannot be written.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        let prepared = sv_prepare_area(&self.configuration)?;
        sv_run_operation(
            "save",
            self.backend_kind,
            prepared.representation,
            prepared.point_count,
            || {
                let rendering = sv_compose_area(&self.configuration, prepared)?;
                let bytes = sv_export(self.backend_kind, &rendering)?;
                std::fs::write(destination, bytes).map_err(|error| SVError::BackendFailure {
                    backend_kind: self.backend_kind,
                    operation: "save",
                    message: format!("cannot write {}: {error}", destination.display()),
                })
            },
        )
    }

    /// Reports that the recipe has no presentation surface.
    ///
    /// A native window is opened by the native backend's own present loop; the
    /// recipe exports documents and frames, it never owns a window.
    ///
    /// # Errors
    ///
    /// Always returns [`SVError::UnsupportedBackendCapability`].
    pub fn show(&self) -> Result<(), SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: self.backend_kind,
            capability: SVBackendCapability::Presentation,
        })
    }
}

/// Dispatches the composed scene to the serializer of the configured kind.
///
/// The scene contract is backend-agnostic on purpose: [`SVAreaRendering`]
/// carries the viewport, the background, and the decoration and mark
/// primitives in draw order. A serializer only replays those lists.
fn sv_serialize(
    backend_kind: SVBackendKind,
    rendering: &SVAreaRendering,
) -> Result<SVRenderOutput, SVError> {
    let bytes = match backend_kind {
        SVBackendKind::Svg => sv_svg_bytes(rendering)?,
        SVBackendKind::Native => sv_native_bytes(rendering)?,
        other => {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind: other,
                capability: SVBackendCapability::Export,
            });
        }
    };
    Ok(SVRenderOutput {
        backend_kind,
        bytes,
    })
}

/// Returns the bytes an export writes for the configured kind.
fn sv_export(backend_kind: SVBackendKind, rendering: &SVAreaRendering) -> Result<Vec<u8>, SVError> {
    match backend_kind {
        SVBackendKind::Svg => sv_svg_bytes(rendering),
        other => Err(SVError::UnsupportedBackendCapability {
            backend_kind: other,
            capability: SVBackendCapability::Export,
        }),
    }
}

#[cfg(feature = "render-svg")]
fn sv_svg_bytes(rendering: &SVAreaRendering) -> Result<Vec<u8>, SVError> {
    Ok(sv_svg_document(rendering))
}

#[cfg(not(feature = "render-svg"))]
fn sv_svg_bytes(_rendering: &SVAreaRendering) -> Result<Vec<u8>, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: SVBackendKind::Svg,
    })
}

#[cfg(feature = "render-native")]
fn sv_native_bytes(rendering: &SVAreaRendering) -> Result<Vec<u8>, SVError> {
    let mut canvas = scienceview_native::SVRasterCanvas::new(&rendering.viewport());
    canvas.fill(rendering.background());
    canvas.draw_decoration_primitives(rendering.grid_and_frame());
    for primitive in rendering.primitives() {
        canvas.draw_mark_primitive(primitive);
    }
    canvas.draw_decoration_primitives(rendering.axes_and_title());
    Ok(canvas.to_frame().into_pixels())
}

#[cfg(not(feature = "render-native"))]
fn sv_native_bytes(_rendering: &SVAreaRendering) -> Result<Vec<u8>, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: SVBackendKind::Native,
    })
}

#[cfg(feature = "render-svg")]
fn sv_svg_document(rendering: &SVAreaRendering) -> Vec<u8> {
    let mut document = scienceview_svg::SVSvgDocument::new(rendering.viewport())
        .with_background(rendering.background());
    if let Some(title) = rendering.title() {
        document = document.with_title(title);
    }
    document.push_decoration_primitives(rendering.grid_and_frame());
    for primitive in rendering.primitives() {
        document.push_mark_primitive(primitive);
    }
    document.push_decoration_primitives(rendering.axes_and_title());
    document.to_bytes()
}
