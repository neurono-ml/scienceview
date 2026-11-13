//! Domain derivation and the two scales a band plot places through.

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

use super::configuration::SVBandPlotConfiguration;
use super::data_contract::SVBandData;

/// The data-space extent of the x channel and of both boundaries.
pub(crate) struct SVBandDomain {
    pub(crate) x_minimum: f64,
    pub(crate) x_maximum: f64,
    pub(crate) y_minimum: f64,
    pub(crate) y_maximum: f64,
}

impl SVBandDomain {
    fn empty() -> Self {
        Self {
            x_minimum: f64::INFINITY,
            x_maximum: f64::NEG_INFINITY,
            y_minimum: f64::INFINITY,
            y_maximum: f64::NEG_INFINITY,
        }
    }

    fn is_empty(&self) -> bool {
        !(self.x_minimum.is_finite()
            && self.x_maximum.is_finite()
            && self.y_minimum.is_finite()
            && self.y_maximum.is_finite())
    }
}

/// Derives the extent of every band without densifying a sparse column.
pub(crate) fn sv_band_domain(data: &mut SVBandData<'_>) -> Result<SVBandDomain, SVError> {
    let mut domain = SVBandDomain::empty();
    for band in &mut data.bands {
        let length = band.x.view().length();
        for index in 0..length {
            let x = band.x.view().value_at(index).unwrap_or(f64::NAN);
            let lower = band.lower.view().value_at(index).unwrap_or(f64::NAN);
            let upper = band.upper.view().value_at(index).unwrap_or(f64::NAN);
            if x.is_finite() {
                domain.x_minimum = domain.x_minimum.min(x);
                domain.x_maximum = domain.x_maximum.max(x);
            }
            if lower.is_finite() {
                domain.y_minimum = domain.y_minimum.min(lower);
                domain.y_maximum = domain.y_maximum.max(lower);
            }
            if upper.is_finite() {
                domain.y_minimum = domain.y_minimum.min(upper);
                domain.y_maximum = domain.y_maximum.max(upper);
            }
        }
    }
    if domain.is_empty() {
        return Err(SVError::EmptyInput {
            operation: "band plot",
        });
    }
    Ok(domain)
}

/// Builds the horizontal scale.
pub(crate) fn sv_horizontal_scale<'scale>(
    configuration: &SVBandPlotConfiguration<'_>,
    domain: &SVBandDomain,
    plot: SVResolvedRectangle,
) -> Result<SVScale<'scale>, SVError> {
    let mut builder = SVScaleBuilder::new(configuration.x_scale_kind)
        .domain(domain.x_minimum, domain.x_maximum)
        .range(plot.x, plot.right());
    if configuration.x_scale_kind == SVScaleKind::Temporal {
        builder = builder.temporal_unit(configuration.temporal_unit);
    }
    builder.build()
}

/// Builds the vertical scale, with the display range inverted.
pub(crate) fn sv_vertical_scale<'scale>(
    domain: &SVBandDomain,
    plot: SVResolvedRectangle,
) -> Result<SVScale<'scale>, SVError> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(domain.y_minimum, domain.y_maximum)
        .range(plot.bottom(), plot.y)
        .build()
}
/// Solves the plot rectangle inside the viewport.
pub(crate) fn sv_plot_area(viewport: SVViewport) -> SVResolvedRectangle {
    let left = 72.0;
    let top = 56.0;
    let width = (viewport.width() - 148.0).max(1.0);
    let height = (viewport.height() - 128.0).max(1.0);
    SVResolvedRectangle::new(left, top, width, height)
}
