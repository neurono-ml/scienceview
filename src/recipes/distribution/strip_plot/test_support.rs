//! Shared mock data and scene helpers for the strip-plot tests.
//!
//! Mock columns are borrowed slices, sparse views and synthetic streaming
//! snapshots; nothing here touches the network or the filesystem.

use super::SVStripPlot;
use super::SVStripScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
    SVSparseView, SVStreamingView,
};
use crate::sv_error::SVError;
use crate::sv_mark::{SVMarkPoint, SVMarkShape};

pub(crate) const VALUES: [f64; 9] = [1.0, 2.0, 2.5, 3.5, 4.0, 5.0, 5.5, 6.0, 6.5];
pub(crate) const CATEGORIES: [f64; 9] = [0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0];
pub(crate) const SERIES: [f64; 9] = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVTestStripSource<'a> {
    pub(crate) values: SVColumnView<'a>,
    pub(crate) categories: SVColumnView<'a>,
    pub(crate) series: Option<SVColumnView<'a>>,
}

impl<'a> SVTestStripSource<'a> {
    pub(crate) fn dense(values: &'a [f64], categories: &'a [f64]) -> Self {
        Self {
            values: SVColumnView::Dense(values),
            categories: SVColumnView::Dense(categories),
            series: None,
        }
    }
}

impl SVDataSource for SVTestStripSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(vec![
            SVColumnDescriptor::new("value", SVColumnRole::Y),
            SVColumnDescriptor::new("category", SVColumnRole::Grouping),
            SVColumnDescriptor::new("series", SVColumnRole::Grouping),
        ])
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "value" => self.values,
            "category" => self.categories,
            "series" => self.series.ok_or_else(|| SVError::ColumnNotFound {
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

pub(crate) fn sv_dense_source() -> SVTestStripSource<'static> {
    SVTestStripSource::dense(&VALUES, &CATEGORIES)
}

pub(crate) fn sv_series_source() -> SVTestStripSource<'static> {
    SVTestStripSource {
        values: SVColumnView::Dense(&VALUES),
        categories: SVColumnView::Dense(&CATEGORIES),
        series: Some(SVColumnView::Dense(&SERIES)),
    }
}

pub(crate) fn sv_sparse_source() -> SVTestStripSource<'static> {
    const INDICES: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let values = SVSparseView::new(&INDICES, &VALUES, 9).expect("the sparse view is well formed");
    let categories =
        SVSparseView::new(&INDICES, &CATEGORIES, 9).expect("the sparse view is well formed");
    SVTestStripSource {
        values: SVColumnView::Sparse(values),
        categories: SVColumnView::Sparse(categories),
        series: None,
    }
}

pub(crate) fn sv_streaming_source() -> SVTestStripSource<'static> {
    SVTestStripSource {
        values: SVColumnView::Streaming(SVStreamingView::new(&VALUES, 9, 0)),
        categories: SVColumnView::Streaming(SVStreamingView::new(&CATEGORIES, 9, 0)),
        series: None,
    }
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVStripPlot<'_> {
    SVStripPlot::builder()
        .data_source(source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

/// Every dot centre drawn by the scene, in display space.
pub(crate) fn sv_dot_centres(scene: &SVStripScene) -> Vec<SVMarkPoint> {
    let mut centres = Vec::new();
    for geometry in scene.mark_geometries() {
        for primitive in geometry.primitives() {
            match primitive.shape() {
                SVMarkShape::Circle(circle) => centres.push(circle.centre()),
                SVMarkShape::Rectangle(rectangle) => {
                    let bounds = rectangle.rectangle();
                    centres.push(SVMarkPoint::new(
                        bounds.x + bounds.width / 2.0,
                        bounds.y + bounds.height / 2.0,
                    ));
                }
                SVMarkShape::Path(path) => {
                    if let Some(first) = path.commands().first() {
                        if let Some(point) = sv_command_point(first) {
                            centres.push(point);
                        }
                    }
                }
                SVMarkShape::Text(_) | SVMarkShape::Image(_) => {}
            }
        }
    }
    centres
}

fn sv_command_point(command: &crate::sv_mark::SVPathCommand) -> Option<SVMarkPoint> {
    match command {
        crate::sv_mark::SVPathCommand::MoveTo(point)
        | crate::sv_mark::SVPathCommand::LineTo(point) => Some(*point),
        crate::sv_mark::SVPathCommand::ArcTo(arc) => Some(arc.centre()),
        crate::sv_mark::SVPathCommand::Close => None,
    }
}

pub(crate) fn sv_scene_to_svg(scene: &SVStripScene) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("strip plot"))
        .with_description(scene.description().unwrap_or("strip plot"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}

pub(crate) fn sv_scene_text(scene: &SVStripScene) -> String {
    scene
        .decoration_primitives_before_marks()
        .iter()
        .chain(scene.decoration_primitives_after_marks())
        .filter_map(|primitive| match primitive {
            crate::sv_decoration::SVDecorationPrimitive::Text { text, .. } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}
