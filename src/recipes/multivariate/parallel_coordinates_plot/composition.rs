//! Composing one line mark per polyline through the mark kernel.

use super::configuration::SVParallelCoordinatesConfiguration;
use super::preparation::SVParallelCoordinatesPrepared;
use super::preparation::axes::sv_identity_scale;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_mark::{SVMark, SVMarkGeometry, SVMarkPrimitive, SVMarkRequest, SVMarkShape};

pub(super) fn sv_compose_geometries(
    configuration: &SVParallelCoordinatesConfiguration<'_>,
    prepared: &SVParallelCoordinatesPrepared,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let horizontal = sv_identity_scale(0.0, prepared.viewport.width())?;
    let vertical = sv_identity_scale(0.0, prepared.viewport.height())?;
    let mut geometries = Vec::with_capacity(prepared.lines.len());

    for line in &prepared.lines {
        if line.vertices().len() < 2 {
            continue;
        }
        let x: Vec<f64> = line.vertices().iter().map(|point| point.x()).collect();
        let y: Vec<f64> = line.vertices().iter().map(|point| point.y()).collect();
        let mark: SVMark = SVMark::line()
            .width(configuration.line_width)
            .opacity(configuration.opacity)
            .colour(line.colour())
            .into();
        let request = SVMarkRequest::new(
            SVColumnView::Dense(&x),
            SVColumnView::Dense(&y),
            &horizontal,
            &vertical,
            prepared.plot_area,
        );
        let geometry = mark.geometry(&request)?;
        geometries.push(sv_own_geometry(geometry)?);
    }
    Ok(geometries)
}

fn sv_own_geometry(geometry: SVMarkGeometry<'_>) -> Result<SVMarkGeometry<'static>, SVError> {
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
        SVMarkShape::Image(_) => Err(SVError::InvalidConfiguration {
            message: "a parallel-coordinates plot draws line paths only; an image primitive is not part of its composition".to_string(),
        }),
    }
}
