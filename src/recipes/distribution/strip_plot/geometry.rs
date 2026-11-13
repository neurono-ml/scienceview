//! Dot geometry: one dot per drawn observation, coloured by its series.
//!
//! The dot kernel owns placement and the shape vocabulary. The recipe only
//! supplies the two position columns — the jittered category slot and the
//! observed value — and repaints every primitive with its series colour.

use super::configuration::SVStripConfiguration;
use super::data_contract::{SVStripObservation, SVStripSeriesGroup};
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{SVMark, SVMarkGeometry, SVMarkPaint, SVMarkPrimitive, SVMarkRequest};
use crate::sv_scale::{SVColour, SVScale};

/// Everything one dot-geometry pass consumes.
pub(super) struct SVStripGeometryInput<'a, 'scale> {
    pub(super) observations: &'a [SVStripObservation],
    pub(super) groups: &'a [SVStripSeriesGroup],
    pub(super) displaced_categories: &'a [f64],
    pub(super) value_scale: &'a SVScale<'scale>,
    pub(super) category_scale: &'a SVScale<'scale>,
    pub(super) plot: SVResolvedRectangle,
    pub(super) configuration: &'a SVStripConfiguration<'a>,
}

/// Composes the dot geometry in draw order.
pub(super) fn sv_dot_geometries(
    input: &SVStripGeometryInput<'_, '_>,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let values: Vec<f64> = input
        .observations
        .iter()
        .map(|observation| observation.value)
        .collect();
    let mut dot = SVMark::dot()
        .shape(input.configuration.marker)
        .opacity(input.configuration.alpha);
    if let Some(size) = input.configuration.size {
        dot = dot.size(size);
    }
    let mark: SVMark = dot.into();

    let rotated = input.configuration.orientation == crate::sv_mark::SVBarOrientation::Horizontal;
    let horizontal: &[f64] = if rotated {
        &values
    } else {
        input.displaced_categories
    };
    let vertical: &[f64] = if rotated {
        input.displaced_categories
    } else {
        &values
    };
    let (horizontal_scale, vertical_scale) = if rotated {
        (input.value_scale, input.category_scale)
    } else {
        (input.category_scale, input.value_scale)
    };
    let request = SVMarkRequest::new(
        crate::sv_data::SVColumnView::Dense(horizontal),
        crate::sv_data::SVColumnView::Dense(vertical),
        horizontal_scale,
        vertical_scale,
        input.plot,
    );
    let geometry = mark.geometry(&request)?;

    let primitives = geometry
        .into_primitives()
        .into_iter()
        .enumerate()
        .filter_map(|(index, primitive)| {
            let shape = sv_owned_shape(primitive.shape())?;
            let colour = sv_colour(input.groups, input.observations.get(index));
            Some(SVMarkPrimitive::new(
                shape,
                sv_tint(primitive.paint(), colour),
            ))
        })
        .collect();
    Ok(vec![SVMarkGeometry::new(primitives)])
}

/// Copies one drawn shape into an owned value so the scene outlives the request.
fn sv_owned_shape(
    shape: &crate::sv_mark::SVMarkShape<'_>,
) -> Option<crate::sv_mark::SVMarkShape<'static>> {
    use crate::sv_mark::SVMarkShape;
    Some(match shape {
        SVMarkShape::Circle(circle) => SVMarkShape::Circle(*circle),
        SVMarkShape::Rectangle(rectangle) => SVMarkShape::Rectangle(*rectangle),
        SVMarkShape::Path(path) => SVMarkShape::Path(path.clone()),
        SVMarkShape::Text(text) => SVMarkShape::Text(text.clone()),
        SVMarkShape::Image(_) => return None,
    })
}

fn sv_colour(groups: &[SVStripSeriesGroup], observation: Option<&SVStripObservation>) -> SVColour {
    let fallback = SVColour::new(0, 0, 0, 255);
    let Some(observation) = observation else {
        return fallback;
    };
    groups
        .get(observation.group_index)
        .map_or(fallback, |group| group.colour)
}

/// Repaints one dot with its series colour, preserving opacity and stroke.
fn sv_tint(paint: &SVMarkPaint, colour: SVColour) -> SVMarkPaint {
    let mut tinted = SVMarkPaint::new()
        .with_opacity(paint.opacity())
        .with_dash(paint.dash().clone())
        .with_join(paint.join())
        .with_cap(paint.cap());
    if paint.fill().is_some() {
        tinted = tinted.with_fill(colour);
    }
    if paint.stroke().is_some() {
        tinted = tinted.with_stroke(colour, paint.stroke_width());
    }
    tinted
}
