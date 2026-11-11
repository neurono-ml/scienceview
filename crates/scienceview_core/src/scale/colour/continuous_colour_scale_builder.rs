//! Building a continuous colour scale from a domain and a colour map.

use super::{SVColourMap, SVContinuousColourScale};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_scale::{SVScaleRange, SVScaleTransform, sv_build_continuous_scale};

/// Builds an [`SVContinuousColourScale`].
///
/// The domain follows the same rules as every numeric scale: explicit when
/// given, derived from a column view otherwise, and never rounded when the
/// caller configured it.
#[derive(Clone, Debug, Default)]
pub struct SVContinuousColourScaleBuilder<'a> {
    explicit_domain: Option<(f64, f64)>,
    data_view: Option<SVColumnView<'a>>,
    transform: SVScaleTransform,
    nice: bool,
    colour_map: Option<SVColourMap>,
}

impl<'a> SVContinuousColourScaleBuilder<'a> {
    /// Starts a colour scale with no domain and no colour map.
    pub fn new() -> Self {
        Self {
            explicit_domain: None,
            data_view: None,
            transform: SVScaleTransform::Linear,
            nice: true,
            colour_map: None,
        }
    }

    /// Configures an explicit domain, which overrides any derivation.
    #[must_use]
    pub fn domain(mut self, lower: f64, upper: f64) -> Self {
        self.explicit_domain = Some((lower, upper));
        self
    }

    /// Attaches the column view a derived domain is read from.
    #[must_use]
    pub fn data_view(mut self, data_view: SVColumnView<'a>) -> Self {
        self.data_view = Some(data_view);
        self
    }

    /// Configures the transform applied to the domain.
    #[must_use]
    pub fn transform(mut self, transform: SVScaleTransform) -> Self {
        self.transform = transform;
        self
    }

    /// Enables or disables the nice rounding of a derived domain.
    #[must_use]
    pub fn nice(mut self, nice: bool) -> Self {
        self.nice = nice;
        self
    }

    /// Configures the colour map values are interpolated on.
    #[must_use]
    pub fn colour_map(mut self, colour_map: SVColourMap) -> Self {
        self.colour_map = Some(colour_map);
        self
    }

    /// Validates the configuration and builds the colour scale.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when no colour map or no
    /// domain is configured, or when the domain falls outside the transform's
    /// domain.
    pub fn build(self) -> Result<SVContinuousColourScale, SVError> {
        let colour_map = self
            .colour_map
            .ok_or_else(|| SVError::InvalidConfiguration {
                message:
                    "a continuous colour scale requires a colour map; pass one with `colour_map`"
                        .to_string(),
            })?;

        let value_scale = sv_build_continuous_scale(
            self.explicit_domain,
            self.data_view,
            self.transform,
            self.nice,
            SVScaleRange::default(),
            None,
        )?;

        Ok(SVContinuousColourScale::new(value_scale, colour_map))
    }
}
