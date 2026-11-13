//! The scales and the display-space geometry the swarm move needs.
//!
//! A move works in data-space units, so the recipe converts the marker diameter
//! and the category slot into those units before it asks the kernel for a
//! layout. The conversion is what makes the kernel's collision test identical to
//! the pixel-space non-overlap test.

use super::channels::SVSwarmObservation;
use crate::sv_error::SVError;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_mark::{SVBarOrientation, SVMarkTheme};
use crate::sv_scale::{
    SVDomain, SVScale, SVScaleBuilder, SVScaleKind, SVScaleTransform, sv_nice_domain,
};

/// The three scales one render needs plus the solved marker geometry.
pub(super) struct SVSwarmScales<'labels> {
    pub(super) category: SVScale<'labels>,
    pub(super) value: SVScale<'labels>,
    pub(super) nominal: SVScale<'labels>,
    pub(super) value_domain: SVDomain,
    pub(super) marker_diameter: f64,
    pub(super) marker_extent_value: f64,
    pub(super) marker_extent_category: f64,
}

/// Builds the scales and derives the move's data-space extents.
pub(super) fn sv_build_scales<'labels>(
    observations: &[SVSwarmObservation],
    labels: &'labels [&'labels str],
    orientation: SVBarOrientation,
    marker_size: Option<SVLength>,
    plot: SVResolvedRectangle,
) -> Result<SVSwarmScales<'labels>, SVError> {
    let category_count = labels.len().max(1);
    let value_domain = sv_value_domain(observations)?;
    let category = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(-0.5, category_count as f64 - 0.5)
        .range(
            match orientation {
                SVBarOrientation::Vertical => plot.x,
                SVBarOrientation::Horizontal => plot.bottom(),
            },
            match orientation {
                SVBarOrientation::Vertical => plot.right(),
                SVBarOrientation::Horizontal => plot.y,
            },
        )
        .build()?;
    let value = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(value_domain.lower(), value_domain.upper())
        .range(
            match orientation {
                SVBarOrientation::Vertical => plot.bottom(),
                SVBarOrientation::Horizontal => plot.x,
            },
            match orientation {
                SVBarOrientation::Vertical => plot.y,
                SVBarOrientation::Horizontal => plot.right(),
            },
        )
        .build()?;
    let (nominal_start, nominal_end) = match orientation {
        SVBarOrientation::Vertical => {
            let inset = plot.width / (2.0 * category_count as f64);
            (inset, plot.width - inset)
        }
        SVBarOrientation::Horizontal => {
            let inset = plot.height / (2.0 * category_count as f64);
            (plot.height - inset, inset)
        }
    };
    let nominal = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(labels)
        .range(nominal_start, nominal_end)
        .build()?;

    let theme = SVMarkTheme::default();
    let marker_diameter = marker_size
        .unwrap_or_else(|| SVLength::absolute(theme.mark_size()))
        .resolve_width(plot.width, theme.mark_size(), plot.height);
    if !marker_diameter.is_finite() || marker_diameter <= 0.0 {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "the swarm marker size resolves to {marker_diameter} logical pixels; a marker needs a positive finite diameter"
            ),
        });
    }

    let category_extent = match orientation {
        SVBarOrientation::Vertical => plot.width,
        SVBarOrientation::Horizontal => plot.height,
    };
    let value_extent = match orientation {
        SVBarOrientation::Vertical => plot.height,
        SVBarOrientation::Horizontal => plot.width,
    };
    let value_pixels_per_unit = value_extent / value_domain.length();
    let marker_extent_value = marker_diameter / value_pixels_per_unit;
    let marker_extent_category = marker_diameter * category_count as f64 / category_extent;

    Ok(SVSwarmScales {
        category,
        value,
        nominal,
        value_domain,
        marker_diameter,
        marker_extent_value,
        marker_extent_category,
    })
}

/// Derives the value domain from the observations the swarm actually draws.
///
/// A degenerate sample — one distinct value, or none — is widened around its
/// value, because a zero-width axis would collapse the swarm into a line and
/// leave the move without a pixel to measure its marker against.
fn sv_value_domain(observations: &[SVSwarmObservation]) -> Result<SVDomain, SVError> {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for value in observations.iter().map(|observation| observation.value) {
        lower = lower.min(value);
        upper = upper.max(value);
    }
    if !lower.is_finite() || !upper.is_finite() {
        return SVDomain::new(0.0, 1.0);
    }
    if lower == upper {
        let half_span = if lower == 0.0 { 0.5 } else { lower.abs() * 0.1 };
        return SVDomain::new(lower - half_span, upper + half_span);
    }
    Ok(sv_nice_domain(
        SVDomain::new(lower, upper)?,
        SVScaleTransform::Linear,
    ))
}

/// Converts the configured width fraction into the move's data-space width.
///
/// `None` uses the whole category slot, which is the FDR's default. The value is
/// additionally capped at the f32 range the kernel option takes, so the typed
/// width validation inside the kernel still reports what it received.
pub(super) fn sv_swarm_width_value(
    fraction: Option<f32>,
    scales: &SVSwarmScales<'_>,
    category_count: usize,
    orientation: SVBarOrientation,
    plot: SVResolvedRectangle,
) -> f64 {
    let category_extent = match orientation {
        SVBarOrientation::Vertical => plot.width,
        SVBarOrientation::Horizontal => plot.height,
    };
    let slot_pixels = category_extent / category_count.max(1) as f64;
    let width_pixels = slot_pixels * f64::from(fraction.unwrap_or(1.0));
    let value_extent = match orientation {
        SVBarOrientation::Vertical => plot.height,
        SVBarOrientation::Horizontal => plot.width,
    };
    width_pixels * scales.value_domain.length() / value_extent
}
