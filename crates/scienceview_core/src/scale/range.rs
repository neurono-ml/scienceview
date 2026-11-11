//! The display interval a scale maps onto.

use crate::sv_error::SVError;

/// The interval of display space a scale covers, in backend-independent units.
///
/// The range is stored as `start..=end`, so a reversed pair (`end < start`)
/// expresses an inverted axis exactly as a renderer would draw it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVScaleRange {
    start: f64,
    end: f64,
}

impl SVScaleRange {
    /// Builds a range from its two bounds.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when a bound is not finite;
    /// a reversed range is valid because it describes an inverted axis.
    pub fn new(start: f64, end: f64) -> Result<Self, SVError> {
        if !start.is_finite() || !end.is_finite() {
            return Err(SVError::InvalidConfiguration {
                message: format!("a scale range needs finite bounds, received ({start}, {end})"),
            });
        }

        Ok(Self { start, end })
    }

    /// Returns the bound a fraction of zero maps to.
    pub fn start(&self) -> f64 {
        self.start
    }

    /// Returns the bound a fraction of one maps to.
    pub fn end(&self) -> f64 {
        self.end
    }

    /// Returns the signed extent of the range, negative when inverted.
    pub fn length(&self) -> f64 {
        self.end - self.start
    }

    /// Places a fraction of the range.
    ///
    /// The fraction is not clamped: a value outside the domain extrapolates
    /// along the same line, which is what an axis needs when the layout uses a
    /// margin. A non-finite fraction resolves to the start bound so a missing
    /// value never poisons a render.
    pub fn interpolate(&self, fraction: f64) -> f64 {
        if !fraction.is_finite() {
            return self.start;
        }
        self.start + (self.end - self.start) * fraction
    }
}

impl Default for SVScaleRange {
    fn default() -> Self {
        Self {
            start: 0.0,
            end: 1.0,
        }
    }
}
