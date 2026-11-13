//! Typed backend dispatch and deterministic output.

use super::SVHeatmapMatrix;
use super::SVHeatmapPlot;
use super::test_support::{sv_build_default, sv_dense_source};
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;

fn sv_matrix() -> SVHeatmapMatrix<'static> {
    const VALUES: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
    SVHeatmapMatrix::new(&VALUES, 2, 2).expect("the shape matches")
}

#[test]
fn an_svg_render_is_a_self_contained_deterministic_document() {
    let plot = SVHeatmapPlot::builder()
        .matrix(sv_matrix())
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let first = plot.render().expect("the SVG render succeeds");
    let second = plot.render().expect("the SVG render repeats");
    assert_eq!(first, second, "the render is deterministic");
    assert_eq!(first.backend_kind(), SVBackendKind::Svg);
    let document = first.as_svg().expect("the SVG artifact");
    let text = String::from_utf8_lossy(document);
    assert!(text.contains("<svg"), "a self-contained SVG document");
    assert!(text.contains("<rect"), "the cells are rectangles");
}

#[cfg(feature = "render-native")]
#[test]
fn a_native_render_produces_an_rgba_frame() {
    let plot = SVHeatmapPlot::builder()
        .matrix(sv_matrix())
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");

    let output = plot.render().expect("the native render succeeds");
    let frame = output.as_native_frame().expect("the native artifact");
    assert!(frame.width() > 0 && frame.height() > 0);
    assert_eq!(frame.pixels().len(), frame.width() * frame.height() * 4);
    assert!(
        frame.pixels().iter().any(|channel| *channel != 0),
        "the frame carries drawn pixels"
    );
}

#[test]
fn a_kind_without_a_recipe_serializer_reports_a_typed_error() {
    let plot = SVHeatmapPlot::builder()
        .matrix(sv_matrix())
        .backend(SVBackendKind::Html)
        .build()
        .expect("the HTML backend supports image cells");
    let error = plot.render().expect_err("no HTML serializer exists yet");
    assert!(matches!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            ..
        }
    ));
}

#[test]
fn the_scene_serializer_refuses_a_foreign_backend() {
    let source = sv_dense_source();
    let _scene = sv_build_default(&source)
        .compose()
        .expect("the SVG scene composes");
    #[cfg(feature = "render-native")]
    {
        assert!(
            _scene.to_native_frame().is_err(),
            "a native serializer cannot draw an SVG scene"
        );
    }
}
