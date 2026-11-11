//! The four transforms a continuous scale can apply to its domain.

use super::SVDomain;

/// The linear threshold of the symmetric logarithm.
///
/// Below this magnitude the symmetric logarithm is almost linear, which is why
/// it can represent zero and negative values at all.
const SYMMETRIC_LOGARITHM_LINEAR_THRESHOLD: f64 = 1.0;

/// How a continuous scale compresses its domain before projecting it.
///
/// Every transform exposes a forward map and its inverse, so a value can be
/// placed on an axis and a position can be read back. A transform that has no
/// value at a point answers `None` rather than producing a `NaN`, which keeps
/// the failure at the configuration boundary.
///
/// | Transform | Forward | Defined on |
/// | --- | --- | --- |
/// | [`SVScaleTransform::Linear`] | the value itself | every finite value |
/// | [`SVScaleTransform::Logarithmic`] | base-10 logarithm | strictly positive values |
/// | [`SVScaleTransform::SymmetricLogarithmic`] | sign-preserving logarithm | every finite value |
/// | [`SVScaleTransform::SquareRoot`] | square root | non-negative values |
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVScaleTransform {
    /// No compression: the domain maps linearly onto the range.
    #[default]
    Linear,
    /// Base-10 logarithm, for data that spans orders of magnitude.
    Logarithmic,
    /// A sign-preserving logarithm that also accepts zero and negatives.
    SymmetricLogarithmic,
    /// Square root, which softens the upper end of the domain.
    SquareRoot,
}

impl SVScaleTransform {
    /// Returns the transform name as written in observability spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVScaleTransform::Linear => "Linear",
            SVScaleTransform::Logarithmic => "Logarithmic",
            SVScaleTransform::SymmetricLogarithmic => "SymmetricLogarithmic",
            SVScaleTransform::SquareRoot => "SquareRoot",
        }
    }

    /// Maps a value into the transform's linear space.
    pub fn forward(&self, value: f64) -> Option<f64> {
        if !value.is_finite() {
            return None;
        }

        match self {
            SVScaleTransform::Linear => Some(value),
            SVScaleTransform::Logarithmic if value > 0.0 => Some(value.log10()),
            SVScaleTransform::Logarithmic => None,
            SVScaleTransform::SymmetricLogarithmic => {
                let magnitude = (1.0 + value.abs() / SYMMETRIC_LOGARITHM_LINEAR_THRESHOLD).log10();
                Some(value.signum() * magnitude)
            }
            SVScaleTransform::SquareRoot if value >= 0.0 => Some(value.sqrt()),
            SVScaleTransform::SquareRoot => None,
        }
    }

    /// Maps a value of the transform's linear space back to the data space.
    pub fn inverse(&self, value: f64) -> Option<f64> {
        if !value.is_finite() {
            return None;
        }

        let restored = match self {
            SVScaleTransform::Linear => value,
            SVScaleTransform::Logarithmic => 10.0_f64.powf(value),
            SVScaleTransform::SymmetricLogarithmic => {
                value.signum()
                    * SYMMETRIC_LOGARITHM_LINEAR_THRESHOLD
                    * (10.0_f64.powf(value.abs()) - 1.0)
            }
            SVScaleTransform::SquareRoot if value >= 0.0 => value * value,
            SVScaleTransform::SquareRoot => return None,
        };

        restored.is_finite().then_some(restored)
    }

    /// Reports whether the transform has a value on a whole domain.
    pub fn domain_is_defined(&self, domain: SVDomain) -> bool {
        match self {
            SVScaleTransform::Linear | SVScaleTransform::SymmetricLogarithmic => true,
            SVScaleTransform::Logarithmic => domain.lower() > 0.0,
            SVScaleTransform::SquareRoot => domain.lower() >= 0.0,
        }
    }
}

impl std::fmt::Display for SVScaleTransform {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
