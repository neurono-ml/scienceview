//! The backend dispatch: a configured kind is resolved, never ignored.

use super::SVRugPlot;
use super::test_support::{sv_build_default, sv_build_with_source, sv_dense_source};
use crate::sv_backend::{SVBackendCapability, SVBackendKind};
use crate::sv_error::SVError;

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
    assert!(document.contains("<path"));
}

#[cfg(feature = "render-native")]
#[test]
fn the_native_kind_rasterizes_the_native_frame() {
    let source = sv_dense_source();
    let plot = sv_build_with_source(&source, SVBackendKind::Native);
    let output = plot.render().expect("the native backend renders");

    assert_eq!(output.backend_kind(), SVBackendKind::Native);
    let frame = output
        .as_native_frame()
        .expect("the output is a native frame");
    assert_eq!(frame.width(), 640);
    assert_eq!(frame.height(), 430);
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
    let scene = sv_build_with_source(&source, SVBackendKind::Native)
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
    let error = sv_build_with_source(&source, SVBackendKind::Html)
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

#[cfg(not(feature = "render-native"))]
#[test]
fn a_compiled_out_native_backend_reports_itself_unavailable() {
    let source = sv_dense_source();
    let error = sv_build_with_source(&source, SVBackendKind::Native)
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
fn the_build_carries_the_configured_kind_and_execution_mode() {
    let source = sv_dense_source();
    let plot = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .backend(SVBackendKind::Tui)
        .execution_mode(crate::sv_figure::SVExecutionMode::Automatic)
        .build()
        .expect("the configuration is complete");

    assert_eq!(plot.backend_kind(), SVBackendKind::Tui);
    assert_eq!(
        plot.execution_mode(),
        crate::sv_figure::SVExecutionMode::Automatic
    );
}

#[cfg(feature = "render-svg")]
#[test]
fn the_public_svg_method_matches_the_checked_in_figure() {
    // Keep this data identical to `examples/rug_plot.rs`: the test proves the
    // figure under `docs/figures/` is exactly what the public `to_svg` method
    // of this test binary produces.
    const BODY_MASS: [f64; 40] = [
        36.2, 37.1, 38.4, 39.0, 39.6, 40.1, 40.4, 40.9, 41.2, 41.8, 42.0, 42.3, 42.9, 43.1, 43.6,
        44.0, 44.2, 44.7, 45.0, 45.4, 45.9, 46.2, 46.5, 47.0, 47.4, 47.9, 48.2, 48.7, 49.1, 49.5,
        50.0, 50.6, 51.1, 51.8, 52.4, 53.0, 53.7, 54.4, 55.2, 56.1,
    ];
    const SPECIES: [f64; 40] = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
    ];

    let table = crate::sv_data::SVDataTableBuilder::new()
        .with_column("body_mass", crate::sv_data::SVColumnRole::X, &BODY_MASS)
        .with_column("species", crate::sv_data::SVColumnRole::Grouping, &SPECIES)
        .build()
        .expect("the example table is well formed");

    let scene = SVRugPlot::builder()
        .data_source(&table)
        .value("body_mass")
        .series("species")
        .series_labels(&["Adelie", "Gentoo"])
        .height(0.08)
        .alpha(0.85)
        .title("Body mass of penguins")
        .value_label("body mass (g)")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the example configuration is complete")
        .compose()
        .expect("the example figure composes");

    let generated = scene.to_svg().expect("the SVG serializer runs");
    let checked_in = include_str!("../../../../docs/figures/rug_plot.svg");
    assert_eq!(
        generated, checked_in,
        "docs/figures/rug_plot.svg drifted from the public to_svg output"
    );
}
