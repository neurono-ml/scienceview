//! The SVG and native serializers of one swarm scene.

use super::SVSwarmPlot;
use super::test_support::{circle_centres, dense_source, scene, scene_svg};
use crate::sv_backend::{SVBackendCapability, SVBackendKind};
use crate::sv_error::SVError;

#[test]
fn the_svg_document_is_self_contained_and_deterministic() {
    let first = scene_svg(&scene(&dense_source()));
    let second = scene_svg(&scene(&dense_source()));

    assert!(first.starts_with("<svg"));
    assert_eq!(first, second);
    assert!(!first.contains("<image"), "the figure embeds no raster");
    assert!(
        !first.contains("xlink:href"),
        "the figure references nothing external"
    );
    assert!(first.contains("<circle") || first.contains("<path"));
}

#[test]
fn render_serializes_the_configured_svg_backend() {
    let source = dense_source();
    let output = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the SVG render succeeds");

    assert_eq!(output.backend_kind(), SVBackendKind::Svg);
    let svg = output.as_svg().expect("the SVG artifact is present");
    assert!(svg.starts_with(b"<svg"));
}

#[cfg(feature = "render-native")]
#[test]
fn the_native_frame_paints_every_dot() {
    let source = dense_source();
    let scene = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");
    let frame = scene
        .to_native_frame()
        .expect("the native serializer accepts its kind");
    assert_eq!(frame.width(), scene.viewport().width() as usize);
    assert_eq!(frame.height(), scene.viewport().height() as usize);
    let background = crate::sv_decoration::SVDecorationStyle::built_in().background();
    let painted = circle_centres(&scene)
        .iter()
        .filter(|(x, y)| {
            let device_x = x.round().max(0.0) as usize;
            let device_y = y.round().max(0.0) as usize;
            frame
                .pixel(device_x, device_y)
                .is_some_and(|pixel| pixel != background)
        })
        .count();
    assert_eq!(painted, scene.number_of_marks());
}

#[cfg(feature = "render-native")]
#[test]
fn render_serializes_the_configured_native_backend() {
    let source = dense_source();
    let output = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the native render succeeds");
    assert_eq!(output.backend_kind(), SVBackendKind::Native);
    let frame = output
        .as_native_frame()
        .expect("the native frame is present");
    assert_eq!(frame.pixels().len(), frame.width() * frame.height() * 4);
}

#[cfg(feature = "render-native")]
#[test]
fn an_svg_scene_rejects_the_native_serializer() {
    let scene = scene(&dense_source());
    let error = scene
        .to_native_frame()
        .expect_err("an SVG scene is not a native frame");
    assert!(matches!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Svg,
            capability: SVBackendCapability::Presentation,
        }
    ));
}
