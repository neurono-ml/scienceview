//! The continuous colour scale the cell values are painted through.

use super::configuration::SVDensityHeatmapConfiguration;
use super::grid::SVDensityHeatmapGrid;
use crate::sv_error::SVError;
use crate::sv_scale::{
    SVColour, SVContinuousColourScale, SVContinuousColourScaleBuilder, SVScaleTransform,
};

/// The value domain and the continuous colour scale built over it.
#[derive(Clone, Debug)]
pub(crate) struct SVHeatmapColourScale {
    pub(crate) lower: f64,
    pub(crate) upper: f64,
    pub(crate) scale: SVContinuousColourScale,
}

impl SVHeatmapColourScale {
    /// Returns the colour of one cell value.
    pub(crate) fn colour(&self, value: f64) -> Result<SVColour, SVError> {
        self.scale.colour(value)
    }

    /// Returns the colour of the lowest value of the domain.
    pub(crate) fn lowest_colour(&self) -> Result<SVColour, SVError> {
        self.scale.colour(self.lower)
    }
}

/// Builds the colour scale from the occupied cells of every grid.
pub(crate) fn sv_build_colour_scale(
    configuration: &SVDensityHeatmapConfiguration<'_>,
    grids: &[SVDensityHeatmapGrid],
) -> Result<SVHeatmapColourScale, SVError> {
    let (lower, upper) = sv_occupied_range(grids);
    let upper = if upper > lower { upper } else { lower + 1.0 };
    let scale = SVContinuousColourScaleBuilder::new()
        .domain(lower, upper)
        .transform(SVScaleTransform::Linear)
        .colour_map(configuration.colour_map.clone())
        .nice(false)
        .build()?;
    Ok(SVHeatmapColourScale {
        lower,
        upper,
        scale,
    })
}

fn sv_occupied_range(grids: &[SVDensityHeatmapGrid]) -> (f64, f64) {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for grid in grids {
        for value in grid.values() {
            if value.is_finite() && *value > 0.0 {
                lower = lower.min(*value);
                upper = upper.max(*value);
            }
        }
    }
    if !lower.is_finite() || !upper.is_finite() {
        return (0.0, 1.0);
    }
    (lower, upper)
}
