//! The backend dispatch: a configured kind is resolved, never ignored.

use super::SVImagePlot;
#[cfg(feature = "render-svg")]
use super::SVImageRenderOutput;
use super::test_support::{SVTestSource, dimensions, sv_build_default};
use crate::sv_backend::{SVBackendCapability, SVBackendKind};
use crate::sv_error::SVError;

fn sv_build_with_backend(
    source: &dyn crate::sv_data::SVDataSource,
    backend_kind: SVBackendKind,
) -> SVImagePlot<'_> {
    SVImagePlot::builder()
        .data_source(source)
        .raster(super::SVImageSource::intensity("field", dimensions()))
        .backend(backend_kind)
        .build()
        .expect("the configuration is complete")
}

#[cfg(feature = "render-svg")]
#[test]
fn the_svg_kind_serializes_a_self_contained_document() {
    let source = SVTestSource::dense();
    let output = sv_build_default(&source)
        .render()
        .expect("the SVG backend renders");

    assert_eq!(output.backend_kind(), SVBackendKind::Svg);
    let document = std::str::from_utf8(output.as_svg().expect("the output is SVG"))
        .expect("the SVG document is text");
    assert!(document.starts_with("<svg"));
    assert!(document.contains("viewBox=\"0 0 680 512\""));
    assert!(document.contains("<image "));
    assert!(document.contains("data:image/png;base64,"));
    assert!(document.contains("image-rendering=\"pixelated\""));
}

#[cfg(feature = "render-svg")]
#[test]
fn bilinear_sampling_reaches_the_svg_document() {
    let source = SVTestSource::dense();
    let document = sv_build_default(&source)
        .compose()
        .expect("the image composes")
        .to_svg()
        .expect("the scene serializes");
    let bilinear = SVImagePlot::builder()
        .data_source(&source)
        .raster(super::SVImageSource::intensity("field", dimensions()))
        .interpolation(super::SVImageInterpolation::Bilinear)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the image composes")
        .to_svg()
        .expect("the scene serializes");

    assert!(document.contains("image-rendering=\"pixelated\""));
    assert!(bilinear.contains("image-rendering=\"auto\""));
}

#[cfg(feature = "render-svg")]
#[test]
fn two_renders_of_the_same_configuration_are_byte_identical() {
    let source = SVTestSource::dense();
    let plot = sv_build_default(&source);
    let first = plot.render().expect("the first render");
    let second = plot.render().expect("the second render");

    assert_eq!(first, second);
    assert!(matches!(first, SVImageRenderOutput::Svg(_)));
}

#[cfg(feature = "render-native")]
#[test]
fn the_native_kind_rasterizes_the_native_frame() {
    let source = SVTestSource::dense();
    let plot = sv_build_with_backend(&source, SVBackendKind::Native);
    let output = plot.render().expect("the native backend renders");

    assert_eq!(output.backend_kind(), SVBackendKind::Native);
    let frame = output
        .as_native_frame()
        .expect("the output is a native frame");
    assert_eq!(frame.width(), 680);
    assert_eq!(frame.height(), 512);
    let painted = frame
        .pixels()
        .chunks_exact(4)
        .filter(|pixel| pixel[3] > 0)
        .count();
    assert!(painted > 0, "the native frame carries drawn pixels");
}

#[test]
fn a_kind_without_a_recipe_serializer_reports_a_capability_error() {
    let source = SVTestSource::dense();
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

#[cfg(all(feature = "render-svg", feature = "render-native"))]
#[test]
fn to_svg_rejects_a_scene_composed_for_another_backend() {
    let source = SVTestSource::dense();
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

#[test]
fn the_built_plot_reports_its_configuration() {
    let source = SVTestSource::dense();
    let plot = sv_build_default(&source);

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(
        plot.execution_mode(),
        crate::sv_figure::SVExecutionMode::Automatic
    );
    assert_eq!(plot.dimensions().columns(), 4);
    assert_eq!(plot.point_count(), 12);
    assert!(plot.extent().is_none());
}

#[cfg(not(feature = "render-svg"))]
#[test]
fn a_compiled_out_backend_reports_itself_unavailable() {
    let source = SVTestSource::dense();
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
