//! The built regression chart: its configuration and its operations.
//!
//! The fit runs inside `compose`, before any backend is consulted. The plot is
//! not generic over the backend: it stores the configured kind, and every
//! operation records it in its span (patterns.md P2 and P8).

use super::composition::{SVRegressionComposition, sv_compose};
use super::configuration::SVRegressionConfiguration;
use super::pairing::{SVResolvedInputs, sv_resolve_inputs};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVDataSource, SVRepresentation};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_observability::sv_run_operation;

/// A regression chart ready to compose and render.
///
/// Built only by [`SVRegressionPlotBuilder`](super::SVRegressionPlotBuilder):
/// the direct constructor is private, so the required backend selection cannot
/// be bypassed.
pub struct SVRegressionPlot<'source> {
    source: &'source dyn SVDataSource,
    backend_kind: SVBackendKind,
    execution_mode: SVExecutionMode,
    title: Option<String>,
    configuration: SVRegressionConfiguration,
}

impl<'source> SVRegressionPlot<'source> {
    /// Transfers a validated configuration into a plot.
    pub(crate) fn from_configuration(
        source: &'source dyn SVDataSource,
        backend_kind: SVBackendKind,
        execution_mode: SVExecutionMode,
        title: Option<String>,
        configuration: SVRegressionConfiguration,
    ) -> Self {
        Self {
            source,
            backend_kind,
            execution_mode,
            title,
            configuration,
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

    /// Returns the chart title, when one was set.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the validated configuration.
    pub fn configuration(&self) -> &SVRegressionConfiguration {
        &self.configuration
    }

    /// Resolves the data and fits one model per series.
    ///
    /// This is the operation the documentation example and the tests inspect;
    /// it performs the whole data path without serializing a backend format.
    ///
    /// # Errors
    ///
    /// Propagates the data-boundary and fit-kernel errors described by
    /// [`sv_resolve_inputs`] and
    /// [`SVStat::Fit`](scienceview_core::sv_stat::SVStat::Fit).
    pub fn compose(&self) -> Result<SVRegressionComposition, SVError> {
        let inputs = sv_resolve_inputs(self.source, &self.configuration)?;
        sv_run_composition("compose", self.backend_kind, inputs, &self.configuration)
    }

    /// Renders the chart to a self-contained SVG document.
    ///
    /// Only the SVG backend is implemented by this change. The plot still
    /// builds for every backend that can draw its marks, so a later backend
    /// only adds its own rendering path.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnsupportedBackendCapability`] when the configured
    /// kind is not `Svg`, and propagates composition errors.
    #[cfg(feature = "render-svg")]
    pub fn render(&self) -> Result<scienceview_core::sv_backend::SVRenderOutput, SVError> {
        if self.backend_kind != SVBackendKind::Svg {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind: self.backend_kind,
                capability: scienceview_core::sv_backend::SVBackendCapability::Export,
            });
        }
        let inputs = sv_resolve_inputs(self.source, &self.configuration)?;
        let representation = inputs.representation;
        let points = inputs.point_count();
        sv_run_operation("render", self.backend_kind, representation, points, || {
            let composition = sv_compose(inputs, &self.configuration)?;
            let bytes = super::render_svg::sv_render_svg(
                &composition,
                &self.configuration,
                self.title.as_deref(),
            )?;
            Ok(scienceview_core::sv_backend::SVRenderOutput {
                backend_kind: self.backend_kind,
                bytes: bytes.into_bytes(),
            })
        })
    }

    /// Writes the rendered SVG document to `destination`.
    ///
    /// # Errors
    ///
    /// Propagates [`SVRegressionPlot::render`] and the filesystem error.
    #[cfg(feature = "render-svg")]
    pub fn save(&self, destination: &std::path::Path) -> Result<(), SVError> {
        let output = self.render()?;
        std::fs::write(destination, output.bytes).map_err(|error| SVError::BackendFailure {
            backend_kind: self.backend_kind,
            operation: "save",
            message: error.to_string(),
        })
    }
}

/// Runs the composition inside one operation span.
fn sv_run_composition(
    operation: &'static str,
    backend_kind: SVBackendKind,
    inputs: SVResolvedInputs,
    configuration: &SVRegressionConfiguration,
) -> Result<SVRegressionComposition, SVError> {
    let representation: SVRepresentation = inputs.representation;
    let points = inputs.point_count();
    sv_run_operation(operation, backend_kind, representation, points, || {
        sv_compose(inputs, configuration)
    })
}
