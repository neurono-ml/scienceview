//! The backend dispatch: a configured kind is resolved, never ignored.

use super::SVStripPlot;
use super::test_support::{sv_build_default, sv_dense_source};
use crate::sv_backend::{SVBackendCapability, SVBackendKind};
use crate::sv_error::SVError;

#[cfg(feature = "render-native")]
fn sv_build_native(source: &dyn crate::sv_data::SVDataSource) -> SVStripPlot<'_> {
    SVStripPlot::builder()
        .data_source(source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete")
}

#[cfg(feature = "render-svg")]
#[test]
fn the_svg_kind_serializes_the_svg_document() {
    let source = sv_dense_source();
    let output = sv_build_default(&source)
        .render()
        .expect("the SVG backend renders");

    assert_eq!(output.backend_kind(), SVBackendKind::Svg);
    let document = std::str::from_utf8(output.as_svg().expect("the output is SVG"))
        .expect("the SVG document is UTF-8");
    assert!(document.starts_with("<svg"));
    assert!(document.contains("viewBox=\"0 0 640 430\""));
}

#[cfg(feature = "render-native")]
#[test]
fn the_native_kind_rasterizes_the_native_frame() {
    let source = sv_dense_source();
    let output = sv_build_native(&source)
        .render()
        .expect("the native backend renders");

    assert_eq!(output.backend_kind(), SVBackendKind::Native);
    let frame = output
        .as_native_frame()
        .expect("the output is a native frame");
    assert_eq!(frame.width(), 640);
    assert_eq!(frame.height(), 430);
    assert!(frame.pixels().chunks_exact(4).any(|pixel| pixel[3] > 0));
}

#[cfg(all(feature = "render-svg", feature = "render-native"))]
#[test]
fn to_svg_rejects_a_scene_composed_for_another_backend() {
    let source = sv_dense_source();
    let scene = sv_build_native(&source)
        .compose()
        .expect("the native scene composes");
    let error = scene
        .to_svg()
        .expect_err("an SVG serializer rejects a native scene");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Native,
            capability: SVBackendCapability::Export,
        }
    );
}

#[cfg(all(feature = "render-svg", feature = "render-native"))]
#[test]
fn to_native_frame_rejects_a_scene_composed_for_another_backend() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the SVG scene composes");
    let error = scene
        .to_native_frame()
        .expect_err("the rasterizer rejects an SVG scene");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Svg,
            capability: SVBackendCapability::Presentation,
        }
    );
}

#[test]
fn a_kind_without_a_recipe_serializer_fails_with_a_typed_error() {
    let source = sv_dense_source();
    let plot = SVStripPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Html)
        .build()
        .expect("the configuration is complete");
    let error = plot
        .render()
        .expect_err("the recipe has no HTML serializer yet");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            capability: SVBackendCapability::Export,
        }
    );
}

#[cfg(not(feature = "render-svg"))]
#[test]
fn a_compiled_out_svg_backend_reports_itself_unavailable() {
    let source = sv_dense_source();
    let error = sv_build_default(&source)
        .render()
        .expect_err("the SVG feature is disabled");

    assert_eq!(
        error,
        SVError::BackendUnavailable {
            backend_kind: SVBackendKind::Svg,
        }
    );
}
