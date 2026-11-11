//! A continuous scale: a numeric domain projected onto a display range.

use super::{SVDomain, SVDomainSource, SVScaleKind, SVScaleRange, SVScaleTransform};
use crate::sv_error::SVError;

/// A numeric scale.
///
/// The scale owns both intervals (design D1): the domain is the data interval
/// and the range is the display interval, so a moving domain reprojects every
/// value through the same object and an axis cannot disagree with the marks on
/// it. The transform compresses the domain before the projection; the default
/// is linear.
///
/// The direct constructor is private: [`SVScaleBuilder`](super::SVScaleBuilder)
/// is the only public way to obtain a scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVContinuousScale {
    domain: SVDomain,
    domain_source: SVDomainSource,
    range: SVScaleRange,
    transform: SVScaleTransform,
    tick_count: Option<usize>,
}

impl SVContinuousScale {
    /// Assembles a validated continuous scale.
    pub(crate) fn new(
        domain: SVDomain,
        domain_source: SVDomainSource,
        range: SVScaleRange,
        transform: SVScaleTransform,
        tick_count: Option<usize>,
    ) -> Self {
        Self {
            domain,
            domain_source,
            range,
            transform,
            tick_count,
        }
    }

    /// Returns the kind of this scale.
    pub fn kind(&self) -> SVScaleKind {
        SVScaleKind::Continuous
    }

    /// Returns the data interval.
    pub fn domain(&self) -> SVDomain {
        self.domain
    }

    /// Returns where the domain came from.
    pub fn domain_source(&self) -> SVDomainSource {
        self.domain_source
    }

    /// Returns the display interval.
    pub fn range(&self) -> SVScaleRange {
        self.range
    }

    /// Returns the transform applied to the domain.
    pub fn transform(&self) -> SVScaleTransform {
        self.transform
    }

    /// Returns the explicit tick count, or `None` when the layout decides.
    pub fn tick_count(&self) -> Option<usize> {
        self.tick_count
    }

    /// Returns the fraction of the transformed domain a value occupies.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the value has no position
    /// in the transform's space or is not finite.
    pub fn fraction(&self, value: f64) -> Result<f64, SVError> {
        let transformed_value = self.transformed(value)?;
        let transformed_lower = self.transformed(self.domain.lower())?;
        let transformed_upper = self.transformed(self.domain.upper())?;
        let span = transformed_upper - transformed_lower;
        if span == 0.0 {
            return Ok(0.5);
        }

        Ok((transformed_value - transformed_lower) / span)
    }

    /// Places a value on the range.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the value has no
    /// fraction (it is not finite or outside the transform's domain).
    pub fn position(&self, value: f64) -> Result<f64, SVError> {
        Ok(self.range.interpolate(self.fraction(value)?))
    }

    /// Returns the value at a position on the range.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the position is not
    /// finite or the transform cannot restore the value.
    pub fn inverse_position(&self, position: f64) -> Result<f64, SVError> {
        self.inverse_of_fraction(self.fraction_of_position(position)?)
    }

    /// Returns the fraction of the range a position occupies.
    pub(crate) fn fraction_of_position(&self, position: f64) -> Result<f64, SVError> {
        if !position.is_finite() {
            return Err(SVError::InvalidConfiguration {
                message: format!("the position {position} is not a finite display coordinate"),
            });
        }

        if self.range.length() == 0.0 {
            return Ok(0.5);
        }

        Ok((position - self.range.start()) / self.range.length())
    }

    /// Returns the value at a fraction of the transformed domain.
    pub(crate) fn inverse_of_fraction(&self, fraction: f64) -> Result<f64, SVError> {
        if !fraction.is_finite() {
            return Err(SVError::InvalidConfiguration {
                message: format!("the fraction {fraction} is not finite"),
            });
        }

        let transformed_lower = self.transformed(self.domain.lower())?;
        let transformed_upper = self.transformed(self.domain.upper())?;
        let transformed = transformed_lower + (transformed_upper - transformed_lower) * fraction;

        self.transform
            .inverse(transformed)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!(
                    "the {} transform cannot restore the value at fraction {fraction}",
                    self.transform
                ),
            })
    }

    /// Maps a data value into the transform's space.
    fn transformed(&self, value: f64) -> Result<f64, SVError> {
        self.transform
            .forward(value)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!(
                    "the value {value} has no position under the {} transform",
                    self.transform
                ),
            })
    }
}
