//! Shared mock data and scene helpers for the pair-grid tests.

use super::SVPairScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource,
    SVRepresentation, SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_error::SVError;

pub(crate) const FIRST: [f64; 9] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
pub(crate) const SECOND: [f64; 9] = [1.0, 4.0, 2.0, 5.0, 3.0, 6.0, 2.5, 7.5, 4.5];
pub(crate) const THIRD: [f64; 9] = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 3.0, 4.0, 5.0];
pub(crate) const HUE: [f64; 9] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
const INDICES: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVPairTestSource {
    columns: Vec<(&'static str, SVColumnView<'static>)>,
}

impl SVPairTestSource {
    fn dense() -> Self {
        Self::with_representation(SVRepresentation::Dense)
    }

    fn sparse() -> Self {
        Self::with_representation(SVRepresentation::Sparse)
    }

    fn streaming() -> Self {
        Self::with_representation(SVRepresentation::Streaming)
    }

    fn with_representation(representation: SVRepresentation) -> Self {
        let view = |values: &'static [f64]| match representation {
            SVRepresentation::Dense => SVColumnView::Dense(values),
            SVRepresentation::Sparse => SVColumnView::Sparse(
                SVSparseView::new(&INDICES, values, values.len())
                    .expect("the sparse view is well formed"),
            ),
            SVRepresentation::Streaming => {
                SVColumnView::Streaming(SVStreamingView::new(values, 9, 0))
            }
        };
        let columns = vec![
            ("first", view(&FIRST)),
            ("second", view(&SECOND)),
            ("third", view(&THIRD)),
            ("species", view(&HUE)),
        ];
        Self { columns }
    }
}

impl SVDataSource for SVPairTestSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(vec![
            SVColumnDescriptor::new("first", SVColumnRole::X),
            SVColumnDescriptor::new("second", SVColumnRole::Y),
            SVColumnDescriptor::new("third", SVColumnRole::Y),
            SVColumnDescriptor::new("species", SVColumnRole::Grouping),
        ])
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        self.columns
            .iter()
            .find(|(candidate, _)| *candidate == name)
            .map(|(_, view)| SVColumnSnapshot::Borrowed(*view))
            .ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })
    }
}

pub(crate) fn sv_dense_source() -> SVPairTestSource {
    SVPairTestSource::dense()
}

pub(crate) fn sv_sparse_source() -> SVPairTestSource {
    SVPairTestSource::sparse()
}

pub(crate) fn sv_streaming_source() -> SVPairTestSource {
    SVPairTestSource::streaming()
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> super::SVPairPlot<'_> {
    super::SVPairPlot::builder()
        .data_source(source)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_scene_to_svg(scene: &SVPairScene) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("pair plot"))
        .with_description(scene.description().unwrap_or("pair plot"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}
