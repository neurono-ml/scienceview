//! The dot geometry of one swarm render.
//!
//! The dot kernel owns placement, radius and shape; the recipe maps the series
//! colour and the constant opacity onto each primitive without re-deriving a
//! position.

use super::channels::SVSwarmObservation;
use super::groups::SVSwarmSeriesGroup;
use super::scales::SVSwarmScales;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_mark::{
    SVBarOrientation, SVDotShape, SVGeometryCircle, SVGeometryPath, SVGeometryRectangle, SVMark,
    SVMarkGeometry, SVMarkPaint, SVMarkPrimitive, SVMarkRequest, SVMarkShape, SVMarkTheme,
};
use crate::sv_scale::SVColour;

/// Everything one dot-geometry pass consumes.
pub(super) struct SVSwarmMarkInput<'a, 'labels> {
    pub(super) observations: &'a [SVSwarmObservation],
    pub(super) category_positions: &'a [f64],
    pub(super) series: &'a [SVSwarmSeriesGroup],
    pub(super) scales: &'a SVSwarmScales<'labels>,
    pub(super) orientation: SVBarOrientation,
    pub(super) marker: SVDotShape,
    pub(super) alpha: f32,
    pub(super) plot: SVResolvedRectangle,
}

/// Builds the mark geometry in observation order.
pub(super) fn sv_compose_mark_geometry(
    input: &SVSwarmMarkInput<'_, '_>,
) -> Result<SVMarkGeometry<'static>, SVError> {
    let (primary, secondary) = sv_channels(input);
    let mark: SVMark = SVMark::dot()
        .shape(input.marker)
        .size(SVLength::absolute(input.scales.marker_diameter))
        .opacity(input.alpha)
        .into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&primary),
        SVColumnView::Dense(&secondary),
        &input.scales.category,
        &input.scales.value,
        input.plot,
    );
    let geometry = mark.geometry(&request)?;
    let theme = SVMarkTheme::default();
    let cross = input.marker == SVDotShape::Cross;

    let mut primitives = Vec::with_capacity(input.observations.len());
    for (index, primitive) in geometry.into_primitives().into_iter().enumerate() {
        let shape =
            sv_own_shape(primitive.shape()).ok_or_else(|| SVError::InvalidConfiguration {
                message: "a swarm dot produced a raster primitive".to_string(),
            })?;
        let colour = sv_observation_colour(input, index);
        let paint = sv_swarm_paint(colour, input.alpha, cross, theme.line_width());
        primitives.push(SVMarkPrimitive::new(shape, paint));
    }

    Ok(SVMarkGeometry::new(primitives))
}

/// Rebuilds a mark shape with an owned lifetime.
///
/// A dot never produces a raster primitive, so the image variant is skipped
/// instead of being copied.
fn sv_own_shape(shape: &SVMarkShape<'_>) -> Option<SVMarkShape<'static>> {
    Some(match shape {
        SVMarkShape::Circle(circle) => {
            SVMarkShape::Circle(SVGeometryCircle::new(circle.centre(), circle.radius()))
        }
        SVMarkShape::Rectangle(rectangle) => SVMarkShape::Rectangle(SVGeometryRectangle::new(
            rectangle.rectangle(),
            rectangle.corner_radius(),
        )),
        SVMarkShape::Path(path) => SVMarkShape::Path(SVGeometryPath::new(path.commands().to_vec())),
        SVMarkShape::Text(text) => SVMarkShape::Text(text.clone()),
        SVMarkShape::Image(_) => return None,
    })
}

fn sv_channels(input: &SVSwarmMarkInput<'_, '_>) -> (Vec<f64>, Vec<f64>) {
    let mut primary = Vec::with_capacity(input.observations.len());
    let mut secondary = Vec::with_capacity(input.observations.len());
    for (observation, category_position) in input.observations.iter().zip(input.category_positions)
    {
        match input.orientation {
            SVBarOrientation::Vertical => {
                primary.push(*category_position);
                secondary.push(observation.value);
            }
            SVBarOrientation::Horizontal => {
                primary.push(observation.value);
                secondary.push(*category_position);
            }
        }
    }
    (primary, secondary)
}

fn sv_observation_colour(input: &SVSwarmMarkInput<'_, '_>, index: usize) -> SVColour {
    input
        .observations
        .get(index)
        .and_then(|observation| input.series.get(observation.series_index))
        .map_or_else(
            || SVMarkTheme::default().mark_colour(),
            |group| group.colour,
        )
}

fn sv_swarm_paint(colour: SVColour, alpha: f32, cross: bool, line_width: f64) -> SVMarkPaint {
    let paint = if cross {
        SVMarkPaint::stroked(colour, line_width)
    } else {
        SVMarkPaint::filled(colour)
    };
    paint.with_opacity(alpha)
}
