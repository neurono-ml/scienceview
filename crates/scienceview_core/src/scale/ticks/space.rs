//! The space the layout engine grants an axis, and the tick budget it implies.

use crate::sv_error::SVError;

/// The space an axis has, and how many ticks fit in it.
///
/// Tick count is a layout decision (design D4): the scale never picks a number
/// of ticks on its own. The layout engine reports the length it granted the
/// axis and the minimum spacing a label needs; the scale turns that into a
/// budget, which is why a narrow axis stays readable with fewer ticks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVTickSpace {
    available_length: f64,
    minimum_tick_spacing: f64,
}

impl SVTickSpace {
    /// Builds a tick space.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when either length is not a
    /// positive finite number.
    pub fn new(available_length: f64, minimum_tick_spacing: f64) -> Result<Self, SVError> {
        if !available_length.is_finite() || available_length <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "an axis needs a positive finite length, received {available_length}"
                ),
            });
        }
        if !minimum_tick_spacing.is_finite() || minimum_tick_spacing <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "a tick needs a positive finite spacing, received {minimum_tick_spacing}"
                ),
            });
        }

        Ok(Self {
            available_length,
            minimum_tick_spacing,
        })
    }

    /// Returns the length the layout engine granted the axis.
    pub fn available_length(&self) -> f64 {
        self.available_length
    }

    /// Returns the minimum spacing a tick label needs.
    pub fn minimum_tick_spacing(&self) -> f64 {
        self.minimum_tick_spacing
    }

    /// Returns how many ticks fit, at least one.
    pub fn maximum_number_of_ticks(&self) -> usize {
        let count = (self.available_length / self.minimum_tick_spacing).floor();
        if count.is_finite() && count >= 1.0 {
            count as usize
        } else {
            1
        }
    }
}
