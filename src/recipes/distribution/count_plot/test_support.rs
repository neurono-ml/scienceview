//! Shared mock data and scene helpers for the count-plot tests.
//!
//! Mock columns are borrowed slices, sparse views and synthetic streaming
//! snapshots; nothing here touches the network or the filesystem.

use super::SVCountPlot;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
    SVSparseView, SVStreamingView,
};
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_error::SVError;

/// Three categories: `0` once, `1` twice and `2` three times.
pub(crate) const CATEGORIES: [f64; 6] = [0.0, 1.0, 1.0, 2.0, 2.0, 2.0];
/// Two series splitting the same six observations.
pub(crate) const SERIES: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 0.0, 1.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVTestSource<'a> {
    pub(crate) category: SVColumnView<'a>,
    pub(crate) series: Option<SVColumnView<'a>>,
}

impl SVTestSource<'_> {
    pub(crate) fn dense(category: &'static [f64]) -> SVTestSource<'static> {
        SVTestSource {
            category: SVColumnView::Dense(category),
            series: None,
        }
    }
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        let mut columns = vec![SVColumnDescriptor::new("category", SVColumnRole::Grouping)];
        if self.series.is_some() {
            columns.push(SVColumnDescriptor::new("series", SVColumnRole::Grouping));
        }
        SVSourceSchema::owned(columns)
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "category" => self.category,
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
    SVTestSource::dense(&CATEGORIES)
}

pub(crate) fn sv_series_source() -> SVTestSource<'static> {
    SVTestSource {
        category: SVColumnView::Dense(&CATEGORIES),
        series: Some(SVColumnView::Dense(&SERIES)),
    }
}

pub(crate) fn sv_sparse_source() -> SVTestSource<'static> {
    const INDICES: [usize; 5] = [1, 2, 3, 4, 5];
    const STORED: [f64; 5] = [1.0, 1.0, 2.0, 2.0, 2.0];
    let sparse =
        SVSparseView::new(&INDICES, &STORED, CATEGORIES.len()).expect("the sparse view is valid");
    SVTestSource {
        category: SVColumnView::Sparse(sparse),
        series: None,
    }
}

pub(crate) fn sv_sparse_series_source() -> SVTestSource<'static> {
    const INDICES: [usize; 5] = [1, 2, 3, 4, 5];
    const STORED: [f64; 5] = [1.0, 1.0, 2.0, 2.0, 2.0];
    let sparse =
        SVSparseView::new(&INDICES, &STORED, CATEGORIES.len()).expect("the sparse view is valid");
    SVTestSource {
        category: SVColumnView::Sparse(sparse),
        series: Some(SVColumnView::Dense(&SERIES)),
    }
}

pub(crate) fn sv_streaming_source() -> SVTestSource<'static> {
    SVTestSource {
        category: SVColumnView::Streaming(SVStreamingView::new(
            &CATEGORIES,
            CATEGORIES.len() as u64,
            0,
        )),
        series: None,
    }
}

pub(crate) fn sv_streaming_series_source() -> SVTestSource<'static> {
    SVTestSource {
        category: SVColumnView::Streaming(SVStreamingView::new(
            &CATEGORIES,
            CATEGORIES.len() as u64,
            0,
        )),
        series: Some(SVColumnView::Streaming(SVStreamingView::new(
            &SERIES,
            SERIES.len() as u64,
            0,
        ))),
    }
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVCountPlot<'_> {
    SVCountPlot::builder()
        .data_source(source)
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_build_series(source: &dyn SVDataSource) -> SVCountPlot<'_> {
    SVCountPlot::builder()
        .data_source(source)
        .category("category")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the series configuration is complete")
}

pub(crate) fn sv_scene_to_svg(
    scene: &crate::sv_recipes::sv_distribution::sv_count_plot::SVCountScene,
) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("count plot"))
        .with_description(scene.description().unwrap_or("count plot"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}

pub(crate) fn sv_scene_text(
    scene: &crate::sv_recipes::sv_distribution::sv_count_plot::SVCountScene,
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
