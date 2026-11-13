//! Shared mock data and scene helpers for the scatter tests.
//!
//! Mock columns are borrowed slices, sparse views and synthetic streaming
//! snapshots; nothing here touches the network or the filesystem.

use super::SVScatterPlot;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_error::SVError;

pub(crate) const HORIZONTAL: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
pub(crate) const VERTICAL: [f64; 5] = [1.0, 4.0, 2.0, 5.0, 3.0];
pub(crate) const SERIES: [f64; 5] = [0.0, 0.0, 1.0, 1.0, 2.0];
pub(crate) const SIZES: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
pub(crate) const COLOURS: [f64; 5] = [10.0, 20.0, 30.0, 40.0, 50.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVTestSource<'a> {
    pub horizontal: SVColumnView<'a>,
    pub vertical: SVColumnView<'a>,
    pub series: Option<SVColumnView<'a>>,
    pub sizes: Option<SVColumnView<'a>>,
    pub colours: Option<SVColumnView<'a>>,
}

impl SVTestSource<'_> {
    pub(crate) fn dense(
        horizontal: &'static [f64],
        vertical: &'static [f64],
    ) -> SVTestSource<'static> {
        SVTestSource {
            horizontal: SVColumnView::Dense(horizontal),
            vertical: SVColumnView::Dense(vertical),
            series: None,
            sizes: None,
            colours: None,
        }
    }
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the scatter recipe resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "x" => self.horizontal,
            "y" => self.vertical,
            "series" => self.series.ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?,
            "size" => self.sizes.ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?,
            "colour" => self.colours.ok_or_else(|| SVError::ColumnNotFound {
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
    SVTestSource::dense(&HORIZONTAL, &VERTICAL)
}

pub(crate) fn sv_full_source() -> SVTestSource<'static> {
    SVTestSource {
        horizontal: SVColumnView::Dense(&HORIZONTAL),
        vertical: SVColumnView::Dense(&VERTICAL),
        series: Some(SVColumnView::Dense(&SERIES)),
        sizes: Some(SVColumnView::Dense(&SIZES)),
        colours: Some(SVColumnView::Dense(&COLOURS)),
    }
}

pub(crate) fn sv_sparse_source() -> SVTestSource<'static> {
    const HORIZONTAL_INDICES: [usize; 5] = [0, 1, 2, 3, 4];
    const VERTICAL_INDICES: [usize; 5] = [0, 1, 2, 3, 4];
    let horizontal = SVSparseView::new(&HORIZONTAL_INDICES, &HORIZONTAL, 5)
        .expect("the sparse horizontal view is well formed");
    let vertical =
        SVSparseView::new(&VERTICAL_INDICES, &VERTICAL, 5).expect("the sparse view is well formed");
    SVTestSource {
        horizontal: SVColumnView::Sparse(horizontal),
        vertical: SVColumnView::Sparse(vertical),
        series: None,
        sizes: None,
        colours: None,
    }
}

pub(crate) fn sv_streaming_source() -> SVTestSource<'static> {
    SVTestSource {
        horizontal: SVColumnView::Streaming(SVStreamingView::new(&HORIZONTAL, 5, 0)),
        vertical: SVColumnView::Streaming(SVStreamingView::new(&VERTICAL, 5, 0)),
        series: None,
        sizes: None,
        colours: None,
    }
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVScatterPlot<'_> {
    SVScatterPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_scene_to_svg(
    scene: &crate::recipes::relational::scatter_plot::SVScatterScene,
) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("scatter plot"))
        .with_description(scene.description().unwrap_or("scatter plot"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}

pub(crate) fn sv_scene_text(
    scene: &crate::recipes::relational::scatter_plot::SVScatterScene,
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
