//! Sources and helpers shared by the box-plot parity and concurrency tests.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
    SVSparseView, SVStreamingView,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVMarkShape, SVPathCommand};

use super::{SVBoxPlotBuilder, SVBoxRenderOutput};

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

pub(crate) const CATEGORY: [f64; 10] = [0.0; 10];
pub(crate) const VALUES: [f64; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
const INDICES: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub(crate) fn dense_source<'values>() -> SVTestSource<'values> {
    SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Dense(&VALUES)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Dense(&CATEGORY),
        ),
    ])
}

pub(crate) fn sparse_source<'values>() -> SVTestSource<'values> {
    let values = SVSparseView::new(&INDICES, &VALUES, VALUES.len()).expect("valid sparse storage");
    let categories =
        SVSparseView::new(&INDICES, &CATEGORY, CATEGORY.len()).expect("valid sparse storage");
    SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Sparse(values)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Sparse(categories),
        ),
    ])
}

pub(crate) fn streaming_source<'values>() -> SVTestSource<'values> {
    SVTestSource::new(vec![
        (
            "value",
            SVColumnRole::Y,
            SVColumnView::Streaming(SVStreamingView::new(&VALUES, 10, 0)),
        ),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Streaming(SVStreamingView::new(&CATEGORY, 10, 0)),
        ),
    ])
}

pub(crate) fn render(source: &dyn SVDataSource) -> SVBoxRenderOutput {
    SVBoxPlotBuilder::new()
        .data_source(source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the box plot draws")
}

pub(crate) fn points(output: &SVBoxRenderOutput) -> Vec<(f64, f64)> {
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
        assert!((first.0 - second.0).abs() < 1e-9, "{first:?} vs {second:?}");
        assert!((first.1 - second.1).abs() < 1e-9, "{first:?} vs {second:?}");
    }
}
