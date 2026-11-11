//! The closed scale model: four kinds under one enumeration.

use super::{
    SVBooleanScale, SVContinuousScale, SVDomain, SVDomainSource, SVNominalScale, SVScaleKind,
    SVScaleRange, SVScaleTransform, SVScaleValue, SVTemporalScale, SVTemporalUnit,
};
use crate::sv_error::SVError;

/// A scale, dispatched exhaustively over the four closed kinds.
///
/// The enumeration is the kernel the marks and the axis consume: a decoration
/// asks a scale for a position and a tick list, and never inspects the kind
/// unless it needs kind-specific configuration.
#[derive(Clone, Debug, PartialEq)]
pub enum SVScale<'a> {
    /// A numeric scale over a continuous domain.
    Continuous(SVContinuousScale),
    /// A category scale over labelled slots.
    Nominal(SVNominalScale<'a>),
    /// A numeric scale whose values are timestamps.
    Temporal(SVTemporalScale),
    /// A scale over the two boolean slots.
    Boolean(SVBooleanScale),
}

impl<'a> SVScale<'a> {
    /// Returns the kind of this scale.
    pub fn kind(&self) -> SVScaleKind {
        match self {
            SVScale::Continuous(scale) => scale.kind(),
            SVScale::Nominal(scale) => scale.kind(),
            SVScale::Temporal(scale) => scale.kind(),
            SVScale::Boolean(scale) => scale.kind(),
        }
    }

    /// Returns the data interval the scale covers.
    pub fn domain(&self) -> SVDomain {
        match self {
            SVScale::Continuous(scale) => scale.domain(),
            SVScale::Nominal(scale) => scale.domain(),
            SVScale::Temporal(scale) => scale.domain(),
            SVScale::Boolean(scale) => scale.domain(),
        }
    }

    /// Returns where the domain came from.
    pub fn domain_source(&self) -> SVDomainSource {
        match self {
            SVScale::Continuous(scale) => scale.domain_source(),
            SVScale::Nominal(scale) => scale.domain_source(),
            SVScale::Temporal(scale) => scale.domain_source(),
            SVScale::Boolean(_) => SVDomainSource::Intrinsic,
        }
    }

    /// Returns the display interval.
    pub fn range(&self) -> SVScaleRange {
        match self {
            SVScale::Continuous(scale) => scale.range(),
            SVScale::Nominal(scale) => scale.range(),
            SVScale::Temporal(scale) => scale.range(),
            SVScale::Boolean(scale) => scale.range(),
        }
    }

    /// Returns the explicit tick count, or `None` when the layout decides.
    pub fn tick_count(&self) -> Option<usize> {
        match self {
            SVScale::Continuous(scale) => scale.tick_count(),
            SVScale::Nominal(_) | SVScale::Boolean(_) => None,
            SVScale::Temporal(scale) => scale.tick_count(),
        }
    }

    /// Returns the boolean labels, or `None` for every other kind.
    pub fn boolean_labels(&self) -> Option<(String, String)> {
        match self {
            SVScale::Boolean(scale) => {
                let (false_label, true_label) = scale.labels();
                Some((false_label.to_string(), true_label.to_string()))
            }
            SVScale::Continuous(_) | SVScale::Nominal(_) | SVScale::Temporal(_) => None,
        }
    }

    /// Returns the transform of a numeric scale, or `None` for the discrete
    /// kinds.
    pub fn transform(&self) -> Option<SVScaleTransform> {
        match self {
            SVScale::Continuous(scale) => Some(scale.transform()),
            SVScale::Temporal(scale) => Some(scale.transform()),
            SVScale::Nominal(_) | SVScale::Boolean(_) => None,
        }
    }

    /// Borrows the nominal scale, when this is a nominal scale.
    pub fn as_nominal(&self) -> Option<&SVNominalScale<'a>> {
        match self {
            SVScale::Nominal(scale) => Some(scale),
            SVScale::Continuous(_) | SVScale::Temporal(_) | SVScale::Boolean(_) => None,
        }
    }

    /// Adds categories a nominal scale has not seen yet.
    ///
    /// Existing categories keep their assignment index and therefore their
    /// colour; no category is removed.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when this is not a nominal
    /// scale.
    pub fn assign_categories(&mut self, categories: &[&'a str]) -> Result<(), SVError> {
        match self {
            SVScale::Nominal(scale) => {
                scale.assign_categories(categories);
                Ok(())
            }
            scale => Err(SVError::InvalidConfiguration {
                message: format!("a {} scale has no categories to assign", scale.kind()),
            }),
        }
    }

    /// Borrows the continuous projection, when this is a continuous scale.
    pub fn as_continuous(&self) -> Option<&SVContinuousScale> {
        match self {
            SVScale::Continuous(scale) => Some(scale),
            SVScale::Nominal(_) | SVScale::Temporal(_) | SVScale::Boolean(_) => None,
        }
    }

    /// Returns the temporal unit, or `None` for every other kind.
    pub fn temporal_unit(&self) -> Option<SVTemporalUnit> {
        match self {
            SVScale::Temporal(scale) => Some(scale.unit()),
            SVScale::Continuous(_) | SVScale::Nominal(_) | SVScale::Boolean(_) => None,
        }
    }

    /// Places a value on the range.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the value kind does not
    /// belong to this scale, when a category is unknown or when a numeric
    /// value is not finite.
    pub fn position(&self, value: SVScaleValue<'_>) -> Result<f64, SVError> {
        match (self, value) {
            (SVScale::Continuous(scale), SVScaleValue::Numeric(value)) => scale.position(value),
            (SVScale::Nominal(scale), SVScaleValue::Category(category)) => scale.position(category),
            (SVScale::Temporal(scale), SVScaleValue::Timestamp(timestamp)) => {
                scale.position(timestamp)
            }
            (SVScale::Boolean(scale), SVScaleValue::Boolean(value)) => Ok(scale.position(value)),
            (scale, value) => Err(SVError::InvalidConfiguration {
                message: format!(
                    "a {} scale cannot place a {} value",
                    scale.kind(),
                    value.kind()
                ),
            }),
        }
    }
}
