//! The backend dispatch: a configured kind is resolved, never ignored.
//!
//! The recipe has two serializer paths — the SVG document and the native
//! raster frame. A backend without a recipe serializer reports a typed
//! capability error; a backend whose crate feature is disabled reports the
//! backend as unavailable. The tests cover both paths, both failures and the
//! serializer-mismatch guard on the scene itself.

use super::SVScatterPlot;
#[cfg(feature = "render-svg")]
use super::SVScatterRenderOutput;
use super::test_support::{sv_build_default, sv_dense_source};
use crate::sv_backend::{SVBackendCapability, SVBackendKind};
use crate::sv_error::SVError;

fn sv_build_with_backend(
    source: &dyn crate::sv_data::SVDataSource,
    backend_kind: SVBackendKind,
) -> SVScatterPlot<'_> {
    SVScatterPlot::builder()
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
    let source = sv_dense_source();
    let output = sv_build_default(&source)
        .render()
        .expect("the SVG backend renders");

    assert_eq!(output.backend_kind(), SVBackendKind::Svg);
    let document = std::str::from_utf8(output.as_svg().expect("the output is SVG")).unwrap();
    assert!(document.starts_with("<svg"));
    assert!(document.contains("viewBox=\"0 0 640 400\""));
    assert!(document.contains("<path"));
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
    // Keep this data identical to `examples/scatter_plot.rs`: the test proves
    // the figure under `docs/figures/` is exactly what the public `to_svg`
    // method of this test binary produces.
    const SEPAL_WIDTH: [f64; 18] = [
        1.4, 1.7, 1.9, 1.5, 2.0, 1.6, 2.6, 3.0, 2.8, 3.2, 2.7, 3.4, 3.1, 3.5, 3.3, 3.7, 3.9, 4.4,
    ];
    const PETAL_LENGTH: [f64; 18] = [
        2.6, 3.1, 3.4, 2.9, 3.8, 3.0, 4.3, 5.1, 4.7, 5.6, 4.5, 6.0, 5.4, 6.3, 5.8, 6.7, 7.1, 7.6,
    ];
    const SPECIES: [f64; 18] = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
    ];
    const BODY_MASS: [f64; 18] = [
        1.0, 1.4, 1.2, 0.9, 1.6, 1.1, 2.2, 2.8, 2.4, 3.0, 2.1, 3.2, 2.9, 3.4, 3.1, 3.7, 3.9, 4.2,
    ];

    let table = crate::sv_data::SVDataTableBuilder::new()
        .with_column("sepal_width", crate::sv_data::SVColumnRole::X, &SEPAL_WIDTH)
        .with_column(
            "petal_length",
            crate::sv_data::SVColumnRole::Y,
            &PETAL_LENGTH,
        )
        .with_column("species", crate::sv_data::SVColumnRole::Grouping, &SPECIES)
        .with_column(
            "body_mass",
            crate::sv_data::SVColumnRole::Weight,
            &BODY_MASS,
        )
        .build()
        .expect("the example table is well formed");

    let scene = SVScatterPlot::builder()
        .data_source(&table)
        .x("sepal_width")
        .y("petal_length")
        .series("species")
        .series_labels(&["setosa", "versicolor", "virginica"])
        .size("body_mass")
        .alpha(0.85)
        .title("Sepal width against petal length")
        .x_label("sepal width")
        .y_label("petal length")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the example configuration is complete")
        .compose()
        .expect("the example figure composes");

    let generated = scene.to_svg().expect("the SVG serializer runs");
    let checked_in = include_str!("../../../../docs/figures/scatter_plot.svg");
    assert_eq!(
        generated, checked_in,
        "docs/figures/scatter_plot.svg drifted from the public to_svg output"
    );

    let rendered = sv_build_default(&sv_dense_source())
        .render()
        .expect("the dispatch path renders");
    assert!(matches!(rendered, SVScatterRenderOutput::Svg(_)));
}
