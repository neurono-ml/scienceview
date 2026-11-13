//! Sources and helpers shared by the bar-plot tests.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
    SVSparseView, SVStreamingView,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::SVMarkShape;

use super::{SVBarPlotBuilder, SVBarRenderOutput};

/// A source over borrowed column views.
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

pub(crate) const VALUES: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
pub(crate) const CATEGORIES: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
pub(crate) const SERIES: [f64; 6] = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
const INDICES: [usize; 6] = [0, 1, 2, 3, 4, 5];

pub(crate) fn dense_source<'values>() -> SVTestSource<'values> {
    SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Dense(&VALUES)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Dense(&CATEGORIES),
        ),
        (
            "series",
            SVColumnRole::Grouping,
            SVColumnView::Dense(&SERIES),
        ),
    ])
}

pub(crate) fn sparse_source<'values>() -> SVTestSource<'values> {
    let values = SVSparseView::new(&INDICES, &VALUES, VALUES.len()).expect("valid sparse storage");
    let categories =
        SVSparseView::new(&INDICES, &CATEGORIES, CATEGORIES.len()).expect("valid sparse storage");
    let series = SVSparseView::new(&INDICES, &SERIES, SERIES.len()).expect("valid sparse storage");
    SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Sparse(values)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Sparse(categories),
        ),
        (
            "series",
            SVColumnRole::Grouping,
            SVColumnView::Sparse(series),
        ),
    ])
}

pub(crate) fn streaming_source<'values>() -> SVTestSource<'values> {
    SVTestSource::new(vec![
        (
            "value",
            SVColumnRole::Y,
            SVColumnView::Streaming(SVStreamingView::new(&VALUES, 6, 0)),
        ),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Streaming(SVStreamingView::new(&CATEGORIES, 6, 0)),
        ),
        (
            "series",
            SVColumnRole::Grouping,
            SVColumnView::Streaming(SVStreamingView::new(&SERIES, 6, 0)),
        ),
    ])
}

pub(crate) fn render(source: &dyn SVDataSource) -> SVBarRenderOutput {
    SVBarPlotBuilder::new()
        .data_source(source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the bar plot draws")
}

/// Renders through the series path, using the `series` column of the source.
pub(crate) fn render_series(source: &dyn SVDataSource) -> SVBarRenderOutput {
    SVBarPlotBuilder::new()
        .data_source(source)
        .value("value")
        .category("category")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the bar plot draws")
}

/// The drawn bar rectangles as `(x, y, width, height)`.
pub(crate) fn rectangles(output: &SVBarRenderOutput) -> Vec<(f64, f64, f64, f64)> {
    output
        .geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .filter_map(|primitive| match primitive.shape() {
            SVMarkShape::Rectangle(rectangle) => {
                let rectangle = rectangle.rectangle();
                Some((rectangle.x, rectangle.y, rectangle.width, rectangle.height))
            }
            _ => None,
        })
        .collect()
}

pub(crate) fn assert_close(first: &[(f64, f64, f64, f64)], second: &[(f64, f64, f64, f64)]) {
    assert_eq!(first.len(), second.len());
    for (first, second) in first.iter().zip(second) {
        for index in 0..4 {
            let first = [first.0, first.1, first.2, first.3][index];
            let second = [second.0, second.1, second.2, second.3][index];
            assert!((first - second).abs() < 1e-9, "{first:?} vs {second:?}");
        }
    }
}
