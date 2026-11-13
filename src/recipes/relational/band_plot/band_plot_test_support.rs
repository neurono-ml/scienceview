//! Sources and helpers shared by the band-plot parity and concurrency tests.

use std::sync::{Mutex, PoisonError};

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
    SVSparseView, SVStreamingView,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVMarkShape, SVPathCommand};

use super::{SVBandPlotBuilder, SVBandRenderOutput};

pub(crate) struct SVTestSource<'values> {
    columns: Vec<(&'values str, SVColumnRole, SVColumnView<'values>)>,
}

impl<'values> SVTestSource<'values> {
    pub(crate) fn new(columns: Vec<(&'values str, SVColumnRole, SVColumnView<'values>)>) -> Self {
        Self { columns }
    }
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(
            self.columns
                .iter()
                .map(|(name, role, _)| SVColumnDescriptor::new(*name, *role))
                .collect(),
        )
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        self.columns
            .iter()
            .find(|(column_name, _, _)| *column_name == name)
            .map(|(_, _, view)| SVColumnSnapshot::Borrowed(*view))
            .ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })
    }
}

pub(crate) struct SVCountingSource<'source, 'values> {
    pub(crate) inner: &'source SVTestSource<'values>,
    pub(crate) reads: Mutex<Vec<String>>,
}

impl SVDataSource for SVCountingSource<'_, '_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        self.inner.schema()
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        self.reads
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(name.to_string());
        self.inner.column(name)
    }
}

pub(crate) const X: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
pub(crate) const LOWER: [f64; 5] = [0.5, 0.8, 1.2, 1.6, 2.0];
pub(crate) const UPPER: [f64; 5] = [1.5, 2.2, 3.1, 3.4, 4.0];
const INDICES: [usize; 5] = [0, 1, 2, 3, 4];

pub(crate) fn sparse_source<'values>() -> SVTestSource<'values> {
    let x = SVSparseView::new(&INDICES, &X, X.len()).expect("the sparse view is valid");
    let lower = SVSparseView::new(&INDICES, &LOWER, LOWER.len()).expect("the sparse view is valid");
    let upper = SVSparseView::new(&INDICES, &UPPER, UPPER.len()).expect("the sparse view is valid");
    SVTestSource::new(vec![
        ("x", SVColumnRole::X, SVColumnView::Sparse(x)),
        ("lower", SVColumnRole::Y, SVColumnView::Sparse(lower)),
        ("upper", SVColumnRole::Y, SVColumnView::Sparse(upper)),
    ])
}

pub(crate) fn dense_source<'values>() -> SVTestSource<'values> {
    SVTestSource::new(vec![
        ("x", SVColumnRole::X, SVColumnView::Dense(&X)),
        ("lower", SVColumnRole::Y, SVColumnView::Dense(&LOWER)),
        ("upper", SVColumnRole::Y, SVColumnView::Dense(&UPPER)),
    ])
}

pub(crate) fn streaming_source<'values>() -> SVTestSource<'values> {
    SVTestSource::new(vec![
        (
            "x",
            SVColumnRole::X,
            SVColumnView::Streaming(SVStreamingView::new(&X, 5, 0)),
        ),
        (
            "lower",
            SVColumnRole::Y,
            SVColumnView::Streaming(SVStreamingView::new(&LOWER, 5, 0)),
        ),
        (
            "upper",
            SVColumnRole::Y,
            SVColumnView::Streaming(SVStreamingView::new(&UPPER, 5, 0)),
        ),
    ])
}

pub(crate) fn render(source: &dyn SVDataSource) -> SVBandRenderOutput {
    SVBandPlotBuilder::new()
        .data_source(source)
        .x("x")
        .lower("lower")
        .upper("upper")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws")
}

pub(crate) fn points(output: &SVBandRenderOutput) -> Vec<(f64, f64)> {
    output
        .geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .filter_map(|primitive| match primitive.shape() {
            SVMarkShape::Path(path) => Some(path.commands()),
            _ => None,
        })
        .flatten()
        .filter_map(|command| match command {
            SVPathCommand::MoveTo(point) | SVPathCommand::LineTo(point) => {
                Some((point.x(), point.y()))
            }
            _ => None,
        })
        .collect()
}

pub(crate) fn assert_close(first: &[(f64, f64)], second: &[(f64, f64)]) {
    assert_eq!(first.len(), second.len());
    for (first, second) in first.iter().zip(second) {
        assert!((first.0 - second.0).abs() < 1e-9);
        assert!((first.1 - second.1).abs() < 1e-9);
    }
}
