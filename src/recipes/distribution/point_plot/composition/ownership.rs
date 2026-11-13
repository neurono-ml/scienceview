//! Owning a mark geometry so the render output outlives its scale borrows.

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVMarkGeometry, SVMarkPrimitive, SVMarkShape};

pub(super) fn sv_own_geometry(
    geometry: SVMarkGeometry<'_>,
) -> Result<SVMarkGeometry<'static>, SVError> {
    let primitives = geometry
        .primitives()
        .iter()
        .map(|primitive| {
            Ok(SVMarkPrimitive::new(
                sv_own_shape(primitive.shape())?,
                primitive.paint().clone(),
            ))
        })
        .collect::<Result<Vec<_>, SVError>>()?;
    Ok(SVMarkGeometry::new(primitives))
}

fn sv_own_shape(shape: &SVMarkShape<'_>) -> Result<SVMarkShape<'static>, SVError> {
    match shape {
        SVMarkShape::Path(path) => Ok(SVMarkShape::Path(path.clone())),
        SVMarkShape::Rectangle(rectangle) => Ok(SVMarkShape::Rectangle(*rectangle)),
        SVMarkShape::Circle(circle) => Ok(SVMarkShape::Circle(*circle)),
        SVMarkShape::Text(text) => Ok(SVMarkShape::Text(text.clone())),
        SVMarkShape::Image(_) => Err(super::super::configuration::sv_invalid(
            "a point plot draws dots, paths and labels only; an image primitive is not part of its composition",
        )),
    }
}
