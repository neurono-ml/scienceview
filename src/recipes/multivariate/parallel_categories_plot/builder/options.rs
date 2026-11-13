//! The option setters of [`SVParallelCategoriesPlotBuilder`].
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVParallelCategoriesPlotBuilder;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;

use super::super::{
    SVCategoryOrder, SVParallelCategoriesColourMode, SVParallelCategoriesMissingValuePolicy,
};

impl<'data> SVParallelCategoriesPlotBuilder<'data> {
    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the categorical columns, one per axis, at least two.
    #[must_use]
    pub fn variables(mut self, variables: &[&str]) -> Self {
        self.variables = variables.iter().map(|name| (*name).to_string()).collect();
        self
    }

    /// Supplies the display title of every axis, in axis order.
    ///
    /// An axis without a supplied label keeps its column name.
    #[must_use]
    pub fn variable_labels(mut self, labels: &[&str]) -> Self {
        self.variable_labels = labels.iter().map(|label| (*label).to_string()).collect();
        self
    }

    /// Supplies the display labels of one axis' categories, in its category
    /// order. Calling it again for the same axis replaces the previous list.
    #[must_use]
    pub fn category_labels(mut self, variable: impl Into<String>, labels: &[&str]) -> Self {
        let variable = variable.into();
        let labels: Vec<String> = labels.iter().map(|label| (*label).to_string()).collect();
        self.category_labels
            .retain(|(candidate, _)| candidate != &variable);
        self.category_labels.push((variable, labels));
        self
    }

    /// Names the optional grouping column that colours the ribbons.
    #[must_use]
    pub fn colour(mut self, colour: impl Into<String>) -> Self {
        self.colour = Some(colour.into());
        self
    }

    /// Selects how a ribbon derives its colour.
    #[must_use]
    pub fn colour_mode(mut self, colour_mode: SVParallelCategoriesColourMode) -> Self {
        self.colour_mode = colour_mode;
        self
    }

    /// Sets the ribbon opacity, from `0` to `1`.
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Sets the fraction of a category slot left empty between two bands.
    #[must_use]
    pub fn band_gap(mut self, band_gap: f32) -> Self {
        self.band_gap = band_gap;
        self
    }

    /// Selects the order of the categories on every axis.
    #[must_use]
    pub fn category_order(mut self, category_order: SVCategoryOrder) -> Self {
        self.category_order = category_order;
        self
    }

    /// Rotates the category labels clockwise by the given degrees.
    #[must_use]
    pub fn label_angle(mut self, label_angle: f32) -> Self {
        self.label_angle = label_angle;
        self
    }

    /// Selects what happens to a non-finite category observation.
    #[must_use]
    pub fn missing_value_policy(
        mut self,
        missing_value_policy: SVParallelCategoriesMissingValuePolicy,
    ) -> Self {
        self.missing_value_policy = missing_value_policy;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
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
