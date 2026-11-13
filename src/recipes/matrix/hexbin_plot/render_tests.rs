//! The typed backend dispatch of one hexbin render.

use super::SVHexbinPlot;
#[cfg(feature = "render-svg")]
use super::test_support::sv_build_default;
use super::test_support::sv_dense_source;
use crate::sv_backend::{SVBackendCapability, SVBackendKind};
use crate::sv_error::SVError;

fn sv_build_with_backend(
    source: &dyn crate::sv_data::SVDataSource,
    backend_kind: SVBackendKind,
) -> SVHexbinPlot<'_> {
    SVHexbinPlot::builder()
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

    assert_eq!(output.backend_kind, SVBackendKind::Svg);
    let document = std::str::from_utf8(&output.bytes).expect("the output is valid UTF-8");
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

    assert_eq!(output.backend_kind, SVBackendKind::Native);
    assert_eq!(output.bytes.len(), 640 * 400 * 4);
    assert!(output.bytes.chunks_exact(4).any(|pixel| pixel[3] > 0));
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
    // Keep this data identical to `examples/hexbin_plot.rs`: the test proves
    // the figure under `docs/figures/` is exactly what the public `to_svg`
    // method of this test binary produces.
    struct SVLinearCongruential {
        state: u64,
    }
    impl SVLinearCongruential {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }
        fn next_unit(&mut self) -> f64 {
            self.state = self
                .state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((self.state >> 11) as f64) / ((1u64 << 53) as f64)
        }
        fn next_normal(&mut self) -> f64 {
            let first = self.next_unit().max(f64::MIN_POSITIVE);
            let second = self.next_unit();
            (-2.0 * first.ln()).sqrt() * (std::f64::consts::TAU * second).cos()
        }
    }

    let mut generator = SVLinearCongruential::new(0x5EED_0BEE);
    let mut horizontal = Vec::with_capacity(1_200);
    let mut vertical = Vec::with_capacity(1_200);
    for index in 0..1_200 {
        if index % 3 == 0 {
            horizontal.push(2.0 + 0.9 * generator.next_normal());
            vertical.push(2.0 + 0.9 * generator.next_normal());
        } else if index % 3 == 1 {
            horizontal.push(5.5 + 0.7 * generator.next_normal());
            vertical.push(4.8 + 1.1 * generator.next_normal());
        } else {
            horizontal.push(4.0 + 1.3 * generator.next_normal());
            vertical.push(6.5 + 0.6 * generator.next_normal());
        }
    }
    let table = crate::sv_data::SVDataTable::builder()
        .with_column(
            "first measurement",
            crate::sv_data::SVColumnRole::X,
            &horizontal,
        )
        .with_column(
            "second measurement",
            crate::sv_data::SVColumnRole::Y,
            &vertical,
        )
        .build()
        .expect("the example table is well formed");
    let document = SVHexbinPlot::builder()
        .data_source(&table)
        .x("first measurement")
        .y("second measurement")
        .grid_size(28)
        .min_count(2)
        .title("Hexagonal density of two measurements")
        .x_label("first measurement")
        .y_label("second measurement")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the example configuration is complete")
        .compose()
        .expect("the example lattice composes")
        .to_svg()
        .expect("the example scene serializes");
    let checked_in = include_str!("../../../../docs/figures/hexbin_plot.svg");
    assert_eq!(document, checked_in);
}
