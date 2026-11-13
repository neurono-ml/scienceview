//! Shared mock data and scene helpers for the scatterplot-matrix tests.
//!
//! Mock columns are borrowed slices, sparse views and synthetic streaming
//! snapshots; nothing here touches the network or the filesystem.

use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_error::SVError;
use crate::sv_recipes::sv_multivariate::sv_splom_plot::{SVSplomPlot, SVSplomScene};

pub(crate) const HEIGHT: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
pub(crate) const WEIGHT: [f64; 6] = [2.0, 4.0, 5.0, 8.0, 9.0, 11.0];
pub(crate) const AGE: [f64; 6] = [20.0, 25.0, 30.0, 35.0, 40.0, 45.0];
pub(crate) const GROUP: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
pub(crate) const VARIABLES: [&str; 3] = ["height", "weight", "age"];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVSplomTestSource<'a> {
    pub height: SVColumnView<'a>,
    pub weight: SVColumnView<'a>,
    pub age: SVColumnView<'a>,
    pub group: Option<SVColumnView<'a>>,
}

impl SVSplomTestSource<'_> {
    fn dense() -> SVSplomTestSource<'static> {
        SVSplomTestSource {
            height: SVColumnView::Dense(&HEIGHT),
            weight: SVColumnView::Dense(&WEIGHT),
            age: SVColumnView::Dense(&AGE),
            group: None,
        }
    }
}

impl SVDataSource for SVSplomTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the matrix recipe resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "height" => self.height,
            "weight" => self.weight,
            "age" => self.age,
            "group" => self.group.ok_or_else(|| SVError::ColumnNotFound {
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

pub(crate) fn sv_dense_source() -> SVSplomTestSource<'static> {
    SVSplomTestSource::dense()
}

pub(crate) fn sv_grouped_source() -> SVSplomTestSource<'static> {
    SVSplomTestSource {
        group: Some(SVColumnView::Dense(&GROUP)),
        ..SVSplomTestSource::dense()
    }
}

pub(crate) fn sv_sparse_source() -> SVSplomTestSource<'static> {
    const INDICES: [usize; 6] = [0, 1, 2, 3, 4, 5];
    let height = SVSparseView::new(&INDICES, &HEIGHT, 6).expect("the sparse height view is valid");
    let weight = SVSparseView::new(&INDICES, &WEIGHT, 6).expect("the sparse weight view is valid");
    let age = SVSparseView::new(&INDICES, &AGE, 6).expect("the sparse age view is valid");
    SVSplomTestSource {
        height: SVColumnView::Sparse(height),
        weight: SVColumnView::Sparse(weight),
        age: SVColumnView::Sparse(age),
        group: None,
    }
}

pub(crate) fn sv_streaming_source() -> SVSplomTestSource<'static> {
    SVSplomTestSource {
        height: SVColumnView::Streaming(SVStreamingView::new(&HEIGHT, 6, 0)),
        weight: SVColumnView::Streaming(SVStreamingView::new(&WEIGHT, 6, 0)),
        age: SVColumnView::Streaming(SVStreamingView::new(&AGE, 6, 0)),
        group: None,
    }
}

pub(crate) const HEIGHT_WITH_GAP: [f64; 6] = [1.0, 2.0, f64::NAN, 4.0, 5.0, 6.0];

/// A source whose first variable carries one non-finite observation.
pub(crate) struct SVGapSource;

impl SVDataSource for SVGapSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the matrix recipe resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "height" => SVColumnView::Dense(&HEIGHT_WITH_GAP),
            "weight" => SVColumnView::Dense(&WEIGHT),
            "age" => SVColumnView::Dense(&AGE),
            _ => {
                return Err(SVError::ColumnNotFound {
                    name: name.to_string(),
                });
            }
        };
        Ok(SVColumnSnapshot::Borrowed(view))
    }
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVSplomPlot<'_> {
    SVSplomPlot::builder()
        .data_source(source)
        .variables(&VARIABLES)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_scene_to_svg(scene: &SVSplomScene) -> String {
    let mut document = scienceview_svg::SVSvgDocument::new(scene.viewport())
        .with_title(scene.title().unwrap_or("scatterplot matrix"))
        .with_description(scene.description().unwrap_or("scatterplot matrix"))
        .with_background(scene.background());
    document.push_decoration_primitives(scene.decoration_primitives_before_marks());
    for geometry in scene.mark_geometries() {
        document.push_mark_geometry(geometry);
    }
    document.push_decoration_primitives(scene.decoration_primitives_after_marks());
    document.to_svg()
}

pub(crate) fn sv_scene_text(scene: &SVSplomScene) -> String {
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
