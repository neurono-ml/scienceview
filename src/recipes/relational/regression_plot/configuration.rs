//! The configuration surface of [`SVRegressionPlot`](super::SVRegressionPlot).
//!
//! The configuration is assembled by the builder and validated at `build`; it
//! names columns and options only, never geometry. The values mirror the FDR's
//! configuration table, so a reader of the record can find every option here.

pub(crate) use super::extrapolation::SVExtrapolationPolicy;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::SVDotShape;
use scienceview_core::sv_stat::SVFitModel;

/// The validated configuration of one regression chart.
#[derive(Clone, Debug, PartialEq)]
pub struct SVRegressionConfiguration {
    pub(crate) x: Option<String>,
    pub(crate) y: Option<String>,
    pub(crate) series: Option<String>,
    pub(crate) model: SVFitModel,
    pub(crate) order: usize,
    pub(crate) confidence: Option<f64>,
    pub(crate) show_points: bool,
    pub(crate) show_fit: bool,
    pub(crate) marker: SVDotShape,
    pub(crate) line_width: SVLength,
    pub(crate) extrapolation: SVExtrapolationPolicy,
    pub(crate) visible_x_domain: Option<(f64, f64)>,
}

impl Default for SVRegressionConfiguration {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            series: None,
            model: SVFitModel::Linear,
            order: 1,
            confidence: None,
            show_points: true,
            show_fit: true,
            marker: SVDotShape::Circle,
            line_width: SVLength::absolute(2.0),
            extrapolation: SVExtrapolationPolicy::FitRange,
            visible_x_domain: None,
        }
    }
}

impl SVRegressionConfiguration {
    /// Starts the documented default configuration without bound columns.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the predictor column name, when one was bound.
    pub fn x(&self) -> Option<&str> {
        self.x.as_deref()
    }

    /// Returns the response column name, when one was bound.
    pub fn y(&self) -> Option<&str> {
        self.y.as_deref()
    }

    /// Returns the grouping column name, when one was bound.
    pub fn series(&self) -> Option<&str> {
        self.series.as_deref()
    }

    /// Returns the configured model.
    pub fn model(&self) -> SVFitModel {
        self.model
    }

    /// Returns the configured polynomial order.
    pub fn order(&self) -> usize {
        self.order
    }

    /// Returns the configured confidence level, when a band is requested.
    pub fn confidence(&self) -> Option<f64> {
        self.confidence
    }

    /// Returns whether the scatter points are drawn.
    pub fn show_points(&self) -> bool {
        self.show_points
    }

    /// Returns whether the fitted line is drawn.
    pub fn show_fit(&self) -> bool {
        self.show_fit
    }

    /// Returns the configured dot shape.
    pub fn marker(&self) -> SVDotShape {
        self.marker
    }

    /// Returns the configured fit line width.
    pub fn line_width(&self) -> SVLength {
        self.line_width
    }

    /// Returns the extrapolation policy.
    pub fn extrapolation(&self) -> SVExtrapolationPolicy {
        self.extrapolation
    }

    /// Returns the explicit visible horizontal domain, when one was set.
    pub fn visible_x_domain(&self) -> Option<(f64, f64)> {
        self.visible_x_domain
    }

    /// Validates the option values that do not depend on the data source.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`](scienceview_core::sv_error::SVError::InvalidConfiguration)
    /// for missing or equal columns, a zero polynomial order, a confidence
    /// level outside `(0, 1)`, an inverted visible domain or a chart that would
    /// draw nothing.
    pub(crate) fn validate(&self) -> Result<(), scienceview_core::sv_error::SVError> {
        let x = self.x.as_deref().ok_or_else(|| {
            scienceview_core::sv_error::SVError::InvalidConfiguration {
                message: "a regression plot needs a predictor column through `x`".to_string(),
            }
        })?;
        let y = self.y.as_deref().ok_or_else(|| {
            scienceview_core::sv_error::SVError::InvalidConfiguration {
                message: "a regression plot needs a response column through `y`".to_string(),
            }
        })?;
        if x == y {
            return Err(scienceview_core::sv_error::SVError::InvalidConfiguration {
                message: "the predictor and the response must be different columns".to_string(),
            });
        }
        if self.model == SVFitModel::Polynomial && self.order == 0 {
            return Err(scienceview_core::sv_error::SVError::InvalidConfiguration {
                message: "a polynomial regression needs an order of at least one".to_string(),
            });
        }
        if let Some(confidence) = self.confidence {
            if !confidence.is_finite() || !(0.0..1.0).contains(&confidence) {
                return Err(scienceview_core::sv_error::SVError::InvalidConfiguration {
                    message: "the confidence level must lie strictly inside (0, 1)".to_string(),
                });
            }
            if !matches!(self.model, SVFitModel::Linear | SVFitModel::Polynomial) {
                return Err(scienceview_core::sv_error::SVError::InvalidConfiguration {
                    message: format!(
                        "`{}` does not carry a computed confidence band",
                        self.model.as_str()
                    ),
                });
            }
        }
        if !self.show_points && !self.show_fit {
            return Err(scienceview_core::sv_error::SVError::InvalidConfiguration {
                message: "a regression plot must draw its points, its fit, or both".to_string(),
            });
        }
        if let Some((lower, upper)) = self.visible_x_domain {
            if !lower.is_finite() || !upper.is_finite() || lower >= upper {
                return Err(scienceview_core::sv_error::SVError::InvalidConfiguration {
                    message: "the visible horizontal domain must be finite and strictly increasing"
                        .to_string(),
                });
            }
        }
        Ok(())
    }
}
