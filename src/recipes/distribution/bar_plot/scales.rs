//! Domain derivation and the scales a bar plot places through.

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::SVBarOrientation;
use scienceview_core::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

use super::configuration::SVBarPlotConfiguration;
use super::statistics::SVBarSummary;

/// The data-space extent every bar and error bar reaches.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SVBarDomain {
    pub(crate) minimum: f64,
    pub(crate) maximum: f64,
}

impl SVBarDomain {
    pub(crate) fn from_summary(summary: &SVBarSummary) -> Result<Self, SVError> {
        let mut minimum = 0.0f64;
        let mut maximum = 0.0f64;
        for bin in summary.bins() {
            minimum = minimum.min(bin.baseline()).min(bin.top());
            maximum = maximum.max(bin.baseline()).max(bin.top());
            if let Some(error) = bin.error() {
                minimum = minimum.min(error.lower());
                maximum = maximum.max(error.upper());
            }
        }
        if !minimum.is_finite() || !maximum.is_finite() {
            return Err(SVError::EmptyInput {
                operation: "bar plot",
            });
        }
        if maximum - minimum <= f64::EPSILON {
            let padding = if minimum.abs() <= f64::EPSILON {
                1.0
            } else {
                minimum.abs() * 0.1
            };
            minimum -= padding;
            maximum += padding;
        }
        Ok(Self { minimum, maximum })
    }
}

/// The three scales one render needs.
pub(crate) struct SVBarScales<'labels> {
    pub(crate) value: SVScale<'labels>,
    pub(crate) category: SVScale<'labels>,
    pub(crate) nominal: SVScale<'labels>,
}

/// Builds the value scale, the category placement scale and the nominal axis.
pub(crate) fn sv_build_scales<'labels>(
    configuration: &SVBarPlotConfiguration<'_>,
    summary: &SVBarSummary,
    domain: SVBarDomain,
    labels: &'labels [&'labels str],
    plot: SVResolvedRectangle,
) -> Result<SVBarScales<'labels>, SVError> {
    let value = match configuration.orientation {
        SVBarOrientation::Vertical => SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(domain.minimum, domain.maximum)
            .range(plot.bottom(), plot.y)
            .build()?,
        SVBarOrientation::Horizontal => SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(domain.minimum, domain.maximum)
            .range(plot.x, plot.right())
            .build()?,
    };

    let category_count = summary.categories().len().max(1) as f64;
    let (category_start, category_end) = match configuration.orientation {
        SVBarOrientation::Vertical => (plot.x, plot.right()),
        SVBarOrientation::Horizontal => (plot.bottom(), plot.y),
    };
    let category = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(-0.5, category_count - 0.5)
        .range(category_start, category_end)
        .build()?;

    let (nominal_start, nominal_end) = match configuration.orientation {
        SVBarOrientation::Vertical => {
            let inset = plot.width / (2.0 * category_count);
            (inset, plot.width - inset)
        }
        SVBarOrientation::Horizontal => {
            let inset = plot.height / (2.0 * category_count);
            (plot.height - inset, inset)
        }
    };
    let nominal = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(labels)
        .range(nominal_start, nominal_end)
        .build()?;

    Ok(SVBarScales {
        value,
        category,
        nominal,
    })
}

/// Solves the plot rectangle inside the viewport.
pub(crate) fn sv_plot_area(viewport: SVViewport) -> SVResolvedRectangle {
    let left = 72.0;
    let top = 56.0;
    let width = (viewport.width() - 148.0).max(1.0);
    let height = (viewport.height() - 128.0).max(1.0);
    SVResolvedRectangle::new(left, top, width, height)
}
