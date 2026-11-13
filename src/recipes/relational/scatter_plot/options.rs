//! The typed options a scatter exposes beyond its channel names.

/// Where the series legend is placed relative to the plot area.
///
/// `Auto` keeps the legend beside the plot area on the right, the placement
/// every source library uses when the caller expresses no preference.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVScatterLegendPlacement {
    /// Place the legend where it does not cover the marks.
    #[default]
    Auto,
    /// Beside the plot area on the right.
    Right,
    /// Beside the plot area on the left.
    Left,
    /// Above the plot area.
    Top,
    /// Below the plot area.
    Bottom,
    /// Over the plot area.
    Inside,
}

/// The display interval a continuous `size` column is mapped onto.
///
/// The values are diameters in logical pixels, so a point of the smallest
/// value in the column is drawn with `minimum` and the largest with
/// `maximum`. The default mirrors the mark token's size and keeps a large
/// scatter readable.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVScatterSizeScale {
    minimum: f64,
    maximum: f64,
}

impl Default for SVScatterSizeScale {
    fn default() -> Self {
        Self {
            minimum: 4.0,
            maximum: 16.0,
        }
    }
}

impl SVScatterSizeScale {
    /// Builds a size interval from its two endpoints.
    #[must_use]
    pub fn new(minimum: f64, maximum: f64) -> Self {
        Self { minimum, maximum }
    }

    /// Returns the diameter of the smallest value.
    pub fn minimum(&self) -> f64 {
        self.minimum
    }

    /// Returns the diameter of the largest value.
    pub fn maximum(&self) -> f64 {
        self.maximum
    }

    /// Validates the interval at build time.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`](crate::sv_error::SVError::InvalidConfiguration)
    /// when an endpoint is not finite or positive, or when the interval is
    /// inverted.
    pub(crate) fn validate(&self, option: &str) -> Result<(), crate::sv_error::SVError> {
        let valid = self.minimum.is_finite()
            && self.maximum.is_finite()
            && self.minimum > 0.0
            && self.maximum >= self.minimum;
        if valid {
            return Ok(());
        }

        Err(crate::sv_error::SVError::InvalidConfiguration {
            message: format!(
                "{option} needs finite positive diameters with minimum <= maximum, received {}..={}",
                self.minimum, self.maximum
            ),
        })
    }
}
