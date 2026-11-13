//! The extrapolation policy of the fitted curve.

/// How the fitted curve relates to the visible horizontal domain.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVExtrapolationPolicy {
    /// Draw the curve across the observed predictor range (default).
    ///
    /// The fit kernel evaluates its grid over the observations, so this policy
    /// never claims a prediction outside the data it was fitted to.
    #[default]
    FitRange,
    /// Clip the curve to the explicit visible domain.
    ///
    /// Samples of the kernel grid outside
    /// [`SVRegressionConfiguration::visible_x_domain`] are not drawn. The
    /// recipe never extrapolates a model beyond its fitted range: the kernel
    /// does not produce predictions there, and inventing them here would
    /// duplicate the fit.
    ClipToVisibleDomain,
}

impl SVExtrapolationPolicy {
    /// Returns the policy name as written in docs and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVExtrapolationPolicy::FitRange => "FitRange",
            SVExtrapolationPolicy::ClipToVisibleDomain => "ClipToVisibleDomain",
        }
    }
}
