//! The builder value and its starting state.

use super::super::options::{
    SVDimension, SVParallelCoordinatesMissingPolicy, SVParallelCoordinatesNormalization,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::SVMarkTheme;

/// Builds an [`SVParallelCoordinatesPlot`](super::SVParallelCoordinatesPlot).
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`].
#[derive(Clone)]
pub struct SVParallelCoordinatesPlotBuilder<'values> {
    pub(super) data_source: Option<&'values dyn SVDataSource>,
    pub(super) variables: Vec<&'values str>,
    pub(super) colour: Option<&'values str>,
    pub(super) line_width: SVLength,
    pub(super) opacity: f32,
    pub(super) dimensions: Vec<SVDimension<'values>>,
    pub(super) range: Option<Vec<(f64, f64)>>,
    pub(super) label_angle: f32,
    pub(super) axis_label: Option<String>,
    pub(super) title: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) normalization: SVParallelCoordinatesNormalization,
    pub(super) missing: SVParallelCoordinatesMissingPolicy,
}

impl SVParallelCoordinatesPlotBuilder<'_> {
    /// Starts a parallel-coordinates plot with the documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            variables: Vec::new(),
            colour: None,
            line_width: SVLength::absolute(SVMarkTheme::default().line_width()),
            opacity: 0.5,
            dimensions: Vec::new(),
            range: None,
            label_angle: 0.0,
            axis_label: None,
            title: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
            normalization: SVParallelCoordinatesNormalization::PerAxis,
            missing: SVParallelCoordinatesMissingPolicy::Gap,
        }
    }
}

impl Default for SVParallelCoordinatesPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
