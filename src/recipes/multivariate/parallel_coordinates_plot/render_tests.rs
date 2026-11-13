//! The public render path: SVG, native and the checked-in figure contract.

#[cfg(feature = "render-svg")]
use super::test_support::sv_build_default;
#[cfg(any(feature = "render-svg", feature = "render-native"))]
use super::test_support::sv_dense_source;

#[cfg(feature = "render-svg")]
#[test]
fn the_svg_render_matches_the_scene_serializer() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    let output = plot.compose().expect("the chart composes");
    let rendered = plot.render().expect("the SVG backend renders");
    assert_eq!(
        rendered.bytes,
        output.to_svg().expect("the scene serializes").as_bytes()
    );
}

#[cfg(feature = "render-native")]
#[test]
fn the_native_render_produces_a_frame() {
    let source = sv_dense_source();
    let plot = super::SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second", "third"])
        .backend(crate::sv_backend::SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");
    let output = plot.render().expect("the native backend renders");
    assert!(!output.bytes.is_empty());
}

#[cfg(feature = "render-svg")]
#[test]
fn the_documented_figure_exists_and_is_an_svg_document() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/figures/parallel_coordinates_plot.svg"
    );
    let document = std::fs::read_to_string(path)
        .expect("the checked-in figure exists; run the example to generate it");
    assert!(document.starts_with("<svg"));
    assert!(document.contains("parallel axes"));
}
