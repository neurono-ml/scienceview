//! The backend dispatch: a configured kind is resolved, never ignored.

use super::SVParallelCategoriesPlot;
#[cfg(feature = "render-svg")]
use super::SVParallelCategoriesRenderOutput;
#[cfg(feature = "render-svg")]
use super::test_support::sv_build_default;
use super::test_support::sv_dense_source;
use crate::sv_backend::{SVBackendCapability, SVBackendKind};
use crate::sv_error::SVError;

fn sv_build_with_backend(
    source: &dyn crate::sv_data::SVDataSource,
    backend_kind: SVBackendKind,
) -> SVParallelCategoriesPlot<'_> {
    SVParallelCategoriesPlot::builder()
        .data_source(source)
        .variables(&["first", "second", "third"])
        .backend(backend_kind)
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
    assert!(document.contains("viewBox=\"0 0 720 460\""));
    assert!(document.contains("<path"));
}

#[cfg(feature = "render-svg")]
#[test]
fn the_update_path_serializes_the_same_document_shape() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    let output = plot.update(2).expect("the update renders");

    assert_eq!(output.backend_kind(), SVBackendKind::Svg);
    assert!(matches!(output, SVParallelCategoriesRenderOutput::Svg(_)));
}

#[cfg(feature = "render-native")]
#[test]
fn the_native_kind_rasterizes_the_native_frame() {
    let source = sv_dense_source();
    let plot = sv_build_with_backend(&source, SVBackendKind::Native);
    let output = plot.render().expect("the native backend renders");

    assert_eq!(output.backend_kind(), SVBackendKind::Native);
    let frame = output
        .as_native_frame()
        .expect("the output is a native frame");
    assert_eq!(frame.width(), 720);
    assert_eq!(frame.height(), 460);
    let painted = frame
        .pixels()
        .chunks_exact(4)
        .filter(|pixel| pixel[3] > 0)
        .count();
    assert!(painted > 0, "the native frame carries drawn pixels");
}

#[test]
fn a_kind_without_a_recipe_serializer_fails_explicitly() {
    let source = sv_dense_source();
    let plot = sv_build_with_backend(&source, SVBackendKind::Html);
    let error = plot.render().expect_err("the HTML kind has no serializer");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            capability: SVBackendCapability::Export,
        }
    );
}

#[cfg(all(feature = "render-svg", feature = "render-native"))]
#[test]
fn to_svg_rejects_a_scene_composed_for_another_backend() {
    let source = sv_dense_source();
    let scene = sv_build_with_backend(&source, SVBackendKind::Native)
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
    let scene = sv_build_with_backend(&source, SVBackendKind::Svg)
        .compose()
        .expect("the SVG scene composes");
    let error = scene
        .to_native_frame()
        .expect_err("a native serializer rejects an SVG scene");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Svg,
            capability: SVBackendCapability::Presentation,
        }
    );
}
