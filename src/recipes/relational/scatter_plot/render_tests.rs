//! The optional backend surfaces: SVG and the native rasterizer.

use super::test_support::{sv_build_default, sv_dense_source};

#[cfg(feature = "render-svg")]
#[test]
fn a_scene_serializes_as_a_self_contained_svg_document() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .render()
        .expect("the scatter renders");
    let svg = scene.to_svg();

    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("viewBox=\"0 0 640 400\""));
    assert!(svg.contains("<path"));
}

#[cfg(feature = "render-native")]
#[test]
fn the_same_scene_rasterizes_natively() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .render()
        .expect("the scatter renders");
    let frame = scene
        .to_native_frame()
        .expect("the native rasterizer draws the scene");

    assert_eq!(frame.width(), 640);
    assert_eq!(frame.height(), 400);
    let painted = frame
        .pixels()
        .chunks_exact(4)
        .filter(|pixel| pixel[3] > 0)
        .count();
    assert!(painted > 0, "the native frame carries drawn pixels");
}
