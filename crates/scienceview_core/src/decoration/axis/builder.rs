//! Building an axis: the option surface, validated nowhere because no option
//! can contradict another.

use super::axis_definition::SVAxis;
use super::side::SVAxisSide;
use crate::sv_scale::SVTickFormat;

/// Builds an [`SVAxis`].
///
/// The side is fixed at construction and every other option refines it.
#[derive(Clone, Debug, PartialEq)]
pub struct SVAxisBuilder {
    pub(super) side: SVAxisSide,
    pub(super) title: Option<String>,
    pub(super) format: SVTickFormat,
    pub(super) minimum_tick_spacing: Option<f64>,
}

impl SVAxisBuilder {
    /// Sets the axis title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the label format.
    #[must_use]
    pub fn tick_format(mut self, format: SVTickFormat) -> Self {
        self.format = format;
        self
    }

    /// Overrides the minimum spacing between two tick labels.
    ///
    /// A non-finite or non-positive value is ignored at measure time and the
    /// default derived from the style is used instead.
    #[must_use]
    pub fn minimum_tick_spacing(mut self, spacing: f64) -> Self {
        self.minimum_tick_spacing = Some(spacing);
        self
    }

    /// Builds the axis.
    pub fn build(self) -> SVAxis {
        SVAxis::from_builder(self)
    }
}
