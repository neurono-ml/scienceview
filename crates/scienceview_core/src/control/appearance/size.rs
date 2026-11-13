//! The size reserved for a control.

use crate::sv_error::SVError;

/// The size reserved for a control, in backend-independent units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVControlSize {
    width: f64,
    height: f64,
}

impl SVControlSize {
    /// Builds a size from its two extents.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidControlSize`] when an extent is not finite or
    /// is not strictly positive.
    pub fn new(width: f64, height: f64) -> Result<Self, SVError> {
        if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
            return Err(SVError::InvalidControlSize);
        }
        Ok(Self { width, height })
    }

    pub(crate) const fn sv_from_validated(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// Returns the horizontal extent.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Returns the vertical extent.
    pub fn height(&self) -> f64 {
        self.height
    }
}

impl Default for SVControlSize {
    fn default() -> Self {
        Self {
            width: 24.0,
            height: 24.0,
        }
    }
}
