//! Sources and helpers shared by the swarm tests.
//!
//! Mock columns are borrowed slices, sparse views and synthetic streaming
//! snapshots; nothing here touches the network or the filesystem.

use super::{SVSwarmPlot, SVSwarmPlotBuilder, SVSwarmScene};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
    SVSparseView, SVStreamingView,
};
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkShape;

pub(crate) const CATEGORY: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0];
pub(crate) const VALUES: [f64; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 1.5, 2.5, 3.5, 4.5, 5.5];
pub(crate) const SERIES: [f64; 10] = [0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0];

const INDICES: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub(crate) struct SVTestSource<'values> {
    pub(crate) columns: Vec<(&'values str, SVColumnRole, SVColumnView<'values>)>,
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

pub(crate) fn dense_source<'values>() -> SVTestSource<'values> {
    SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Dense(&VALUES)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Dense(&CATEGORY),
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
        SVSparseView::new(&INDICES, &CATEGORY, CATEGORY.len()).expect("valid sparse storage");
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
            SVColumnView::Streaming(SVStreamingView::new(&VALUES, 10, 0)),
        ),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Streaming(SVStreamingView::new(&CATEGORY, 10, 0)),
        ),
        (
            "series",
            SVColumnRole::Grouping,
            SVColumnView::Streaming(SVStreamingView::new(&SERIES, 10, 0)),
        ),
    ])
}

pub(crate) fn default_builder(source: &dyn SVDataSource) -> SVSwarmPlotBuilder<'_> {
    SVSwarmPlot::builder()
        .data_source(source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
}

pub(crate) fn scene(source: &dyn SVDataSource) -> SVSwarmScene {
    default_builder(source)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes")
}

pub(crate) fn scene_svg(scene: &SVSwarmScene) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("swarm plot"))
        .with_description(scene.description().unwrap_or("swarm plot"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}

pub(crate) fn circle_centres(scene: &SVSwarmScene) -> Vec<(f64, f64)> {
    scene
        .mark_geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .filter_map(|primitive| match primitive.shape() {
            SVMarkShape::Circle(circle) => Some((circle.centre().x(), circle.centre().y())),
            _ => None,
        })
        .collect()
}

pub(crate) fn scene_text(scene: &SVSwarmScene) -> String {
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

pub(crate) fn assert_close(first: &[(f64, f64)], second: &[(f64, f64)]) {
    assert_eq!(first.len(), second.len(), "the same number of dots");
    for (left, right) in first.iter().zip(second) {
        assert!((left.0 - right.0).abs() < 1e-9, "x agrees");
        assert!((left.1 - right.1).abs() < 1e-9, "y agrees");
    }
}
