//! The builder: the only public way to assemble a scale.

use super::super::{SVCategoryOrder, SVScaleKind, SVScaleTransform, SVTemporalUnit};
use crate::sv_data::SVColumnView;

/// Builds an [`SVScale`].
///
/// The kind is fixed at construction, because the kind of a scale is a
/// deliberate modelling decision — the same reason the backend surface has no
/// automatic variant. Every other setting refines the chosen kind and is
/// validated at [`SVScaleBuilder::build`], so an option applied to the wrong
/// kind fails loudly instead of being ignored.
///
/// A numeric scale receives its domain either explicitly with
/// [`SVScaleBuilder::domain`] or by derivation from a column view with
/// [`SVScaleBuilder::data_view`]; the explicit domain always wins.
#[derive(Clone, Debug)]
pub struct SVScaleBuilder<'a> {
    pub(super) kind: SVScaleKind,
    pub(super) explicit_domain: Option<(f64, f64)>,
    pub(super) data_view: Option<SVColumnView<'a>>,
    pub(super) range_bounds: Option<(f64, f64)>,
    pub(super) transform: SVScaleTransform,
    pub(super) transform_configured: bool,
    pub(super) nice: bool,
    pub(super) categories: Option<&'a [&'a str]>,
    pub(super) category_order: SVCategoryOrder,
    pub(super) boolean_labels: Option<(&'a str, &'a str)>,
    pub(super) temporal_unit: SVTemporalUnit,
    pub(super) tick_count: Option<usize>,
}

impl<'a> SVScaleBuilder<'a> {
    /// Starts building a scale of one kind.
    pub fn new(kind: SVScaleKind) -> Self {
        Self {
            kind,
            explicit_domain: None,
            data_view: None,
            range_bounds: None,
            transform: SVScaleTransform::Linear,
            transform_configured: false,
            nice: true,
            categories: None,
            category_order: SVCategoryOrder::DataOrder,
            boolean_labels: None,
            temporal_unit: SVTemporalUnit::Auto,
            tick_count: None,
        }
    }

    /// Configures an explicit domain, which overrides any derivation.
    ///
    /// Only continuous and temporal scales accept an explicit domain; the
    /// domain of a nominal or boolean scale is its own slot model.
    #[must_use]
    pub fn domain(mut self, lower: f64, upper: f64) -> Self {
        self.explicit_domain = Some((lower, upper));
        self
    }

    /// Attaches the column view a derived domain is read from.
    ///
    /// The view is consumed once, at build time, and the representation is
    /// dispatched on exactly once; the scale never stores the column.
    #[must_use]
    pub fn data_view(mut self, data_view: SVColumnView<'a>) -> Self {
        self.data_view = Some(data_view);
        self
    }

    /// Configures the display interval.
    #[must_use]
    pub fn range(mut self, start: f64, end: f64) -> Self {
        self.range_bounds = Some((start, end));
        self
    }

    /// Configures the transform of a continuous or temporal scale.
    #[must_use]
    pub fn transform(mut self, transform: SVScaleTransform) -> Self {
        self.transform = transform;
        self.transform_configured = true;
        self
    }

    /// Enables or disables the nice rounding of a derived domain.
    ///
    /// Rounding is on by default; an explicit domain is never rounded.
    #[must_use]
    pub fn nice(mut self, nice: bool) -> Self {
        self.nice = nice;
        self
    }

    /// Configures the categories of a nominal scale.
    #[must_use]
    pub fn categories(mut self, categories: &'a [&'a str]) -> Self {
        self.categories = Some(categories);
        self
    }

    /// Configures the display order of a nominal scale.
    #[must_use]
    pub fn category_order(mut self, order: SVCategoryOrder) -> Self {
        self.category_order = order;
        self
    }

    /// Configures the labels of a boolean scale.
    #[must_use]
    pub fn boolean_labels(mut self, false_label: &'a str, true_label: &'a str) -> Self {
        self.boolean_labels = Some((false_label, true_label));
        self
    }

    /// Configures the tick unit of a temporal scale.
    #[must_use]
    pub fn temporal_unit(mut self, unit: SVTemporalUnit) -> Self {
        self.temporal_unit = unit;
        self
    }

    /// Overrides the tick count the layout engine would otherwise derive.
    #[must_use]
    pub fn tick_count(mut self, tick_count: usize) -> Self {
        self.tick_count = Some(tick_count.max(1));
        self
    }
}
