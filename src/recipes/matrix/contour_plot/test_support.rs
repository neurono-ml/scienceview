//! Shared mock data and scene helpers for the contour tests.
//!
//! Mock columns are borrowed slices, sparse views and synthetic streaming
//! snapshots; nothing here touches the network or the filesystem.

use super::SVContourComposition;
use super::{SVContourPlot, SVContourPlotBuilder};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkPrimitive;

/// A five-by-five radial field, row-major.
pub(crate) const FIELD: [f64; 25] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0, 2.0, 4.0, 2.0, 0.0, 0.0, 1.0, 2.0, 1.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
];

/// The horizontal coordinates of a nine-point cloud, in field order.
pub(crate) const POINT_X: [f64; 9] = [0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0];
/// The vertical coordinates of the nine-point cloud.
pub(crate) const POINT_Y: [f64; 9] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
/// A three-by-three cloud: a hill in the middle.
pub(crate) const POINT_VALUE: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVTestSource<'a> {
    pub(crate) value: SVColumnView<'a>,
    pub(crate) x: Option<SVColumnView<'a>>,
    pub(crate) y: Option<SVColumnView<'a>>,
}

impl<'a> SVTestSource<'a> {
    pub(crate) fn matrix(values: &'a [f64]) -> SVTestSource<'a> {
        SVTestSource {
            value: SVColumnView::Dense(values),
            x: None,
            y: None,
        }
    }

    pub(crate) fn points(x: &'a [f64], y: &'a [f64], value: &'a [f64]) -> SVTestSource<'a> {
        SVTestSource {
            value: SVColumnView::Dense(value),
            x: Some(SVColumnView::Dense(x)),
            y: Some(SVColumnView::Dense(y)),
        }
    }
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the contour recipe resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "value" => self.value,
            "x" => self.x.ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?,
            "y" => self.y.ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?,
            _ => {
                return Err(SVError::ColumnNotFound {
                    name: name.to_string(),
                });
            }
        };
        Ok(SVColumnSnapshot::Borrowed(view))
    }
}

pub(crate) fn sv_matrix_source() -> SVTestSource<'static> {
    SVTestSource::matrix(&FIELD)
}

pub(crate) fn sv_points_source() -> SVTestSource<'static> {
    SVTestSource::points(&POINT_X, &POINT_Y, &POINT_VALUE)
}

pub(crate) fn sv_sparse_points_source() -> SVTestSource<'static> {
    const INDICES: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let x = SVSparseView::new(&INDICES, &POINT_X, 9).expect("the sparse view is well formed");
    let y = SVSparseView::new(&INDICES, &POINT_Y, 9).expect("the sparse view is well formed");
    let value =
        SVSparseView::new(&INDICES, &POINT_VALUE, 9).expect("the sparse view is well formed");
    SVTestSource {
        value: SVColumnView::Sparse(value),
        x: Some(SVColumnView::Sparse(x)),
        y: Some(SVColumnView::Sparse(y)),
    }
}

pub(crate) fn sv_streaming_points_source() -> SVTestSource<'static> {
    SVTestSource {
        value: SVColumnView::Streaming(SVStreamingView::new(&POINT_VALUE, 9, 0)),
        x: Some(SVColumnView::Streaming(SVStreamingView::new(
            &POINT_X, 9, 0,
        ))),
        y: Some(SVColumnView::Streaming(SVStreamingView::new(
            &POINT_Y, 9, 0,
        ))),
    }
}

pub(crate) fn sv_builder(source: &dyn SVDataSource) -> SVContourPlotBuilder<'_> {
    SVContourPlot::builder()
        .data_source(source)
        .value("value")
        .matrix(5, 5)
        .levels(3)
        .backend(SVBackendKind::Svg)
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVContourPlot<'_> {
    sv_builder(source)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_compose_default(source: &dyn SVDataSource) -> SVContourComposition {
    sv_build_default(source)
        .compose()
        .expect("the composition succeeds")
}

pub(crate) fn sv_points_builder(source: &dyn SVDataSource) -> SVContourPlotBuilder<'_> {
    SVContourPlot::builder()
        .data_source(source)
        .value("value")
        .x("x")
        .y("y")
        .grid_count(3)
        .levels(2)
        .backend(SVBackendKind::Svg)
}

pub(crate) fn sv_primitives(composition: &SVContourComposition) -> Vec<SVMarkPrimitive<'static>> {
    let mut primitives = Vec::new();
    composition
        .visit_mark_geometries(|geometry| primitives.extend(geometry.primitives().iter().cloned()))
        .expect("the contour geometry builds");
    primitives
}

pub(crate) fn sv_scene_to_svg(composition: &SVContourComposition) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(composition.viewport())
        .with_title(composition.title().unwrap_or("contour plot"))
        .with_description("Contour plot generated by scienceview")
        .with_background(composition.background());
    document.push_decoration_primitives(composition.decoration_primitives_before_marks());
    composition
        .visit_mark_geometries(|geometry| document.push_mark_geometry(geometry))
        .expect("the contour geometry builds");
    document.push_decoration_primitives(composition.decoration_primitives_after_marks());
    document.to_svg()
}
