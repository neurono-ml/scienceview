//! Level selection and the colour scale the levels are painted through.

use super::configuration::SVContourConfiguration;
use super::field::SVContourField;
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, SVContinuousColourScale, SVContinuousColourScaleBuilder};

/// The thresholds a contour composition paints.
#[derive(Clone, Debug)]
pub(crate) struct SVContourLevels {
    pub(crate) thresholds: Vec<f64>,
    pub(crate) lower: f64,
    pub(crate) upper: f64,
    pub(crate) colour_scale: SVContinuousColourScale,
}

impl SVContourLevels {
    /// Returns one colour per band: the background band first, then one per
    /// threshold.
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

    /// Formats a threshold for a drawn label.
    pub(crate) fn label(threshold: f64) -> String {
        let formatted = format!("{threshold:.3}");
        formatted
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }

    fn boundaries(&self) -> Vec<f64> {
        let mut boundaries = Vec::with_capacity(self.thresholds.len() + 2);
        boundaries.push(self.lower);
        boundaries.extend(self.thresholds.iter().copied());
        boundaries.push(self.upper);
        boundaries
    }
}

/// Selects the thresholds from the requested count, size and the field range.
pub(crate) fn sv_select_levels(
    configuration: &SVContourConfiguration<'_>,
    field: &SVContourField,
) -> Result<SVContourLevels, SVError> {
    let (observed_lower, observed_upper) = sv_field_range(field);
    let (lower, upper) = configuration
        .level_range
        .unwrap_or((observed_lower, observed_upper));
    let upper = if upper > lower { upper } else { lower + 1.0 };
    let thresholds = match configuration.level_size {
        Some(level_size) => sv_spaced_thresholds(lower, upper, level_size),
        None => sv_counted_thresholds(lower, upper, configuration.levels.max(1)),
    };

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

fn sv_counted_thresholds(lower: f64, upper: f64, count: usize) -> Vec<f64> {
    (1..=count)
        .map(|position| lower + (upper - lower) * position as f64 / (count + 1) as f64)
        .collect()
}

fn sv_spaced_thresholds(lower: f64, upper: f64, level_size: f64) -> Vec<f64> {
    let mut thresholds = Vec::new();
    let mut position = 1usize;
    loop {
        let threshold = lower + level_size * position as f64;
        if threshold >= upper {
            break;
        }
        thresholds.push(threshold);
        position += 1;
        if position > 1_000_000 {
            break;
        }
    }
    if thresholds.is_empty() {
        thresholds.push((lower + upper) / 2.0);
    }
    thresholds
}

fn sv_field_range(field: &SVContourField) -> (f64, f64) {
    let lower = field.lower_value();
    let upper = field.upper_value();
    if !lower.is_finite() || !upper.is_finite() || upper <= lower {
        return (0.0, 1.0);
    }
    (lower, upper)
}
