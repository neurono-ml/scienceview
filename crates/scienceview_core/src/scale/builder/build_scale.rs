//! The validation pass: every rule a scale configuration must satisfy.

use super::super::{
    SVBooleanScale, SVContinuousScale, SVDomain, SVDomainSource, SVNominalScale, SVScale,
    SVScaleKind, SVScaleRange, SVScaleTransform, SVTemporalScale, sv_nice_domain,
};
use super::SVScaleBuilder;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;

impl<'a> SVScaleBuilder<'a> {
    /// Validates the configuration and builds the scale.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the configuration is
    /// incomplete, when an option does not apply to the chosen kind or when the
    /// domain falls outside the transform's mathematical domain.
    pub fn build(self) -> Result<SVScale<'a>, SVError> {
        let range = match self.range_bounds {
            Some((start, end)) => SVScaleRange::new(start, end)?,
            None => SVScaleRange::default(),
        };

        match self.kind {
            SVScaleKind::Continuous => {
                sv_reject_option(self.categories.is_some(), "categories", self.kind)?;
                sv_reject_option(self.boolean_labels.is_some(), "boolean labels", self.kind)?;
                let continuous = sv_build_continuous_scale(
                    self.explicit_domain,
                    self.data_view,
                    self.transform,
                    self.nice,
                    range,
                    self.tick_count,
                )?;
                Ok(SVScale::Continuous(continuous))
            }
            SVScaleKind::Nominal => {
                sv_reject_option(
                    self.explicit_domain.is_some(),
                    "a numeric domain",
                    self.kind,
                )?;
                sv_reject_option(self.data_view.is_some(), "a numeric data view", self.kind)?;
                sv_reject_option(self.boolean_labels.is_some(), "boolean labels", self.kind)?;
                sv_reject_option(self.transform_configured, "a transform", self.kind)?;
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
                if self.transform_configured && self.transform != SVScaleTransform::Linear {
                    return Err(SVError::InvalidConfiguration {
                        message: format!(
                            "a temporal scale interpolates linearly and cannot apply the {} transform",
                            self.transform
                        ),
                    });
                }
                let continuous = sv_build_continuous_scale(
                    self.explicit_domain,
                    self.data_view,
                    SVScaleTransform::Linear,
                    self.nice,
                    range,
                    self.tick_count,
                )?;
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
                sv_reject_option(self.data_view.is_some(), "a numeric data view", self.kind)?;
                sv_reject_option(self.categories.is_some(), "categories", self.kind)?;
                sv_reject_option(self.transform_configured, "a transform", self.kind)?;
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

/// Builds a numeric scale from an explicit or a derived domain.
fn sv_build_continuous_scale(
    explicit_domain: Option<(f64, f64)>,
    data_view: Option<SVColumnView<'_>>,
    transform: SVScaleTransform,
    nice: bool,
    range: SVScaleRange,
    tick_count: Option<usize>,
) -> Result<SVContinuousScale, SVError> {
    let (domain, domain_source) = match explicit_domain {
        Some((lower, upper)) => (SVDomain::new(lower, upper)?, SVDomainSource::Explicit),
        None => {
            let view = data_view.ok_or_else(sv_missing_domain)?;
            let derived = SVDomain::from_column_view(view)?;
            let domain = if nice {
                sv_nice_domain(derived, transform)
            } else {
                derived
            };
            (domain, SVDomainSource::Derived)
        }
    };

    if !transform.domain_is_defined(domain) {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "the {transform} transform is undefined on the domain [{}, {}]",
                domain.lower(),
                domain.upper()
            ),
        });
    }

    Ok(SVContinuousScale::new(
        domain,
        domain_source,
        range,
        transform,
        tick_count,
    ))
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
fn sv_missing_domain() -> SVError {
    SVError::InvalidConfiguration {
        message: "a numeric scale requires an explicit domain or a data view".to_string(),
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
