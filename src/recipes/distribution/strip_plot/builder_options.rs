//! The option setters of [`SVStripPlotBuilder`](super::SVStripPlotBuilder).
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVStripMissingValuePolicy;
use super::builder::SVStripPlotBuilder;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::{SVBarOrientation, SVDotShape};
use crate::sv_move::SVJitterMode;

impl<'data> SVStripPlotBuilder<'data> {
    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the required column of observed values.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Names the required categorical column the observations sit on.
    #[must_use]
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Names the optional column that colours the dots within a category.
    #[must_use]
    pub fn series(mut self, series: impl Into<String>) -> Self {
        self.series = Some(series.into());
        self
    }

    /// Supplies human labels for the series values, in first-appearance order.
    #[must_use]
    pub fn series_labels(mut self, labels: &[&str]) -> Self {
        self.series_labels = labels.iter().map(|label| (*label).to_string()).collect();
        self
    }

    /// Sets the jitter amount as a fraction of the category slot (default `0.1`).
    #[must_use]
    pub fn jitter(mut self, jitter: f32) -> Self {
        self.jitter = jitter;
        self
    }

    /// Selects the distribution the jitter draws from (default `Uniform`).
    #[must_use]
    pub fn jitter_mode(mut self, jitter_mode: SVJitterMode) -> Self {
        self.jitter_mode = jitter_mode;
        self
    }

    /// Fixes the jitter seed, making the displacement reproducible.
    #[must_use]
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Selects the marker shape (default `Circle`).
    #[must_use]
    pub fn marker(mut self, marker: SVDotShape) -> Self {
        self.marker = marker;
        self
    }

    /// Sets the constant opacity of every dot, from `0.0` to `1.0`.
    #[must_use]
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Sets the dot size; `None` keeps the `mark.size` token.
    #[must_use]
    pub fn size(mut self, size: SVLength) -> Self {
        self.size = Some(size);
        self
    }

    /// Chooses whether categories run along the horizontal or vertical axis.
    #[must_use]
    pub fn orientation(mut self, orientation: SVBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Chooses what happens to a non-finite observation (default `Omit`).
    #[must_use]
    pub fn missing_value_policy(mut self, policy: SVStripMissingValuePolicy) -> Self {
        self.missing_value_policy = policy;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the title of the value axis.
    #[must_use]
    pub fn value_label(mut self, value_label: impl Into<String>) -> Self {
        self.value_label = Some(value_label.into());
        self
    }

    /// Sets the title of the categorical axis.
    #[must_use]
    pub fn category_label(mut self, category_label: impl Into<String>) -> Self {
        self.category_label = Some(category_label.into());
        self
    }

    /// Selects the rendering backend. Required: there is no automatic backend.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Selects the execution mode. Defaults to [`SVExecutionMode::Automatic`].
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}
