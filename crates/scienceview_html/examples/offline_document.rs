//! Renders one self-contained HTML document and prints it to standard output.
//!
//! Run with `cargo run -p scienceview_html --example offline_document` and
//! redirect the output to a file; the page needs no network access. The
//! payload and the emitter are the public surface a recipe uses when it
//! composes its own document instead of going through the figure builder.

use scienceview_core::sv_backend::{SVBackendKind, SVRenderBackend, SVRenderRequest};
use scienceview_core::sv_data::{SVColumnView, SVRepresentation};
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_html::{SVHtmlDataPayload, SVHtmlDocument};
use scienceview_svg::{SVSvgBackend, sv_escape_text};

fn main() {
    let values: [f64; 8] = [0.0, 1.0, 4.0, 9.0, 16.0, 9.0, 4.0, 1.0];
    let view = SVColumnView::Dense(&values);
    let svg = SVSvgBackend::new()
        .render(&SVRenderRequest {
            backend_kind: SVBackendKind::Svg,
            execution_mode: SVExecutionMode::Automatic,
            view,
            title: Some("Offline document"),
        })
        .expect("the SVG renderer accepts the column");
    let markup = String::from_utf8(svg.bytes).expect("the SVG markup is UTF-8");
    let payload = SVHtmlDataPayload::new(view, markup);

    assert_eq!(payload.representation(), SVRepresentation::Dense);
    let document = SVHtmlDocument::new(payload).with_title("Offline document");
    assert!(
        document
            .to_html()
            .contains(&sv_escape_text("Offline document"))
    );

    print!("{}", document.to_html());
}
