//! A validated interval in data coordinates.

use core::fmt;

use crate::sv_error::SVError;
use crate::sv_scale::SVDomain;

/// One interval a control reads or writes, in data coordinates.
///
/// Both bounds are finite and `lower <= upper`. The type is the value a range
/// slider carries, the value a zoom produces and the value an event reports, so
/// every step of the way speaks the same interval.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVControlRange {
    lower: f64,
    upper: f64,
}

impl SVControlRange {
    /// Builds a range from its two bounds.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when a bound is not finite or
    /// when the bounds are inverted.
    pub fn new(lower: f64, upper: f64) -> Result<Self, SVError> {
        if !lower.is_finite() || !upper.is_finite() {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "a control range needs finite bounds, received ({lower}, {upper})"
                ),
            });
        }
        if lower > upper {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "a control range needs lower <= upper, received ({lower}, {upper})"
                ),
            });
        }
        Ok(Self { lower, upper })
    }

    /// Returns the unit interval `0..=1`.
    pub const fn unit() -> Self {
        Self {
            lower: 0.0,
            upper: 1.0,
        }
    }

    /// Returns the lower bound.
    pub fn lower(&self) -> f64 {
        self.lower
    }

    /// Returns the upper bound.
    pub fn upper(&self) -> f64 {
        self.upper
    }

    /// Returns the extent of the range.
    pub fn length(&self) -> f64 {
        self.upper - self.lower
    }

    /// Returns `true` when the value falls inside the range.
    pub fn contains(&self, value: f64) -> bool {
        value.is_finite() && value >= self.lower && value <= self.upper
    }

    /// Returns the range clamped into `bounds`, preserving the order.
    ///
    /// The clamp is monotonic, so clamping both bounds independently is enough:
    /// a range entirely to one side of the bounds collapses onto the nearest
    /// bound instead of inverting.
    pub fn clamp_into(self, bounds: Self) -> Self {
        Self {
            lower: self.lower.max(bounds.lower).min(bounds.upper),
            upper: self.upper.min(bounds.upper).max(bounds.lower),
        }
    }

    /// Returns the range as a scale domain.
    ///
    /// The range's own construction already proved both bounds finite and
    /// ordered, so the domain shares that proof.
    pub fn to_domain(self) -> SVDomain {
        SVDomain::from_validated(self.lower, self.upper)
    }

    /// Converts a scale domain into a control range.
    pub fn from_domain(domain: SVDomain) -> Self {
        Self {
            lower: domain.lower(),
            upper: domain.upper(),
        }
    }
}

impl fmt::Display for SVControlRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{},{}", self.lower, self.upper)
    }
}

/// One axis and the range it currently shows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVAxisRange {
    axis: super::SVControlAxis,
    range: SVControlRange,
}

impl SVAxisRange {
    /// Pairs an axis with a range.
    pub fn new(axis: super::SVControlAxis, range: SVControlRange) -> Self {
        Self { axis, range }
    }

    /// Returns the axis.
    pub fn axis(&self) -> super::SVControlAxis {
        self.axis
    }

    /// Returns the range.
    pub fn range(&self) -> SVControlRange {
        self.range
    }
}

impl fmt::Display for SVAxisRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}={}", self.axis, self.range)
    }
}
