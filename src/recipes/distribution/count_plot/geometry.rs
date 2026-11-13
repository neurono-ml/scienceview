//! Placing the prepared counts through the bar mark.
//!
//! One category slot is exactly one unit wide. The recipe derives the slot
//! rectangle from the category scale, hands the slot to the bar mark as its
//! area and the narrowed fraction from the move kernel as its width, so a
//! stacked or dodged bar keeps the geometry the move computed.

use super::configuration::SVCountConfiguration;
use super::preparation::{SVPreparedCount, SVPreparedSeries};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{
    SVBarOrientation, SVGeometryImage, SVImageInterpolation, SVMark, SVMarkPaint, SVMarkPrimitive,
    SVMarkRequest, SVMarkShape, SVMarkTheme,
};
use crate::sv_scale::{SVScale, SVScaleValue};

/// The data-space domains the composition solves its scales over.
pub(super) struct SVCountDomains {
    pub(super) category_low: f64,
    pub(super) category_high: f64,
    pub(super) value_low: f64,
    pub(super) value_high: f64,
}

/// Derives the domains from the prepared counts.
pub(super) fn sv_domains(prepared: &SVPreparedCount) -> SVCountDomains {
    let number_of_categories = prepared.categories.len().max(1) as f64;
    let mut value_high = 0.0f64;
    for series in &prepared.series {
        for (top, baseline) in series.tops.iter().zip(series.baselines.iter()) {
            value_high = value_high.max(*top).max(*baseline);
        }
    }
    SVCountDomains {
        category_low: -0.5,
        category_high: number_of_categories - 0.5,
        value_low: 0.0,
        value_high: if value_high > 0.0 { value_high } else { 1.0 },
    }
}

/// Builds the primitives of one prepared series.
pub(super) fn sv_series_primitives(
    configuration: &SVCountConfiguration<'_>,
    series: &SVPreparedSeries,
    number_of_series: usize,
    plot: SVResolvedRectangle,
    category: &SVScale<'_>,
    value: &SVScale<'_>,
    theme: &SVMarkTheme,
) -> Result<Vec<SVMarkPrimitive<'static>>, SVError> {
    let opacity = if configuration.multiple == super::SVCountMultiple::Layer && number_of_series > 1
    {
        0.75
    } else {
        1.0
    };
    let mut primitives = Vec::new();
    for category_index in 0..series.tops.len() {
        let center =
            category_index as f64 + series.offsets.get(category_index).copied().unwrap_or(0.0);
        let top = series.tops.get(category_index).copied().unwrap_or(0.0);
        let baseline = series.baselines.get(category_index).copied().unwrap_or(0.0);
        let width = series
            .width_fractions
            .get(category_index)
            .copied()
            .unwrap_or(f64::from(configuration.width))
            .clamp(1.0e-4, 1.0);

        let left = sv_position(category, category_index as f64 - 0.5)?;
        let right = sv_position(category, category_index as f64 + 0.5)?;
        let slot = (right - left).abs();
        let (primary, secondary, area) = match configuration.orientation {
            SVBarOrientation::Vertical => (
                vec![center],
                vec![top],
                SVResolvedRectangle::new(left.min(right), plot.y, slot, plot.height),
            ),
            SVBarOrientation::Horizontal => (
                vec![top],
                vec![center],
                SVResolvedRectangle::new(plot.x, left.min(right), plot.width, slot),
            ),
        };
        if area.width <= 0.0 || area.height <= 0.0 {
            continue;
        }

        let options = SVMark::bar()
            .orientation(configuration.orientation)
            .width(width)
            .baseline(baseline)
            .colour(series.colour)
            .opacity(opacity);
        let mark: SVMark = options.into();
        let (horizontal, vertical) = match configuration.orientation {
            SVBarOrientation::Vertical => (category, value),
            SVBarOrientation::Horizontal => (value, category),
        };
        let request = SVMarkRequest::new(
            SVColumnView::Dense(&primary),
            SVColumnView::Dense(&secondary),
            horizontal,
            vertical,
            area,
        )
        .with_theme(theme.clone());
        sv_owned_primitives(&mark.geometry(&request)?, &mut primitives)?;
    }
    Ok(primitives)
}

fn sv_position(scale: &SVScale<'_>, value: f64) -> Result<f64, SVError> {
    scale.position(SVScaleValue::Numeric(value))
}

fn sv_owned_primitives(
    geometry: &crate::sv_mark::SVMarkGeometry<'_>,
    primitives: &mut Vec<SVMarkPrimitive<'static>>,
) -> Result<(), SVError> {
    for primitive in geometry.primitives() {
        let paint: SVMarkPaint = primitive.paint().clone();
        let shape = match primitive.shape() {
            SVMarkShape::Path(path) => SVMarkShape::Path(path.clone()),
            SVMarkShape::Rectangle(rectangle) => SVMarkShape::Rectangle(*rectangle),
            SVMarkShape::Circle(circle) => SVMarkShape::Circle(*circle),
            SVMarkShape::Text(text) => SVMarkShape::Text(text.clone()),
            SVMarkShape::Image(_) => SVMarkShape::Image(SVGeometryImage::new(
                SVResolvedRectangle::new(0.0, 0.0, 0.0, 0.0),
                0,
                0,
                &[],
                SVImageInterpolation::Nearest,
            )),
        };
        primitives.push(SVMarkPrimitive::new(shape, paint));
    }
    Ok(())
}
