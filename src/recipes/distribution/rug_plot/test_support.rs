//! Shared mock data and scene helpers for the rug-plot tests.
//!
//! Mock columns are borrowed slices, sparse views and synthetic streaming
//! snapshots; nothing here touches the network or the filesystem.

use super::SVRugPlot;
use super::SVRugScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_error::SVError;
use crate::sv_mark::{SVMarkPoint, SVMarkShape, SVPathCommand};

pub(crate) const VALUES: [f64; 6] = [1.0, 2.0, 2.5, 3.5, 4.0, 5.0];
pub(crate) const SERIES: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVTestRugSource<'a> {
    pub values: SVColumnView<'a>,
    pub series: Option<SVColumnView<'a>>,
}

impl<'a> SVTestRugSource<'a> {
    pub(crate) fn dense(values: &'a [f64]) -> Self {
        Self {
            values: SVColumnView::Dense(values),
            series: None,
        }
    }
}

impl SVDataSource for SVTestRugSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the rug recipe resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "value" => self.values,
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

pub(crate) fn sv_dense_source() -> SVTestRugSource<'static> {
    SVTestRugSource::dense(&VALUES)
}

pub(crate) fn sv_series_source() -> SVTestRugSource<'static> {
    SVTestRugSource {
        values: SVColumnView::Dense(&VALUES),
        series: Some(SVColumnView::Dense(&SERIES)),
    }
}

pub(crate) fn sv_sparse_source() -> SVTestRugSource<'static> {
    const INDICES: [usize; 6] = [0, 1, 2, 3, 4, 5];
    let values = SVSparseView::new(&INDICES, &VALUES, 6).expect("the sparse view is well formed");
    SVTestRugSource {
        values: SVColumnView::Sparse(values),
        series: None,
    }
}

pub(crate) fn sv_streaming_source() -> SVTestRugSource<'static> {
    SVTestRugSource {
        values: SVColumnView::Streaming(SVStreamingView::new(&VALUES, 6, 0)),
        series: None,
    }
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVRugPlot<'_> {
    SVRugPlot::builder()
        .data_source(source)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_build_with_source(
    source: &dyn SVDataSource,
    backend_kind: SVBackendKind,
) -> SVRugPlot<'_> {
    SVRugPlot::builder()
        .data_source(source)
        .value("value")
        .backend(backend_kind)
        .build()
        .expect("the configuration is complete")
}

/// Every tick drawn by the scene, as `(start, end)` display-space segments.
pub(crate) fn sv_tick_segments(scene: &SVRugScene) -> Vec<(SVMarkPoint, SVMarkPoint)> {
    let mut segments = Vec::new();
    for geometry in scene.mark_geometries() {
        for primitive in geometry.primitives() {
            let SVMarkShape::Path(path) = primitive.shape() else {
                continue;
            };
            let mut start: Option<SVMarkPoint> = None;
            for command in path.commands() {
                match command {
                    SVPathCommand::MoveTo(point) => start = Some(*point),
                    SVPathCommand::LineTo(point) => {
                        if let Some(start) = start {
                            segments.push((start, *point));
                        }
                    }
                    SVPathCommand::ArcTo(_) | SVPathCommand::Close => start = None,
                }
            }
        }
    }
    segments
}

pub(crate) fn sv_scene_to_svg(scene: &SVRugScene) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("rug plot"))
        .with_description(scene.description().unwrap_or("rug plot"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}

pub(crate) fn sv_scene_text(scene: &SVRugScene) -> String {
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
