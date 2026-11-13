//! The option setters of [`SVCountPlotBuilder`].
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVCountPlotBuilder;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVBarOrientation;
use crate::sv_scale::SVColour;
use crate::sv_stat::{SVCountOrder, SVNormalisation};

use super::super::{SVCountMissingValuePolicy, SVCountMultiple};

impl<'data> SVCountPlotBuilder<'data> {
    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the required category column to count.
    #[must_use]
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Names the optional grouping column; every series is counted separately.
    #[must_use]
    pub fn series(mut self, series: impl Into<String>) -> Self {
        self.series = Some(series.into());
        self
    }

    /// Supplies human labels for the categories, in axis order.
    ///
    /// The data boundary carries numeric category codes; a label list is how a
    /// caller names them. A category without a supplied label keeps its code.
    #[must_use]
    pub fn category_labels(mut self, labels: &[&str]) -> Self {
        self.category_labels = labels.iter().map(|label| (*label).to_string()).collect();
        self
    }

    /// Supplies human labels for the series values, in first-appearance order.
    #[must_use]
    pub fn series_labels(mut self, labels: &[&str]) -> Self {
        self.series_labels = labels.iter().map(|label| (*label).to_string()).collect();
        self
    }

    /// Selects whether the bars grow upwards or rightwards.
    #[must_use]
    pub fn orient(mut self, orientation: SVBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the fraction of a category slot the bar occupies, from `0` to `1`.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Selects the normalisation of the counted values.
    #[must_use]
    pub fn normalise(mut self, normalisation: SVNormalisation) -> Self {
        self.normalisation = normalisation;
        self
    }

    /// Selects the order of the categories on the axis.
    #[must_use]
    pub fn order(mut self, order: SVCountOrder) -> Self {
        self.order = order;
        self
    }

    /// Overrides the theme palette with one colour for every bar.
    #[must_use]
    pub fn colour(mut self, colour: SVColour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Selects how several series share one category: layered, stacked or
    /// dodged.
    #[must_use]
    pub fn multiple(mut self, multiple: SVCountMultiple) -> Self {
        self.multiple = multiple;
        self
    }

    /// Selects what happens to a missing or non-finite category observation.
    #[must_use]
    pub fn missing_value_policy(mut self, missing_value_policy: SVCountMissingValuePolicy) -> Self {
        self.missing_value_policy = missing_value_policy;
        self
    }

    /// Names the optional observation weight column.
    ///
    /// The shipped `SVStat::Count` kernel does not consume weights, so a
    /// configured weight is rejected at build time with a typed configuration
    /// error naming the kernel gap; the recipe never reimplements weighted
    /// counting.
    #[must_use]
    pub fn weight(mut self, weight: impl Into<String>) -> Self {
        self.weight = Some(weight.into());
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the horizontal axis title.
    #[must_use]
    pub fn x_label(mut self, x_label: impl Into<String>) -> Self {
        self.x_label = Some(x_label.into());
        self
    }

    /// Sets the vertical axis title.
    #[must_use]
    pub fn y_label(mut self, y_label: impl Into<String>) -> Self {
        self.y_label = Some(y_label.into());
        self
    }

    /// Selects the rendering backend. Required.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Selects the execution mode. Defaults to `Automatic`.
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}
