//! Golden document: the complete bytes of a small figure are checked in, so a
//! change to any emitted element is visible in review.

use crate::SVHtmlBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;

const GOLDEN_DOCUMENT: &str = include_str!("golden/empty_figure.html");

fn render_empty_figure() -> String {
    let output = SVHtmlBackend::new()
        .render(&SVRenderRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&[]),
            title: Some("Golden figure"),
        })
        .expect("the backend renders an empty figure");

    String::from_utf8(output.bytes).expect("the document is UTF-8")
}

#[test]
fn an_empty_figure_matches_the_golden_document() {
    let rendered = render_empty_figure();

    assert_eq!(rendered, GOLDEN_DOCUMENT);
}

#[test]
#[ignore = "regenerates the checked-in golden document"]
fn write_golden_document() {
    let destination = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/crate_tests/golden/empty_figure.html");
    std::fs::create_dir_all(destination.parent().expect("the folder exists"))
        .expect("the golden folder is created");
    std::fs::write(destination, render_empty_figure()).expect("the golden document is written");
}
