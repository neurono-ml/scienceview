//! Render dispatch, typed errors and the serialized figure.

use super::test_support::{sv_build_default, sv_builder, sv_matrix_source, sv_scene_to_svg};
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;

#[test]
fn the_serialized_svg_is_self_contained_and_deterministic() {
    let source = sv_matrix_source();
    let first = sv_scene_to_svg(&sv_build_default(&source).compose().expect("composes"));
    let second = sv_scene_to_svg(&sv_build_default(&source).compose().expect("composes"));

    assert_eq!(first, second, "the figure is deterministic");
    assert!(first.contains("<svg"));
    assert!(first.contains("<path"));
    assert!(!first.contains("href=\"http"));
}

#[test]
fn a_backend_without_a_recipe_serializer_is_a_typed_error() {
    let source = sv_matrix_source();
    let plot = sv_builder(&source)
        .backend(SVBackendKind::Html)
        .build()
        .expect("the configuration is complete");
    let result = plot.render();

    assert!(matches!(
        result,
        Err(SVError::UnsupportedBackendCapability { .. })
    ));
}

#[test]
fn an_unavailable_backend_feature_is_reported_not_degraded() {
    let source = sv_matrix_source();
    let plot = sv_builder(&source)
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");
    let result = plot.render();

    #[cfg(not(feature = "render-native"))]
    assert!(matches!(result, Err(SVError::BackendUnavailable { .. })));
    #[cfg(feature = "render-native")]
    assert!(result.is_ok());
}

#[cfg(feature = "render-svg")]
#[test]
fn the_svg_render_path_matches_the_composed_document() {
    let source = sv_matrix_source();
    let plot = sv_build_default(&source);
    let composition = plot.compose().expect("the chart composes");
    let rendered = plot.render().expect("the chart renders");
    let expected = sv_scene_to_svg(&composition);

    assert_eq!(rendered.backend_kind, SVBackendKind::Svg);
    assert_eq!(String::from_utf8(rendered.bytes).expect("utf8"), expected);
}

#[cfg(all(test, feature = "render-native"))]
#[test]
fn the_native_frame_matches_the_svg_pixels_dimensions() {
    let source = sv_matrix_source();
    let plot = sv_builder(&source)
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");
    let composition = plot.compose().expect("the chart composes");
    let frame = composition.to_native_frame().expect("the frame rasterizes");

    assert_eq!(frame.width() as f64, composition.viewport().width());
    assert_eq!(frame.height() as f64, composition.viewport().height());
    assert!(frame.pixels().chunks(4).any(|pixel| pixel[3] > 0));
}

#[test]
fn save_writes_the_svg_document_on_the_svg_backend() {
    #[cfg(feature = "render-svg")]
    {
        let source = sv_matrix_source();
        let plot = sv_build_default(&source);
        let directory = std::env::temp_dir().join("scienceview-contour-save");
        std::fs::create_dir_all(&directory).expect("the temporary directory is creatable");
        let destination = directory.join("figure.svg");
        plot.save(&destination).expect("the figure saves");
        let saved = std::fs::read_to_string(&destination).expect("the document is readable");
        assert!(saved.contains("<path"));
    }

    #[cfg(not(feature = "render-svg"))]
    {
        let source = sv_matrix_source();
        let plot = sv_build_default(&source);
        let result = plot.save(std::path::Path::new("/tmp/unused.svg"));
        assert!(matches!(result, Err(SVError::BackendUnavailable { .. })));
    }
}
