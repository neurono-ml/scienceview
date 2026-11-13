//! The typed options a joint plot exposes beyond its column names.

use crate::sv_stat::SVFitModel;

/// The form the central bivariate panel takes.
///
/// Every variant is a composition of existing kernels or recipes: the scatter
/// draws the dot kernel, the hexbin delegates to the hexagonal-binning recipe,
/// the density contour delegates to the contour recipe, and the regression
/// draws the fit kernel's curve.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVJointKind {
    /// One dot per observation (default).
    #[default]
    Scatter,
    /// Hexagonal binning of the two variables.
    Hexbin,
    /// Two-dimensional density contour bands.
    DensityContour,
    /// A fitted model over the observations.
    Regression,
}

impl SVJointKind {
    /// Returns the option name as written in spans and errors.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Scatter => "Scatter",
            Self::Hexbin => "Hexbin",
            Self::DensityContour => "DensityContour",
            Self::Regression => "Regression",
        }
    }
}

/// The form the two marginal panels take.
///
/// `Auto` resolves to the univariate distribution that matches the central
/// form: a histogram for a scatter, a hexbin or a regression, and a density
/// curve for a density contour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVJointMarginal {
    /// Choose the marginal from the central form (default).
    #[default]
    Auto,
    /// Binned bars.
    Histogram,
    /// A kernel density curve.
    Density,
    /// One tick per observation.
    Rug,
}

impl SVJointMarginal {
    /// Returns the option name as written in spans and errors.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Histogram => "Histogram",
            Self::Density => "Density",
            Self::Rug => "Rug",
        }
    }

    /// Resolves `Auto` against the configured central form.
    pub fn resolve(self, kind: SVJointKind) -> Self {
        match self {
            Self::Auto => match kind {
                SVJointKind::DensityContour => Self::Density,
                SVJointKind::Scatter | SVJointKind::Hexbin | SVJointKind::Regression => {
                    Self::Histogram
                }
            },
            explicit => explicit,
        }
    }
}

/// The closed set of regression models a joint plot can request.
///
/// The enumeration is the stat kernel's own model set, re-exposed so a caller
/// configures the joint plot without importing the kernel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVJointRegressionModel {
    /// A straight line (default).
    #[default]
    Linear,
    /// A polynomial of the requested order.
    Polynomial,
    /// An exponential curve.
    Exponential,
    /// A logarithm.
    Logarithmic,
    /// A power law.
    Power,
}

impl SVJointRegressionModel {
    /// Returns the kernel model this option selects.
    pub fn as_fit_model(self) -> SVFitModel {
        match self {
            Self::Linear => SVFitModel::Linear,
            Self::Polynomial => SVFitModel::Polynomial,
            Self::Exponential => SVFitModel::Exponential,
            Self::Logarithmic => SVFitModel::Logarithmic,
            Self::Power => SVFitModel::Power,
        }
    }
}
