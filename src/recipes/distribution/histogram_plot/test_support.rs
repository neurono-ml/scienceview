//! Shared mock data and scene helpers for the histogram tests.
//!
//! Mock columns are borrowed slices, sparse views and synthetic streaming
//! snapshots; nothing here touches the network or the filesystem.

use super::SVHistogramPlot;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_error::SVError;

pub(crate) const VALUES: [f64; 8] = [1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 5.0];
pub(crate) const SERIES: [f64; 8] = [0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVTestSource<'a> {
    pub(crate) value: SVColumnView<'a>,
    pub(crate) series: Option<SVColumnView<'a>>,
}

impl SVTestSource<'_> {
    pub(crate) fn dense(value: &'static [f64]) -> SVTestSource<'static> {
        SVTestSource {
            value: SVColumnView::Dense(value),
            series: None,
        }
    }
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the histogram recipe resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "value" => self.value,
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

pub(crate) fn sv_dense_source() -> SVTestSource<'static> {
    SVTestSource::dense(&VALUES)
}

pub(crate) fn sv_series_source() -> SVTestSource<'static> {
    SVTestSource {
        value: SVColumnView::Dense(&VALUES),
        series: Some(SVColumnView::Dense(&SERIES)),
    }
}

pub(crate) fn sv_sparse_source() -> SVTestSource<'static> {
    const INDICES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let sparse =
        SVSparseView::new(&INDICES, &VALUES, VALUES.len()).expect("the sparse view is well formed");
    SVTestSource {
        value: SVColumnView::Sparse(sparse),
        series: None,
    }
}

pub(crate) fn sv_sparse_series_source() -> SVTestSource<'static> {
    const INDICES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let sparse =
        SVSparseView::new(&INDICES, &VALUES, VALUES.len()).expect("the sparse view is well formed");
    SVTestSource {
        value: SVColumnView::Sparse(sparse),
        series: Some(SVColumnView::Dense(&SERIES)),
    }
}

pub(crate) fn sv_streaming_source() -> SVTestSource<'static> {
    SVTestSource {
        value: SVColumnView::Streaming(SVStreamingView::new(&VALUES, 8, 0)),
        series: None,
    }
}

pub(crate) fn sv_streaming_series_source() -> SVTestSource<'static> {
    SVTestSource {
        value: SVColumnView::Streaming(SVStreamingView::new(&VALUES, 8, 0)),
        series: Some(SVColumnView::Streaming(SVStreamingView::new(&SERIES, 8, 0))),
    }
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVHistogramPlot<'_> {
    SVHistogramPlot::builder()
        .data_source(source)
        .value("value")
        .bin_count(3)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_scene_to_svg(
    scene: &crate::sv_recipes::sv_distribution::sv_histogram_plot::SVHistogramScene,
) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("histogram"))
        .with_description(scene.description().unwrap_or("histogram"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}

pub(crate) fn sv_scene_text(
    scene: &crate::sv_recipes::sv_distribution::sv_histogram_plot::SVHistogramScene,
) -> String {
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
