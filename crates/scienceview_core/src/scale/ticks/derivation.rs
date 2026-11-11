//! The tick derivation hook and the numeric tick algorithms.

use super::{SVTick, SVTickFormat, SVTickSpace};
use crate::sv_scale::{
    SVBooleanScale, SVContinuousScale, SVNominalScale, SVScale, SVScaleTransform,
    domain::sv_nice_step,
};

use super::temporal_ticks::sv_temporal_ticks;

/// The interface the layout engine consumes to draw an axis.
///
/// A scale answers with values, positions and labels; the decoration layer
/// places them. The trait exists so a decoration can accept any scale through
/// one bound, and so a test can prove the hook without matching on the kind.
pub trait SVTickDerivation {
    /// Derives the ticks that fit in the granted space.
    fn ticks(&self, space: SVTickSpace) -> Vec<SVTick>;
}

impl SVTickDerivation for SVScale<'_> {
    fn ticks(&self, space: SVTickSpace) -> Vec<SVTick> {
        SVScale::ticks(self, space)
    }
}

impl SVScale<'_> {
    /// Derives the ticks that fit in the granted space, with automatic labels.
    pub fn ticks(&self, space: SVTickSpace) -> Vec<SVTick> {
        self.ticks_with_format(space, SVTickFormat::Automatic)
    }

    /// Derives the ticks that fit in the granted space with an explicit label
    /// format.
    pub fn ticks_with_format(&self, space: SVTickSpace, format: SVTickFormat) -> Vec<SVTick> {
        match self {
            SVScale::Continuous(scale) => sv_continuous_ticks(scale, space, format),
            SVScale::Nominal(scale) => sv_nominal_ticks(scale, space),
            SVScale::Temporal(scale) => sv_temporal_ticks(scale, space, format),
            SVScale::Boolean(scale) => sv_boolean_ticks(scale),
        }
    }
}

/// Resolves the tick budget: an explicit count wins over the granted space.
pub(crate) fn sv_tick_count(configured: Option<usize>, space: SVTickSpace) -> usize {
    configured
        .unwrap_or_else(|| space.maximum_number_of_ticks())
        .max(1)
}

/// Builds one tick from a value, or `None` when the value has no position.
pub(crate) fn sv_tick_from_value(
    scale: &SVContinuousScale,
    value: f64,
    format: SVTickFormat,
) -> Option<SVTick> {
    let position = scale.position(value).ok()?;
    Some(SVTick::new(value, position, format.format(value)))
}

fn sv_continuous_ticks(
    scale: &SVContinuousScale,
    space: SVTickSpace,
    format: SVTickFormat,
) -> Vec<SVTick> {
    let domain = scale.domain();
    if domain.is_degenerate() {
        return sv_tick_from_value(scale, domain.lower(), format)
            .into_iter()
            .collect();
    }

    let count = sv_tick_count(scale.tick_count(), space);
    let transform = scale.transform();
    let (Some(lower), Some(upper)) = (
        transform.forward(domain.lower()),
        transform.forward(domain.upper()),
    ) else {
        return Vec::new();
    };

    let values = if transform == SVScaleTransform::Logarithmic {
        sv_logarithmic_tick_values(lower, upper, count)
    } else {
        sv_linear_tick_values(lower, upper, count)
    };
    let ticks: Vec<SVTick> = values
        .into_iter()
        .filter_map(|value| transform.inverse(value))
        .filter_map(|value| sv_tick_from_value(scale, value, format))
        .collect();

    if ticks.len() >= 2 {
        return ticks;
    }

    [domain.lower(), domain.upper()]
        .into_iter()
        .filter_map(|value| sv_tick_from_value(scale, value, format))
        .collect()
}

/// Returns evenly spaced 1/2/5 ticks in a linear space.
pub(crate) fn sv_linear_tick_values(lower: f64, upper: f64, count: usize) -> Vec<f64> {
    let intervals = count.saturating_sub(1).max(1);
    let step = sv_nice_step((upper - lower) / intervals as f64);
    if !step.is_finite() || step <= 0.0 {
        return vec![lower, upper];
    }

    let start = (lower / step).ceil() * step;
    let mut values = Vec::new();
    let mut index = 0_usize;
    while index <= count.saturating_add(2) {
        let value = start + step * index as f64;
        if value > upper + step * 1e-9 {
            break;
        }
        values.push(value);
        index += 1;
    }

    if values.len() < 2 {
        vec![lower, upper]
    } else {
        values
    }
}

/// Returns the integer exponents between two logarithmic bounds.
pub(crate) fn sv_logarithmic_tick_values(lower: f64, upper: f64, count: usize) -> Vec<f64> {
    let intervals = count.saturating_sub(1).max(1);
    let step = ((upper - lower) / intervals as f64).ceil().max(1.0);
    let start = lower.ceil();
    let mut values = Vec::new();
    let mut index = 0_usize;
    while index <= count.saturating_add(2) {
        let value = start + step * index as f64;
        if value > upper + 1e-9 {
            break;
        }
        values.push(value);
        index += 1;
    }

    if values.len() < 2 {
        vec![lower, upper]
    } else {
        values
    }
}

fn sv_nominal_ticks(scale: &SVNominalScale<'_>, space: SVTickSpace) -> Vec<SVTick> {
    let categories = scale.categories();
    if categories.is_empty() {
        return Vec::new();
    }

    let budget = space.maximum_number_of_ticks();
    let stride = categories.len().div_ceil(budget).max(1);
    categories
        .iter()
        .enumerate()
        .step_by(stride)
        .filter_map(|(display_index, category)| {
            let position = scale.position(category).ok()?;
            Some(SVTick::new(
                display_index as f64,
                position,
                (*category).to_string(),
            ))
        })
        .collect()
}

fn sv_boolean_ticks(scale: &SVBooleanScale) -> Vec<SVTick> {
    [false, true]
        .into_iter()
        .map(|value| {
            SVTick::new(
                if value { 1.0 } else { 0.0 },
                scale.position(value),
                scale.label_for(value).to_string(),
            )
        })
        .collect()
}
