//! Applying the recipe's paint to the dot kernel's geometry.
//!
//! The dot kernel owns placement and the default shape. The recipe is what maps
//! the optional colour grouping onto those primitives, so this module recolours
//! and re-opacities primitives without re-deriving a position. A primitive is
//! copied into an owned shape so the scene outlives the resolved column
//! snapshots; the raster variant is unreachable for a dot and is skipped
//! explicitly instead of being invented.

use crate::sv_error::SVError;
use crate::sv_mark::{SVMarkGeometry, SVMarkPaint, SVMarkPrimitive, SVMarkShape};
use crate::sv_scale::SVColour;

/// Copies one mark shape into an owned, `'static` shape.
pub(super) fn sv_owned_shape(shape: &SVMarkShape<'_>) -> Option<SVMarkShape<'static>> {
    Some(match shape {
        SVMarkShape::Path(path) => SVMarkShape::Path(path.clone()),
        SVMarkShape::Rectangle(rectangle) => SVMarkShape::Rectangle(*rectangle),
        SVMarkShape::Circle(circle) => SVMarkShape::Circle(*circle),
        SVMarkShape::Text(text) => SVMarkShape::Text(text.clone()),
        SVMarkShape::Image(_) => return None,
    })
}

/// Recolours and re-opacities every point of one cell.
///
/// The dot kernel emits one primitive per observation, in column order, so a
/// primitive index addresses the same observation the colour column resolved.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when a dot produced a raster
/// primitive, which a dot never does.
pub(super) fn sv_paint_points(
    geometry: SVMarkGeometry<'_>,
    point_colours: &[SVColour],
    default_colour: SVColour,
    opacity: f32,
) -> Result<Vec<SVMarkPrimitive<'static>>, SVError> {
    let primitives = geometry.into_primitives();
    let mut painted = Vec::with_capacity(primitives.len());
    for (index, primitive) in primitives.iter().enumerate() {
        let shape = sv_owned_shape(primitive.shape()).ok_or_else(sv_unreachable_shape)?;
        let colour = point_colours.get(index).copied().unwrap_or(default_colour);
        painted.push(SVMarkPrimitive::new(
            shape,
            SVMarkPaint::filled(colour).with_opacity(opacity),
        ));
    }
    Ok(painted)
}

/// Copies a primitive that already carries its paint into an owned shape.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when the primitive is a raster,
/// which neither the bar nor the area kernel produces.
pub(super) fn sv_owned_primitive(
    primitive: &SVMarkPrimitive<'_>,
) -> Result<SVMarkPrimitive<'static>, SVError> {
    let shape = sv_owned_shape(primitive.shape()).ok_or_else(sv_unreachable_shape)?;
    Ok(SVMarkPrimitive::new(shape, primitive.paint().clone()))
}

fn sv_unreachable_shape() -> SVError {
    SVError::InvalidConfiguration {
        message: "a scatterplot-matrix mark produced a raster primitive".to_string(),
    }
}
