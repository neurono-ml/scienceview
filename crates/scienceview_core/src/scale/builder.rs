//! The builder: the only public way to assemble a scale.

use super::{
    SVBooleanScale, SVCategoryOrder, SVContinuousScale, SVDomain, SVDomainSource, SVNominalScale,
    SVScale, SVScaleKind, SVScaleRange, SVTemporalScale, SVTemporalUnit,
};
use crate::sv_error::SVError;

/// Builds an [`SVScale`].
///
/// The kind is fixed at construction, because the kind of a scale is a
/// deliberate modelling decision — the same reason the backend surface has no
/// automatic variant. Every other setting refines the chosen kind and is
/// validated at [`SVScaleBuilder::build`], so an option applied to the wrong
/// kind fails loudly instead of being ignored.
#[derive(Clone, Debug)]
pub struct SVScaleBuilder<'a> {
    kind: SVScaleKind,
    explicit_domain: Option<(f64, f64)>,
    range_bounds: Option<(f64, f64)>,
    categories: Option<&'a [&'a str]>,
    category_order: SVCategoryOrder,
    boolean_labels: Option<(&'a str, &'a str)>,
    temporal_unit: SVTemporalUnit,
    tick_count: Option<usize>,
}

impl<'a> SVScaleBuilder<'a> {
    /// Starts building a scale of one kind.
    pub fn new(kind: SVScaleKind) -> Self {
        Self {
            kind,
            explicit_domain: None,
            range_bounds: None,
            categories: None,
            category_order: SVCategoryOrder::DataOrder,
            boolean_labels: None,
            temporal_unit: SVTemporalUnit::Auto,
            tick_count: None,
        }
    }

    /// Configures an explicit domain, which overrides any later derivation.
    ///
    /// Only continuous and temporal scales accept an explicit domain; the
    /// domain of a nominal or boolean scale is its own slot model.
    #[must_use]
    pub fn domain(mut self, lower: f64, upper: f64) -> Self {
        self.explicit_domain = Some((lower, upper));
        self
    }

    /// Configures the display interval.
    #[must_use]
    pub fn range(mut self, start: f64, end: f64) -> Self {
        self.range_bounds = Some((start, end));
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

    /// Validates the configuration and builds the scale.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the configuration is
    /// incomplete or when an option does not apply to the chosen kind.
    pub fn build(self) -> Result<SVScale<'a>, SVError> {
        let range = match self.range_bounds {
            Some((start, end)) => SVScaleRange::new(start, end)?,
            None => SVScaleRange::default(),
        };

        match self.kind {
            SVScaleKind::Continuous => {
                sv_reject_option(self.categories.is_some(), "categories", self.kind)?;
                sv_reject_option(self.boolean_labels.is_some(), "boolean labels", self.kind)?;
                let domain = self.explicit_domain.ok_or_else(|| {
                    sv_missing_domain("continuous", "an explicit domain or a data view")
                })?;
                let domain = SVDomain::new(domain.0, domain.1)?;
                Ok(SVScale::Continuous(SVContinuousScale::new(
                    domain,
                    SVDomainSource::Explicit,
                    range,
                    self.tick_count,
                )))
            }
            SVScaleKind::Nominal => {
                sv_reject_option(
                    self.explicit_domain.is_some(),
                    "a numeric domain",
                    self.kind,
                )?;
                sv_reject_option(self.boolean_labels.is_some(), "boolean labels", self.kind)?;
                let categories = self
                    .categories
                    .ok_or_else(|| SVError::InvalidConfiguration {
                        message:
                            "a nominal scale requires its categories; pass them with `categories`"
                                .to_string(),
                    })?;
                let categories = sv_unique_categories(categories);
                if categories.is_empty() {
                    return Err(SVError::InvalidConfiguration {
                        message: "a nominal scale requires at least one category".to_string(),
                    });
                }
                Ok(SVScale::Nominal(SVNominalScale::new(
                    categories,
                    self.category_order,
                    range,
                )))
            }
            SVScaleKind::Temporal => {
                sv_reject_option(self.categories.is_some(), "categories", self.kind)?;
                sv_reject_option(self.boolean_labels.is_some(), "boolean labels", self.kind)?;
                let domain = self.explicit_domain.ok_or_else(|| {
                    sv_missing_domain("temporal", "an explicit domain or a data view")
                })?;
                let domain = SVDomain::new(domain.0, domain.1)?;
                let continuous = SVContinuousScale::new(
                    domain,
                    SVDomainSource::Explicit,
                    range,
                    self.tick_count,
                );
                Ok(SVScale::Temporal(SVTemporalScale::new(
                    continuous,
                    self.temporal_unit,
                )))
            }
            SVScaleKind::Boolean => {
                sv_reject_option(
                    self.explicit_domain.is_some(),
                    "a numeric domain",
                    self.kind,
                )?;
                sv_reject_option(self.categories.is_some(), "categories", self.kind)?;
                let labels = match self.boolean_labels {
                    Some((false_label, true_label)) => {
                        (false_label.to_string(), true_label.to_string())
                    }
                    None => ("false".to_string(), "true".to_string()),
                };
                Ok(SVScale::Boolean(SVBooleanScale::new(range, labels)))
            }
        }
    }
}

/// Rejects an option that does not apply to the chosen kind.
fn sv_reject_option(configured: bool, option: &str, kind: SVScaleKind) -> Result<(), SVError> {
    if configured {
        return Err(SVError::InvalidConfiguration {
            message: format!("a {kind} scale does not accept {option}"),
        });
    }
    Ok(())
}

/// Reports a missing domain with the accepted alternatives.
fn sv_missing_domain(kind: &str, alternatives: &str) -> SVError {
    SVError::InvalidConfiguration {
        message: format!("a {kind} scale requires {alternatives}"),
    }
}

/// Keeps the first occurrence of every category, preserving assignment order.
fn sv_unique_categories<'a>(categories: &[&'a str]) -> Vec<&'a str> {
    let mut unique: Vec<&'a str> = Vec::with_capacity(categories.len());
    for category in categories {
        if !unique.contains(category) {
            unique.push(*category);
        }
    }
    unique
}
