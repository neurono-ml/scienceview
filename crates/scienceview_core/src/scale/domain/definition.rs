//! The data interval a scale covers and where it came from.

use crate::sv_error::SVError;

/// How the domain of a scale was obtained.
///
/// The provenance is part of the public model because an animated axis must
/// know whether a domain is fixed by the caller or still follows the data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVDomainSource {
    /// The caller configured the interval; it never follows the data.
    Explicit,
    /// The interval was derived from the data attached to the scale.
    Derived,
    /// The interval is the scale's own category model (nominal and boolean).
    Intrinsic,
}

impl SVDomainSource {
    /// Returns the source name as written in observability spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVDomainSource::Explicit => "Explicit",
            SVDomainSource::Derived => "Derived",
            SVDomainSource::Intrinsic => "Intrinsic",
        }
    }
}

/// The interval of data space a scale covers.
///
/// Both bounds are finite and `lower <= upper`; a degenerate domain
/// (`lower == upper`) is valid and describes a single-valued axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVDomain {
    lower: f64,
    upper: f64,
}

impl SVDomain {
    /// Builds a domain from its two bounds.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when a bound is not finite or
    /// when the bounds are inverted. An inverted domain is a configuration
    /// failure, not a reversed axis: reversals belong to the range, where the
    /// renderer draws them.
    pub fn new(lower: f64, upper: f64) -> Result<Self, SVError> {
        if !lower.is_finite() || !upper.is_finite() {
            return Err(SVError::InvalidConfiguration {
                message: format!("a scale domain needs finite bounds, received ({lower}, {upper})"),
            });
        }

        if lower > upper {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "a scale domain needs lower <= upper, received ({lower}, {upper}); reverse the range instead"
                ),
            });
        }

        Ok(Self { lower, upper })
    }

    /// Builds a domain whose bounds the caller already validated.
    pub(crate) fn sv_from_validated(lower: f64, upper: f64) -> Self {
        Self { lower, upper }
    }

    /// Returns the lower bound.
    pub fn lower(&self) -> f64 {
        self.lower
    }

    /// Returns the upper bound.
    pub fn upper(&self) -> f64 {
        self.upper
    }

    /// Returns the extent of the domain.
    pub fn length(&self) -> f64 {
        self.upper - self.lower
    }

    /// Returns `true` when the domain holds exactly one value.
    pub fn is_degenerate(&self) -> bool {
        self.lower == self.upper
    }

    /// Returns `true` when a finite value falls inside the domain.
    pub fn contains(&self, value: f64) -> bool {
        value.is_finite() && value >= self.lower && value <= self.upper
    }

    /// Returns the position of a value as a fraction of the domain.
    ///
    /// A degenerate domain answers `0.5` for any finite value, which places
    /// the single tick in the middle of the axis. A non-finite value has no
    /// position and answers `None`; the fraction is not clamped, so a value
    /// outside the domain extrapolates.
    pub fn normalize(&self, value: f64) -> Option<f64> {
        if !value.is_finite() {
            return None;
        }

        if self.is_degenerate() {
            return Some(0.5);
        }

        Some((value - self.lower) / self.length())
    }
}
