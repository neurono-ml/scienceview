//! Shared mock data and scene helpers for the parallel categories tests.
//!
//! Mock columns are borrowed slices, sparse views and synthetic streaming
//! snapshots; nothing here touches the network or the filesystem.

use super::SVParallelCategoriesPlot;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
    SVSparseView, SVStreamingView,
};
use crate::sv_error::SVError;

/// Axis one: two categories, `0` five times and `1` seven times.
pub(crate) const FIRST: [f64; 12] = [0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0];
/// Axis two: three categories.
pub(crate) const SECOND: [f64; 12] = [0.0, 1.0, 0.0, 1.0, 2.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0];
/// Axis three: two categories.
pub(crate) const THIRD: [f64; 12] = [0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
/// A grouping column that splits the same observations into three bundles.
pub(crate) const GROUP: [f64; 12] = [0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVTestSource<'a> {
    pub(crate) first: SVColumnView<'a>,
    pub(crate) second: SVColumnView<'a>,
    pub(crate) third: SVColumnView<'a>,
    pub(crate) colour: Option<SVColumnView<'a>>,
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        let mut columns = vec![
            SVColumnDescriptor::new("first", SVColumnRole::Grouping),
            SVColumnDescriptor::new("second", SVColumnRole::Grouping),
            SVColumnDescriptor::new("third", SVColumnRole::Grouping),
        ];
        if self.colour.is_some() {
            columns.push(SVColumnDescriptor::new("colour", SVColumnRole::Grouping));
        }
        SVSourceSchema::owned(columns)
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "first" => self.first,
            "second" => self.second,
            "third" => self.third,
            "colour" => self.colour.ok_or_else(|| SVError::ColumnNotFound {
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

/// A dense source with all three axes.
pub(crate) fn sv_dense_source() -> SVTestSource<'static> {
    SVTestSource {
        first: SVColumnView::Dense(&FIRST),
        second: SVColumnView::Dense(&SECOND),
        third: SVColumnView::Dense(&THIRD),
        colour: None,
    }
}

/// A dense source with a grouping column.
pub(crate) fn sv_colour_source() -> SVTestSource<'static> {
    let mut source = sv_dense_source();
    source.colour = Some(SVColumnView::Dense(&GROUP));
    source
}

fn sv_sparse_view(values: &'static [f64]) -> SVColumnView<'static> {
    let mut indices = Vec::new();
    let mut stored = Vec::new();
    for (index, value) in values.iter().copied().enumerate() {
        if value != 0.0 {
            indices.push(index);
            stored.push(value);
        }
    }
    let indices: &'static [usize] = Box::leak(indices.into_boxed_slice());
    let stored: &'static [f64] = Box::leak(stored.into_boxed_slice());
    SVColumnView::Sparse(
        SVSparseView::new(indices, stored, values.len()).expect("the sparse view is valid"),
    )
}

/// A sparse source: every zero is an implicit zero.
pub(crate) fn sv_sparse_source() -> SVTestSource<'static> {
    SVTestSource {
        first: sv_sparse_view(&FIRST),
        second: sv_sparse_view(&SECOND),
        third: sv_sparse_view(&THIRD),
        colour: None,
    }
}

/// A streaming source: every column is a bounded window snapshot.
pub(crate) fn sv_streaming_source() -> SVTestSource<'static> {
    SVTestSource {
        first: SVColumnView::Streaming(SVStreamingView::new(&FIRST, FIRST.len() as u64, 0)),
        second: SVColumnView::Streaming(SVStreamingView::new(&SECOND, SECOND.len() as u64, 0)),
        third: SVColumnView::Streaming(SVStreamingView::new(&THIRD, THIRD.len() as u64, 0)),
        colour: None,
    }
}

/// A source mixing one representation per axis.
pub(crate) fn sv_mixed_source() -> SVTestSource<'static> {
    SVTestSource {
        first: sv_sparse_view(&FIRST),
        second: SVColumnView::Dense(&SECOND),
        third: SVColumnView::Streaming(SVStreamingView::new(&THIRD, THIRD.len() as u64, 0)),
        colour: None,
    }
}

/// Builds the documented default plot over three axes.
pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVParallelCategoriesPlot<'_> {
    SVParallelCategoriesPlot::builder()
        .data_source(source)
        .variables(&["first", "second", "third"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
}

#[cfg(feature = "render-svg")]
pub(crate) fn sv_scene_to_svg(scene: &super::SVParallelCategoriesScene) -> String {
    scene.to_svg().expect("the scene serializes")
}
