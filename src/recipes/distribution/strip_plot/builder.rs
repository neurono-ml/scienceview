//! The `SVStripPlot` builder: defaults and the option surface.

use super::SVStripMissingValuePolicy;
use super::core_implementation::SVStripPlot;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::{SVBarOrientation, SVDotShape};
use crate::sv_move::SVJitterMode;

/// Builds a [`SVStripPlot`](super::SVStripPlot).
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVStripPlotBuilder::build), so a strip plot that exists can
/// render.
#[derive(Clone)]
pub struct SVStripPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) value: Option<String>,
    pub(super) category: Option<String>,
    pub(super) series: Option<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) jitter: f32,
    pub(super) jitter_mode: SVJitterMode,
    pub(super) seed: Option<u64>,
    pub(super) marker: SVDotShape,
    pub(super) alpha: f32,
    pub(super) size: Option<SVLength>,
    pub(super) orientation: SVBarOrientation,
    pub(super) missing_value_policy: SVStripMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) value_label: Option<String>,
    pub(super) category_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl<'data> SVStripPlotBuilder<'data> {
    /// Starts a strip plot with its documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            value: None,
            category: None,
            series: None,
            series_labels: Vec::new(),
            jitter: 0.1,
            jitter_mode: SVJitterMode::Uniform,
            seed: None,
            marker: SVDotShape::Circle,
            alpha: 1.0,
            size: None,
            orientation: SVBarOrientation::Vertical,
            missing_value_policy: SVStripMissingValuePolicy::Omit,
            title: None,
            value_label: None,
            category_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }

    /// Validates the configuration and builds the strip plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`](crate::sv_error::SVError::MissingDataSource)
    /// or [`SVError::MissingBackendSelection`](crate::sv_error::SVError::MissingBackendSelection)
    /// for a required step, [`SVError::InvalidConfiguration`](crate::sv_error::SVError::InvalidConfiguration)
    /// for a missing column or an impossible option,
    /// [`SVError::ColumnNotFound`](crate::sv_error::SVError::ColumnNotFound) for
    /// a column the source does not expose, [`SVError::MismatchedLengths`](crate::sv_error::SVError::MismatchedLengths)
    /// when the columns disagree and
    /// [`SVError::UnsupportedMark`](crate::sv_error::SVError::UnsupportedMark)
    /// when the backend cannot draw dots.
    pub fn build(self) -> Result<SVStripPlot<'data>, SVError> {
        super::build::sv_build_strip_plot(self)
    }
}

impl Default for SVStripPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
