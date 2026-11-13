//! The backend dispatch: a configured kind is resolved, never ignored.

use super::SVJointPlot;
#[cfg(feature = "render-svg")]
use super::SVJointRenderOutput;
#[cfg(feature = "render-svg")]
use super::test_support::sv_build_default;
use crate::sv_backend::{SVBackendCapability, SVBackendKind};
use crate::sv_data::SVDataSource;
use crate::sv_error::SVError;

fn sv_build_with_backend(
    source: &dyn SVDataSource,
    backend_kind: SVBackendKind,
) -> SVJointPlot<'_> {
    SVJointPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .backend(backend_kind)
        .build()
        .expect("the configuration is complete")
}

#[cfg(feature = "render-svg")]
#[test]
fn the_svg_kind_serializes_the_svg_document() {
    let source = super::test_support::SVJointTestSource::dense();
    let output = sv_build_default(&source)
        .render()
        .expect("the SVG backend renders");

    assert_eq!(output.backend_kind(), SVBackendKind::Svg);
    let document = std::str::from_utf8(output.as_svg().expect("the output is SVG"))
        .expect("the document is UTF-8");
    assert!(document.starts_with("<svg"));
    assert!(document.contains("viewBox=\"0 0 760 600\""));
    assert!(document.contains("<path"));
    assert!(matches!(output, SVJointRenderOutput::Svg(_)));
}

#[cfg(feature = "render-native")]
#[test]
fn the_native_kind_rasterizes_the_native_frame() {
    let source = super::test_support::SVJointTestSource::dense();
    let output = sv_build_with_backend(&source, SVBackendKind::Native)
        .render()
        .expect("the native backend renders");

    let frame = output
        .as_native_frame()
        .expect("the output is a native frame");
    assert_eq!(frame.width(), 760);
    assert_eq!(frame.height(), 600);
    assert!(frame.pixels().chunks_exact(4).any(|pixel| pixel[3] > 0));
}

#[test]
fn a_kind_without_a_recipe_serializer_reports_its_capability() {
    let source = super::test_support::SVJointTestSource::dense();
    let error = sv_build_with_backend(&source, SVBackendKind::Html)
        .render()
        .expect_err("the recipe has no HTML serializer");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            capability: SVBackendCapability::Export,
        }
    );
}
