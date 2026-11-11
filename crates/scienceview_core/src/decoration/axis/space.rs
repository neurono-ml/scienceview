//! The length the layout engine granted an axis.

use crate::sv_error::SVError;

/// The plot extent an axis is measured against.
///
/// The layout engine solves the plot area first and hands its extent to the
/// axis; the axis never guesses a length. A degenerate or negative extent is a
/// configuration failure, because an axis without length has no tick budget to
/// spend.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVAxisSpace {
    plot_extent: f64,
}

impl SVAxisSpace {
    /// Builds the space an axis has along its own direction.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the extent is not a
    /// positive finite number.
    pub fn new(plot_extent: f64) -> Result<Self, SVError> {
        if !plot_extent.is_finite() || plot_extent <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "an axis needs a positive finite plot extent, received {plot_extent}"
                ),
            });
        }

        Ok(Self { plot_extent })
    }

    /// Returns the extent of the plot area along the axis direction.
    pub fn plot_extent(&self) -> f64 {
        self.plot_extent
    }
}
