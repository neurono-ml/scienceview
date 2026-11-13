//! Shared mock data and scene helpers for the clustered heatmap tests.

use super::{
    SVClusterMerge, SVClusterTree, SVClusteredHeatmapPlot, SVClusteredHeatmapScene, SVHeatmapMatrix,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVSparseView};

pub(crate) const ROWS: [f64; 9] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
pub(crate) const COLUMNS: [f64; 9] = [0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0];
pub(crate) const VALUES: [f64; 9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
pub(crate) const ROW_ORDER: [usize; 3] = [2, 0, 1];
pub(crate) const COLUMN_ORDER: [usize; 3] = [1, 2, 0];

pub(crate) const ROW_MERGES: [SVClusterMerge; 2] = [
    SVClusterMerge::new(0, 1, 1.0, 2),
    SVClusterMerge::new(3, 2, 2.0, 3),
];
pub(crate) const COLUMN_MERGES: [SVClusterMerge; 2] = [
    SVClusterMerge::new(0, 2, 0.5, 2),
    SVClusterMerge::new(3, 1, 1.5, 3),
];

pub(crate) fn sv_row_tree() -> SVClusterTree<'static> {
    SVClusterTree::new(&ROW_MERGES)
}

pub(crate) fn sv_column_tree() -> SVClusterTree<'static> {
    SVClusterTree::new(&COLUMN_MERGES)
}

/// A synthetic long table serving every representation through one seam.
pub(crate) struct SVTestSource<'a> {
    pub(crate) rows: SVColumnView<'a>,
    pub(crate) columns: SVColumnView<'a>,
    pub(crate) values: SVColumnView<'a>,
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the clustered heatmap resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, crate::sv_error::SVError> {
        let view = match name {
            "row" => self.rows,
            "column" => self.columns,
            "value" => self.values,
            _ => {
                return Err(crate::sv_error::SVError::ColumnNotFound {
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

pub(crate) fn sv_streaming_source() -> SVTestSource<'static> {
    use crate::sv_data::SVStreamingView;
    SVTestSource {
        rows: SVColumnView::Streaming(SVStreamingView::new(&ROWS, 9, 0)),
        columns: SVColumnView::Streaming(SVStreamingView::new(&COLUMNS, 9, 0)),
        values: SVColumnView::Streaming(SVStreamingView::new(&VALUES, 9, 0)),
    }
}

pub(crate) fn sv_sparse_source() -> SVTestSource<'static> {
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

pub(crate) fn sv_build_default<'a>(source: &'a SVTestSource<'a>) -> SVClusteredHeatmapPlot<'a> {
    SVClusteredHeatmapPlot::builder()
        .data_source(source)
        .row("row")
        .column("column")
        .value("value")
        .row_order(&ROW_ORDER)
        .column_order(&COLUMN_ORDER)
        .row_linkage(sv_row_tree())
        .column_linkage(sv_column_tree())
        .row_labels(&["row 0", "row 1", "row 2"])
        .column_labels(&["column 0", "column 1", "column 2"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_matrix() -> SVHeatmapMatrix<'static> {
    SVHeatmapMatrix::new(&VALUES, 3, 3).expect("the matrix shape matches")
}

pub(crate) fn sv_build_matrix() -> SVClusteredHeatmapPlot<'static> {
    SVClusteredHeatmapPlot::builder()
        .matrix(sv_matrix())
        .row_order(&ROW_ORDER)
        .column_order(&COLUMN_ORDER)
        .row_linkage(sv_row_tree())
        .column_linkage(sv_column_tree())
        .row_labels(&["row 0", "row 1", "row 2"])
        .column_labels(&["column 0", "column 1", "column 2"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the matrix configuration is complete")
}

pub(crate) fn sv_scene_to_svg(scene: &SVClusteredHeatmapScene) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("clustered heatmap"))
        .with_description(scene.description().unwrap_or("clustered heatmap"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}
