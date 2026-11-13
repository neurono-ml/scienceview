//! Shared mock data and scene helpers for the image tests.
//!
//! Every raster is borrowed: dense slices, sparse views and synthetic streaming
//! snapshots. Nothing here touches the network or the filesystem.

use super::{SVImagePlot, SVImageScene, SVImageSource, SVRasterDimensions};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_error::SVError;

pub(crate) const ROWS: usize = 3;
pub(crate) const COLUMNS: usize = 4;

pub(crate) const FIELD: [f64; ROWS * COLUMNS] =
    [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0];

/// A synthetic source serving every representation through one seam.
pub(crate) struct SVTestSource<'a> {
    pub field: SVColumnView<'a>,
}

impl<'a> SVTestSource<'a> {
    pub(crate) fn dense() -> SVTestSource<'static> {
        SVTestSource {
            field: SVColumnView::Dense(&FIELD),
        }
    }

    pub(crate) fn sparse(view: SVSparseView<'a>) -> SVTestSource<'a> {
        SVTestSource {
            field: SVColumnView::Sparse(view),
        }
    }

    pub(crate) fn streaming(values: &'a [f64], sequence: u64) -> SVTestSource<'a> {
        SVTestSource {
            field: SVColumnView::Streaming(SVStreamingView::new(values, sequence, 0)),
        }
    }
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the image recipe resolves its raster column by name")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        if name == "field" {
            return Ok(SVColumnSnapshot::Borrowed(self.field));
        }
        Err(SVError::ColumnNotFound {
            name: name.to_string(),
        })
    }
}

pub(crate) fn dimensions() -> SVRasterDimensions {
    SVRasterDimensions::new(ROWS, COLUMNS).expect("the raster dimensions are positive")
}

pub(crate) fn builder(
    source: &dyn SVDataSource,
    backend_kind: SVBackendKind,
) -> SVImagePlotBuilderHelper {
    SVImagePlotBuilderHelper {
        source,
        backend_kind,
    }
}

/// A small helper that keeps each test from repeating the required options.
pub(crate) struct SVImagePlotBuilderHelper<'a> {
    source: &'a dyn SVDataSource,
    backend_kind: SVBackendKind,
}

impl<'a> SVImagePlotBuilderHelper<'a> {
    pub(crate) fn build(self) -> SVImagePlot<'a> {
        SVImagePlot::builder()
            .data_source(self.source)
            .raster(SVImageSource::intensity("field", dimensions()))
            .backend(self.backend_kind)
            .build()
            .expect("the image configuration is complete")
    }
}

pub(crate) fn sv_build_default(source: &dyn SVDataSource) -> SVImagePlot<'_> {
    builder(source, SVBackendKind::Svg).build()
}

pub(crate) fn sv_scene_to_svg(scene: &SVImageScene) -> String {
    #[cfg(feature = "render-svg")]
    {
        scene.to_svg().expect("the scene serializes as SVG")
    }
    #[cfg(not(feature = "render-svg"))]
    {
        let _ = scene;
        String::new()
    }
}
