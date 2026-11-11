//! A continuous scale whose range is a colour map instead of a position.

use super::{SVColour, SVColourMap};
use crate::sv_error::SVError;
use crate::sv_scale::{SVContinuousScale, SVDomain, SVScaleTransform};

/// Maps numeric values onto a continuous colour map.
///
/// A value scale owns the domain and the transform; the colour map owns the
/// interpolation. The two are distinct because a continuous colour encoding
/// interpolates, while a nominal one never does.
///
/// The direct constructor is private: [`SVContinuousColourScaleBuilder`] is the
/// only public way to obtain a colour scale.
///
/// [`SVContinuousColourScaleBuilder`]: super::SVContinuousColourScaleBuilder
#[derive(Clone, Debug, PartialEq)]
pub struct SVContinuousColourScale {
    value_scale: SVContinuousScale,
    colour_map: SVColourMap,
}

impl SVContinuousColourScale {
    /// Assembles a validated colour scale.
    pub(crate) fn new(value_scale: SVContinuousScale, colour_map: SVColourMap) -> Self {
        Self {
            value_scale,
            colour_map,
        }
    }

    /// Returns the data interval.
    pub fn domain(&self) -> SVDomain {
        self.value_scale.domain()
    }

    /// Returns the transform applied to the domain.
    pub fn transform(&self) -> SVScaleTransform {
        self.value_scale.transform()
    }

    /// Returns the colour map.
    pub fn colour_map(&self) -> &SVColourMap {
        &self.colour_map
    }

    /// Returns the fraction of the colour map a value occupies.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the value is not finite
    /// or has no position under the transform.
    pub fn fraction(&self, value: f64) -> Result<f64, SVError> {
        self.value_scale.fraction(value)
    }

    /// Returns the colour of a value.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the value has no
    /// fraction.
    pub fn colour(&self, value: f64) -> Result<SVColour, SVError> {
        Ok(self.colour_map.colour_at(self.fraction(value)?))
    }

    /// Returns the colour of a value, re-resolving a theme-bound map first.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the value has no fraction
    /// or when the theme no longer carries the map's token.
    #[cfg(feature = "theme")]
    pub fn colour_with_theme(
        &self,
        value: f64,
        theme: &scienceview_theme::SVTheme,
    ) -> Result<SVColour, SVError> {
        let fraction = self.fraction(value)?;
        self.colour_map.colour_at_with_theme(fraction, theme)
    }
}
