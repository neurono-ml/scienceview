//! Shared mock data and scene helpers for the heatmap tests.

use super::SVHeatmapScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_error::SVError;

pub(crate) const ROWS: [f64; 9] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
pub(crate) const COLUMNS: [f64; 9] = [0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0];
pub(crate) const VALUES: [f64; 9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

/// A synthetic long table serving every representation through one seam.
pub(crate) struct SVTestSource<'a> {
    pub rows: SVColumnView<'a>,
    pub columns: SVColumnView<'a>,
    pub values: SVColumnView<'a>,
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the heatmap recipe resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "row" => self.rows,
            "column" => self.columns,
            "value" => self.values,
            _ => {
                return Err(SVError::ColumnNotFound {
                    name: name.to_string(),
                });
            }
        };
        Ok(SVColumnSnapshot::Borrowed(view))
    }
}

pub(crate) fn sv_dense_source() -> SVTestSource<'static> {
    SVTestSource {
        rows: SVColumnView::Dense(&ROWS),
        columns: SVColumnView::Dense(&COLUMNS),
        values: SVColumnView::Dense(&VALUES),
    }
}

pub(crate) fn sv_sparse_source() -> SVTestSource<'static> {
    const INDICES: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let sparse =
        SVSparseView::new(&INDICES, &VALUES, 9).expect("the sparse value view is well formed");
    SVTestSource {
        rows: SVColumnView::Dense(&ROWS),
        columns: SVColumnView::Dense(&COLUMNS),
        values: SVColumnView::Sparse(sparse),
    }
}

pub(crate) fn sv_streaming_source() -> SVTestSource<'static> {
    SVTestSource {
        rows: SVColumnView::Streaming(SVStreamingView::new(&ROWS, 9, 0)),
        columns: SVColumnView::Streaming(SVStreamingView::new(&COLUMNS, 9, 0)),
        values: SVColumnView::Streaming(SVStreamingView::new(&VALUES, 9, 0)),
    }
}

pub(crate) fn sv_partially_sparse_source() -> SVTestSource<'static> {
    const INDICES: [usize; 3] = [0, 4, 8];
    const STORED: [f64; 3] = [1.0, 5.0, 9.0];
    let sparse =
        SVSparseView::new(&INDICES, &STORED, 9).expect("the sparse value view is well formed");
    SVTestSource {
        rows: SVColumnView::Dense(&ROWS),
        columns: SVColumnView::Dense(&COLUMNS),
        values: SVColumnView::Sparse(sparse),
    }
}

pub(crate) fn sv_build_default<'a>(source: &'a SVTestSource<'a>) -> super::SVHeatmapPlot<'a> {
    super::SVHeatmapPlot::builder()
        .data_source(source)
        .row("row")
        .column("column")
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_scene_to_svg(scene: &SVHeatmapScene) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("heatmap"))
        .with_description(scene.description().unwrap_or("heatmap"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}

pub(crate) fn sv_scene_text(scene: &SVHeatmapScene) -> String {
    scene
        .decoration_primitives_before_marks()
        .iter()
        .chain(scene.decoration_primitives_after_marks())
        .filter_map(|primitive| match primitive {
            SVDecorationPrimitive::Text { text, .. } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}
