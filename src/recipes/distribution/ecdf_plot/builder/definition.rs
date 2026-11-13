//! The builder value and its documented defaults.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::SVBarOrientation;

/// Builds an [`SVECdfPlot`](super::super::SVECdfPlot).
///
/// The builder is the only public constructor. `backend(...)` is required:
/// there is no automatic backend, so the rendering form is always a deliberate
/// choice. `execution_mode(...)` defaults to [`SVExecutionMode::Automatic`].
#[derive(Clone)]
pub struct SVECdfPlotBuilder<'source> {
    pub(super) data_source: Option<&'source dyn SVDataSource>,
    pub(super) value: Option<String>,
    pub(super) series: Option<String>,
    pub(super) normalise: bool,
    pub(super) orient: SVBarOrientation,
    pub(super) complementary: bool,
    pub(super) line_width: Option<SVLength>,
    pub(super) viewport: SVViewport,
    pub(super) title: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl SVECdfPlotBuilder<'_> {
    /// Starts an ECDF plot with its documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            value: None,
            series: None,
            normalise: true,
            orient: SVBarOrientation::Vertical,
            complementary: false,
            line_width: None,
            viewport: SVViewport::new(640.0, 400.0),
            title: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::default(),
        }
    }
}

impl Default for SVECdfPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
