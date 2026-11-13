//! The backend dispatch: a configured kind is resolved, never ignored.

use super::SVHistogramPlot;
#[cfg(feature = "render-svg")]
use super::SVHistogramRenderOutput;
use super::test_support::{sv_build_default, sv_dense_source};
use crate::sv_backend::{SVBackendCapability, SVBackendKind};
#[cfg(feature = "render-svg")]
use crate::sv_data::{SVColumnRole, SVDataTableBuilder};
use crate::sv_error::SVError;
#[cfg(feature = "render-svg")]
use crate::sv_stat::SVNormalisation;

fn sv_build_with_backend(
    source: &dyn crate::sv_data::SVDataSource,
    backend_kind: SVBackendKind,
) -> SVHistogramPlot<'_> {
    SVHistogramPlot::builder()
        .data_source(source)
        .value("value")
        .bin_count(3)
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
    assert!(document.contains("viewBox=\"0 0 640 400\""));
    assert!(document.contains("<rect"));
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
    assert_eq!(frame.width(), 640);
    assert_eq!(frame.height(), 400);
    let painted = frame
        .pixels()
        .chunks_exact(4)
        .filter(|pixel| pixel[3] > 0)
        .count();
    assert!(painted > 0, "the native frame carries drawn pixels");
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

#[cfg(not(feature = "render-native"))]
#[test]
fn a_compiled_out_native_backend_reports_itself_unavailable() {
    let source = sv_dense_source();
    let error = sv_build_with_backend(&source, SVBackendKind::Native)
        .render()
        .expect_err("the native feature is disabled");

    assert_eq!(
        error,
        SVError::BackendUnavailable {
            backend_kind: SVBackendKind::Native,
        }
    );
}

#[test]
fn a_kind_without_a_recipe_serializer_fails_with_a_typed_error() {
    let source = sv_dense_source();
    let error = sv_build_with_backend(&source, SVBackendKind::Html)
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

#[cfg(feature = "render-svg")]
#[test]
fn the_public_svg_method_matches_the_checked_in_figure() {
    // Keep this data identical to `examples/histogram_plot.rs`: the test
    // proves the figure under `docs/figures/` is exactly what the public
    // `to_svg` method produces.
    const SEPAL_WIDTH: [f64; 36] = [
        2.9, 3.0, 3.1, 3.1, 3.2, 3.2, 3.3, 3.4, 3.4, 3.5, 3.6, 3.7, 2.5, 2.6, 2.7, 2.8, 2.8, 2.9,
        3.0, 3.0, 3.1, 3.2, 3.3, 3.4, 2.6, 2.8, 2.9, 3.0, 3.0, 3.1, 3.1, 3.2, 3.2, 3.3, 3.4, 3.5,
    ];
    const SPECIES: [f64; 36] = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
    ];

    let table = SVDataTableBuilder::new()
        .with_column("sepal_width", SVColumnRole::X, &SEPAL_WIDTH)
        .with_column("species", SVColumnRole::Grouping, &SPECIES)
        .build()
        .expect("the example table is well formed");

    let scene = SVHistogramPlot::builder()
        .data_source(&table)
        .value("sepal_width")
        .series("species")
        .series_labels(&["setosa", "versicolor", "virginica"])
        .bin_count(10)
        .normalise(SVNormalisation::Probability)
        .cumulative(false)
        .multiple(super::SVHistogramMultiple::Stack)
        .shrink(0.95)
        .title("Sepal width by species")
        .x_label("sepal width (cm)")
        .y_label("probability")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the example configuration is complete")
        .compose()
        .expect("the example figure composes");

    let generated = scene.to_svg().expect("the SVG serializer runs");
    let checked_in = include_str!("../../../../docs/figures/histogram_plot.svg");
    assert_eq!(
        generated, checked_in,
        "docs/figures/histogram_plot.svg drifted from the public to_svg output"
    );

    let rendered = sv_build_default(&sv_dense_source())
        .render()
        .expect("the dispatch path renders");
    assert!(matches!(rendered, SVHistogramRenderOutput::Svg(_)));
}
