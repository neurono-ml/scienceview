//! Tick geometry: one short line segment per observation.
//!
//! Every tick is composed by a two-point [`SVMark::Line`] operation, so the
//! line kernel owns the stroke, width and opacity while the recipe chooses the
//! two endpoints. A large sample groups the tick paths into one compound path
//! per paint, which keeps the document bounded.

use super::channels::{SVRugObservation, SVRugSeriesGroup};
use crate::sv_error::SVError;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_mark::{
    SVBarOrientation, SVGeometryPath, SVMark, SVMarkGeometry, SVMarkPaint, SVMarkPrimitive,
    SVMarkRequest, SVMarkShape, SVPathCommand,
};
use crate::sv_scale::SVScale;

/// Above this many ticks the rug draws one compound path per paint.
pub(super) const SV_LARGE_RUG_THRESHOLD: usize = 512;

/// The two endpoints of one tick, in data space before the scales run.
struct SVRugTickPair {
    primary: [f64; 2],
    secondary: [f64; 2],
}

/// Everything one tick-geometry pass consumes.
pub(super) struct SVRugGeometryInput<'a, 'scale> {
    pub(super) observations: &'a [SVRugObservation],
    pub(super) groups: &'a [SVRugSeriesGroup],
    pub(super) value_scale: &'a SVScale<'scale>,
    pub(super) depth_scale: &'a SVScale<'static>,
    pub(super) orientation: SVBarOrientation,
    pub(super) plot: SVResolvedRectangle,
    pub(super) alpha: f32,
    pub(super) line_width: Option<SVLength>,
}

/// Composes the tick geometry in draw order.
pub(super) fn sv_tick_geometries(
    input: &SVRugGeometryInput<'_, '_>,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let mut primitives: Vec<SVMarkPrimitive<'static>> = Vec::new();

    for observation in input.observations {
        let colour = input.groups.get(observation.group_index).map_or_else(
            || crate::sv_scale::SVColour::new(0, 0, 0, 255),
            |group| group.colour,
        );
        let mut options = SVMark::line().opacity(input.alpha).colour(colour);
        if let Some(width) = input.line_width {
            options = options.width(width);
        }
        let mark: SVMark = options.into();
        let pair = sv_tick_pair(observation.value, input.orientation);
        let (horizontal_scale, vertical_scale) = match input.orientation {
            SVBarOrientation::Vertical => (input.value_scale, input.depth_scale),
            SVBarOrientation::Horizontal => (input.depth_scale, input.value_scale),
        };
        let request = SVMarkRequest::new(
            crate::sv_data::SVColumnView::Dense(&pair.primary),
            crate::sv_data::SVColumnView::Dense(&pair.secondary),
            horizontal_scale,
            vertical_scale,
            input.plot,
        );
        let geometry = mark.geometry(&request)?;
        for primitive in geometry.into_primitives() {
            let SVMarkShape::Path(path) = primitive.shape() else {
                return Err(sv_unreachable_tick_shape());
            };
            let owned: SVMarkShape<'static> = SVMarkShape::Path(path.clone());
            primitives.push(SVMarkPrimitive::new(owned, primitive.paint().clone()));
        }
    }

    if primitives.len() >= SV_LARGE_RUG_THRESHOLD {
        primitives = sv_group_large_rug(primitives);
    }
    Ok(vec![SVMarkGeometry::new(primitives)])
}

/// Places the two endpoints on the value axis and the depth axis.
fn sv_tick_pair(value: f64, orientation: SVBarOrientation) -> SVRugTickPair {
    match orientation {
        SVBarOrientation::Vertical => SVRugTickPair {
            primary: [value, value],
            secondary: [0.0, 1.0],
        },
        SVBarOrientation::Horizontal => SVRugTickPair {
            primary: [0.0, 1.0],
            secondary: [value, value],
        },
    }
}

/// Groups a large sample into one compound path per paint.
///
/// The document stays small without changing what is drawn: each tick becomes
/// a move/line subpath in a path that shares one paint with its peers.
fn sv_group_large_rug(primitives: Vec<SVMarkPrimitive<'static>>) -> Vec<SVMarkPrimitive<'static>> {
    let mut groups: Vec<(SVMarkPaint, Vec<SVPathCommand>)> = Vec::new();
    for primitive in &primitives {
        let SVMarkShape::Path(path) = primitive.shape() else {
            return primitives;
        };
        let paint = primitive.paint().clone();
        let index = match groups.iter().position(|(candidate, _)| *candidate == paint) {
            Some(index) => index,
            None => {
                groups.push((paint, Vec::new()));
                groups.len() - 1
            }
        };
        if let Some((_, commands)) = groups.get_mut(index) {
            commands.extend_from_slice(path.commands());
        }
    }

    groups
        .into_iter()
        .map(|(paint, commands)| {
            SVMarkPrimitive::new(SVMarkShape::Path(SVGeometryPath::new(commands)), paint)
        })
        .collect()
}

fn sv_unreachable_tick_shape() -> SVError {
    SVError::InvalidConfiguration {
        message: "a rug tick produced a primitive that is not a path".to_string(),
    }
}
