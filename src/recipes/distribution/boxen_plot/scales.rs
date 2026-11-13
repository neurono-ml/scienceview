//! Domain derivation and the scales a boxen plot places through.

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::SVBarOrientation;
use scienceview_core::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

use super::configuration::SVBoxenPlotConfiguration;
use super::data_contract::SVBoxenData;
use super::grouping::SVBoxenGrouping;

/// The data-space extent of every observed value, outliers included.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SVBoxenDomain {
    pub(crate) minimum: f64,
    pub(crate) maximum: f64,
}

impl SVBoxenDomain {
    pub(crate) fn from_data(data: &SVBoxenData) -> Result<Self, SVError> {
        let mut minimum = f64::INFINITY;
        let mut maximum = f64::NEG_INFINITY;
        for observation in &data.observations {
            minimum = minimum.min(observation.value);
            maximum = maximum.max(observation.value);
        }
        if !minimum.is_finite() || !maximum.is_finite() {
            return Err(SVError::EmptyInput {
                operation: "boxen plot",
            });
        }
        Ok(Self { minimum, maximum })
    }
}

/// The three scales one render needs.
pub(crate) struct SVBoxenScales<'labels> {
    pub(crate) value: SVScale<'labels>,
    pub(crate) category: SVScale<'labels>,
    pub(crate) nominal: SVScale<'labels>,
}

/// Builds the value scale, the category placement scale and the nominal axis.
pub(crate) fn sv_build_boxen_scales<'labels>(
    configuration: &SVBoxenPlotConfiguration<'_>,
    grouping: &SVBoxenGrouping,
    domain: SVBoxenDomain,
    labels: &'labels [&'labels str],
    plot: SVResolvedRectangle,
) -> Result<SVBoxenScales<'labels>, SVError> {
    let categories = grouping.categories.len().max(1);
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

    // A category slot is exactly one unit wide, and the slot of category `i`
    // is centred on `i`. Half-unit bounds keep that true for a single category
    // too, where a `0..0` domain would collapse every box to a zero-width line.
    let category_count = grouping.categories.len().max(1) as f64;
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
            let inset = plot.width / (2.0 * categories as f64);
            (inset, plot.width - inset)
        }
        SVBarOrientation::Horizontal => {
            let inset = plot.height / (2.0 * categories as f64);
            (plot.height - inset, inset)
        }
    };
    let nominal = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(labels)
        .range(nominal_start, nominal_end)
        .build()?;

    Ok(SVBoxenScales {
        value,
        category,
        nominal,
    })
}

/// Solves the plot rectangle inside the viewport.
pub(crate) fn sv_boxen_plot_area(viewport: SVViewport) -> SVResolvedRectangle {
    let left = 72.0;
    let top = 56.0;
    let width = (viewport.width() - 148.0).max(1.0);
    let height = (viewport.height() - 128.0).max(1.0);
    SVResolvedRectangle::new(left, top, width, height)
}
