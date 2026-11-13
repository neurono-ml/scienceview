//! Shared mock data and scene helpers for the joint-plot tests.

use super::{SVJointKind, SVJointMarginal, SVJointPlot};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_error::SVError;

pub(crate) const HORIZONTAL: [f64; 8] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
pub(crate) const VERTICAL: [f64; 8] = [1.0, 4.0, 2.0, 5.0, 3.0, 6.0, 2.5, 5.5];
pub(crate) const SERIES: [f64; 8] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 0.0, 1.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVJointTestSource<'a> {
    pub horizontal: SVColumnView<'a>,
    pub vertical: SVColumnView<'a>,
    pub series: Option<SVColumnView<'a>>,
}

impl SVJointTestSource<'static> {
    pub(crate) fn dense() -> Self {
        Self {
            horizontal: SVColumnView::Dense(&HORIZONTAL),
            vertical: SVColumnView::Dense(&VERTICAL),
            series: None,
        }
    }

    pub(crate) fn grouped() -> Self {
        Self {
            horizontal: SVColumnView::Dense(&HORIZONTAL),
            vertical: SVColumnView::Dense(&VERTICAL),
            series: Some(SVColumnView::Dense(&SERIES)),
        }
    }

    pub(crate) fn sparse() -> Self {
        const INDICES: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        Self {
            horizontal: SVColumnView::Sparse(
                SVSparseView::new(&INDICES, &HORIZONTAL, 8).expect("the sparse x view is valid"),
            ),
            vertical: SVColumnView::Sparse(
                SVSparseView::new(&INDICES, &VERTICAL, 8).expect("the sparse y view is valid"),
            ),
            series: None,
        }
    }

    pub(crate) fn streaming() -> Self {
        Self {
            horizontal: SVColumnView::Streaming(SVStreamingView::new(&HORIZONTAL, 8, 0)),
            vertical: SVColumnView::Streaming(SVStreamingView::new(&VERTICAL, 8, 0)),
            series: None,
        }
    }
}

impl SVDataSource for SVJointTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the joint recipe resolves columns by name, not by schema")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let view = match name {
            "x" => self.horizontal,
            "y" => self.vertical,
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

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVJointPlot<'_> {
    SVJointPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is complete")
}

pub(crate) fn sv_build_kind(
    source: &dyn SVDataSource,
    kind: SVJointKind,
    marginal_kind: SVJointMarginal,
) -> SVJointPlot<'_> {
    SVJointPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .kind(kind)
        .marginal_kind(marginal_kind)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
}

/// Joins every text run the scene decorates, in draw order.
pub(crate) fn sv_scene_text(
    scene: &crate::sv_recipes::sv_multivariate::sv_joint_plot::SVJointScene,
) -> String {
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
