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
use super::missing_value_policy::SVMissingValuePolicy;
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
    pub fn missing_value_policy_value(&self) -> SVMissingValuePolicy {
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
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the crate was built
    /// without the `render-svg` feature, and the composition error otherwise.
    #[cfg(feature = "render-svg")]
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        let rendering = self.rendering()?;

        Ok(SVRenderOutput {
            backend_kind: self.backend_kind,
            bytes: sv_svg_document(&rendering),
        })
    }

    /// Renders the figure with the configured backend.
    ///
    /// # Errors
    ///
    /// Always returns [`SVError::InvalidConfiguration`] when the crate is built
    /// without the `render-svg` feature: the recipe composes primitives, and a
    /// serializer must be enabled to turn them into bytes.
    #[cfg(not(feature = "render-svg"))]
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        Err(sv_missing_serializer())
    }

    /// Re-renders the current window after `appended_points` new samples.
    ///
    /// An area over a live source is drawn from the bounded window the source
    /// already carries, so the stack baseline is recomputed for that window and
    /// the areas take their new stacked positions.
    ///
    /// # Errors
    ///
    /// Returns the composition error, or the missing-serializer error when the
    /// `render-svg` feature is absent.
    pub fn update(&self, appended_points: usize) -> Result<SVRenderOutput, SVError> {
        let prepared = sv_prepare_area(&self.configuration)?;
        sv_run_operation(
            "update",
            self.backend_kind,
            prepared.representation,
            prepared.point_count,
            || {
                let _ = appended_points;
                let rendering = sv_compose_area(&self.configuration, prepared)?;
                sv_output(self.backend_kind, &rendering)
            },
        )
    }

    /// Writes the figure to `destination`.
    ///
    /// # Errors
    ///
    /// Returns the composition error and [`SVError::BackendFailure`] when the
    /// file cannot be written.
    #[cfg(feature = "render-svg")]
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        let prepared = sv_prepare_area(&self.configuration)?;
        sv_run_operation(
            "save",
            self.backend_kind,
            prepared.representation,
            prepared.point_count,
            || {
                let rendering = sv_compose_area(&self.configuration, prepared)?;
                std::fs::write(destination, sv_svg_document(&rendering)).map_err(|error| {
                    SVError::BackendFailure {
                        backend_kind: self.backend_kind,
                        operation: "save",
                        message: format!("cannot write {}: {error}", destination.display()),
                    }
                })
            },
        )
    }

    /// Writes the figure to `destination`.
    ///
    /// # Errors
    ///
    /// Always returns [`SVError::InvalidConfiguration`] when the crate is built
    /// without the `render-svg` feature.
    #[cfg(not(feature = "render-svg"))]
    pub fn save(&self, _destination: &Path) -> Result<(), SVError> {
        Err(sv_missing_serializer())
    }

    /// Reports that an SVG document has no presentation surface.
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

#[cfg(feature = "render-svg")]
fn sv_output(
    backend_kind: SVBackendKind,
    rendering: &SVAreaRendering,
) -> Result<SVRenderOutput, SVError> {
    Ok(SVRenderOutput {
        backend_kind,
        bytes: sv_svg_document(rendering),
    })
}

#[cfg(not(feature = "render-svg"))]
fn sv_output(
    _backend_kind: SVBackendKind,
    _rendering: &SVAreaRendering,
) -> Result<SVRenderOutput, SVError> {
    Err(sv_missing_serializer())
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

#[cfg(not(feature = "render-svg"))]
fn sv_missing_serializer() -> SVError {
    SVError::InvalidConfiguration {
        message:
            "the area recipe composes primitives; enable the `render-svg` feature to serialize them"
                .to_string(),
    }
}
