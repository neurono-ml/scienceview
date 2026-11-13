//! Level selection and the colour scale the levels are painted through.

use super::configuration::SVDensityContourConfiguration;
use super::grid::SVDensityGrid;
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, SVContinuousColourScale, SVContinuousColourScaleBuilder};

/// The thresholds a density-contour composition paints.
#[derive(Clone, Debug)]
pub(crate) struct SVContourLevels {
    pub(crate) thresholds: Vec<f64>,
    pub(crate) lower: f64,
    pub(crate) upper: f64,
    pub(crate) colour_scale: SVContinuousColourScale,
}

impl SVContourLevels {
    /// Returns `levels + 1` band colours: the background band first, then one
    /// per threshold.
    pub(crate) fn band_colours(&self) -> Result<Vec<SVColour>, SVError> {
        let boundaries = self.boundaries();
        boundaries
            .windows(2)
            .map(|band| self.colour_scale.colour((band[0] + band[1]) / 2.0))
            .collect()
    }

    /// Returns the colour of one threshold line.
    pub(crate) fn line_colour(&self, threshold: f64) -> Result<SVColour, SVError> {
        self.colour_scale.colour(threshold)
    }

    fn boundaries(&self) -> Vec<f64> {
        let mut boundaries = Vec::with_capacity(self.thresholds.len() + 2);
        boundaries.push(self.lower);
        boundaries.extend(self.thresholds.iter().copied());
        boundaries.push(self.upper);
        boundaries
    }
}

/// Selects the thresholds from the requested count and the density range.
pub(crate) fn sv_select_levels(
    configuration: &SVDensityContourConfiguration<'_>,
    grids: &[SVDensityGrid],
) -> Result<SVContourLevels, SVError> {
    let (observed_lower, observed_upper) = sv_density_range(grids);
    let (lower, upper) = configuration
        .level_range
        .unwrap_or((observed_lower, observed_upper));
    let upper = if upper > lower { upper } else { lower + 1.0 };
    let count = configuration.levels.max(1);
    let thresholds: Vec<f64> = (1..=count)
        .map(|position| lower + (upper - lower) * position as f64 / (count + 1) as f64)
        .collect();

    let colour_scale = SVContinuousColourScaleBuilder::new()
        .domain(lower, upper)
        .transform(configuration.colour_scale)
        .colour_map(configuration.colour_map.clone())
        .nice(false)
        .build()?;

    Ok(SVContourLevels {
        thresholds,
        lower,
        upper,
        colour_scale,
    })
}

fn sv_density_range(grids: &[SVDensityGrid]) -> (f64, f64) {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for grid in grids {
        for density in grid.densities() {
            if density.is_finite() {
                lower = lower.min(*density);
                upper = upper.max(*density);
            }
        }
    }
    if !lower.is_finite() || !upper.is_finite() {
        return (0.0, 1.0);
    }
    (lower, upper)
}
