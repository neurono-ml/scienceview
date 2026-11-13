//! Shared mock data and scene helpers for the hexbin tests.
//!
//! The fixture is served as a dense slice, a sparse view and a synthetic
//! streaming window through one source seam, so the parity test differs only in
//! the representation it presents.

use super::SVHexbinPlot;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_error::SVError;

pub(crate) const HORIZONTAL: [f64; 10] = [0.0, 0.2, 0.8, 2.0, 2.2, 2.4, 4.0, 4.1, 4.2, 4.3];
pub(crate) const VERTICAL: [f64; 10] = [0.0, 0.1, 0.2, 2.0, 2.1, 2.2, 4.0, 4.1, 4.2, 4.3];
pub(crate) const VALUES: [f64; 10] = [1.0, 2.0, 3.0, 10.0, 20.0, 30.0, 100.0, 200.0, 300.0, 400.0];
pub(crate) const SERIES: [f64; 10] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVTestSource<'a> {
    pub(crate) horizontal: SVColumnView<'a>,
    pub(crate) vertical: SVColumnView<'a>,
    pub(crate) series: Option<SVColumnView<'a>>,
    pub(crate) values: Option<SVColumnView<'a>>,
}

impl SVTestSource<'_> {
    pub(crate) fn dense() -> SVTestSource<'static> {
        SVTestSource {
            horizontal: SVColumnView::Dense(&HORIZONTAL),
            vertical: SVColumnView::Dense(&VERTICAL),
            series: None,
            values: None,
        }
    }
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the hexbin recipe resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "x" => self.horizontal,
            "y" => self.vertical,
            "series" => self.series.ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?,
            "value" => self.values.ok_or_else(|| SVError::ColumnNotFound {
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
    SVTestSource::dense()
}

pub(crate) fn sv_full_source() -> SVTestSource<'static> {
    SVTestSource {
        horizontal: SVColumnView::Dense(&HORIZONTAL),
        vertical: SVColumnView::Dense(&VERTICAL),
        series: Some(SVColumnView::Dense(&SERIES)),
        values: Some(SVColumnView::Dense(&VALUES)),
    }
}

pub(crate) fn sv_sparse_source() -> SVTestSource<'static> {
    const INDICES: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let horizontal = SVSparseView::new(&INDICES, &HORIZONTAL, 10)
        .expect("the sparse horizontal view is well formed");
    let vertical = SVSparseView::new(&INDICES, &VERTICAL, 10)
        .expect("the sparse vertical view is well formed");
    SVTestSource {
        horizontal: SVColumnView::Sparse(horizontal),
        vertical: SVColumnView::Sparse(vertical),
        series: None,
        values: None,
    }
}

pub(crate) fn sv_streaming_source() -> SVTestSource<'static> {
    SVTestSource {
        horizontal: SVColumnView::Streaming(SVStreamingView::new(&HORIZONTAL, 10, 0)),
        vertical: SVColumnView::Streaming(SVStreamingView::new(&VERTICAL, 10, 0)),
        series: None,
        values: None,
    }
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVHexbinPlot<'_> {
    SVHexbinPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_render_svg(scene: &super::SVHexbinComposition) -> String {
    scene.to_svg().expect("the SVG scene serializes")
}

pub(crate) fn sv_scene_text(scene: &super::SVHexbinComposition) -> String {
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
