//! Determinism and the large-data sanity check.

use crate::backend::SVSvgBackend;
use crate::document::SVSvgDocument;
use scienceview_core::sv_backend::{SVBackendKind, SVRenderBackend, SVRenderRequest};
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;

fn request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Svg,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: Some("Ten thousand points"),
    }
}

#[test]
fn rendering_the_same_figure_twice_is_byte_identical() {
    let values: Vec<f64> = (0..1_000)
        .map(|index| (index as f64 / 25.0).sin())
        .collect();
    let backend = SVSvgBackend::new();

    let first = backend.render(&request(&values)).expect("render");
    let second = backend.render(&request(&values)).expect("render again");

    assert_eq!(first, second);
}

#[test]
fn ten_thousand_points_render_one_path_quickly() {
    let values: Vec<f64> = (0..10_000)
        .map(|index| (index as f64 / 50.0).sin())
        .collect();
    let backend = SVSvgBackend::new();

    let started = std::time::Instant::now();
    let output = backend.render(&request(&values)).expect("render");
    let elapsed = started.elapsed();

    let document = String::from_utf8(output.bytes).expect("UTF-8");
    assert_eq!(document.matches("<path ").count(), 1);
    assert!(
        elapsed < std::time::Duration::from_secs(2),
        "the render took {elapsed:?}"
    );
    assert!(
        document.len() < 10_000 * 160,
        "the document grew to {} bytes",
        document.len()
    );
}

#[test]
fn a_document_is_identical_whether_built_twice() {
    let build = || {
        SVSvgDocument::new(SVViewport::new(100.0, 80.0))
            .with_title("Stable")
            .with_background(scienceview_core::sv_scale::SVColour::new(
                250, 250, 250, 255,
            ))
            .to_svg()
    };

    assert_eq!(build(), build());
}
