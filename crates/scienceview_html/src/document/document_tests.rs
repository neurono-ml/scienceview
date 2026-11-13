//! Tests for the document emitter (spec `render-html-backend`).

use super::SVHtmlDocument;
use super::html_document::{SV_HTML_CAPTION_ELEMENT_ID, SV_HTML_DATA_ELEMENT_ID};
use crate::payload::SVHtmlDataPayload;
use crate::runtime::SV_HTML_RUNTIME;
use scienceview_core::sv_data::SVColumnView;

fn document(title: &str) -> SVHtmlDocument {
    let values = [1.0, 2.0, 3.0];
    let payload = SVHtmlDataPayload::new(SVColumnView::Dense(&values), "<svg></svg>");
    SVHtmlDocument::new(payload).with_title(title)
}

#[test]
fn the_document_embeds_every_resource_it_needs() {
    let html = document("Offline").to_html();

    assert!(
        html.starts_with("<!DOCTYPE html>\n<html lang=\"en\">"),
        "{html}"
    );
    assert!(html.trim_end().ends_with("</html>"));
    assert!(html.contains("<meta charset=\"utf-8\">"));
    assert!(html.contains("<style>\n.scienceview-figure{"));
    assert!(html.contains("</style>"));
    assert!(html.contains("<svg></svg>"), "the inline SVG is present");
    assert!(html.contains(SV_HTML_RUNTIME), "the runtime is inlined");
    assert!(!html.contains("<link "), "no external stylesheet: {html}");
    assert!(!html.contains("@import"), "no imported stylesheet: {html}");
    assert!(
        !html.contains(" src=\""),
        "no external script or image: {html}"
    );
    assert!(!html.contains("http://"), "no network resource: {html}");
    assert!(!html.contains("https://"), "no network resource: {html}");
}

#[test]
fn the_document_carries_accessible_labels() {
    let html = document("Signal & noise").to_html();

    assert!(html.contains("class=\"scienceview-figure\""));
    assert!(html.contains(&format!("aria-labelledby=\"{SV_HTML_CAPTION_ELEMENT_ID}\"")));
    assert!(html.contains(&format!("id=\"{SV_HTML_CAPTION_ELEMENT_ID}\"")));
    assert!(html.contains("<figcaption"), "{html}");
    assert!(html.contains(">Signal &amp; noise</figcaption>"), "{html}");
    assert!(html.contains("<title>Signal &amp; noise</title>"), "{html}");
}

#[test]
fn the_payload_block_is_observable_and_named() {
    let html = document("Payload").to_html();

    assert!(html.contains(&format!("id=\"{SV_HTML_DATA_ELEMENT_ID}\"")));
    assert!(html.contains("data-scienceview-payload=\"true\""));
    assert!(html.contains("type=\"application/json\""));
    assert!(html.contains("data-scienceview-figure=\"true\""));
    assert!(html.contains("data-scienceview-canvas=\"true\""));
}

#[test]
fn a_title_cannot_break_out_of_the_document() {
    let html = document("</title><script>alert(1)</script>").to_html();

    assert!(!html.contains("<script>alert(1)</script>"), "{html}");
    assert!(
        html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"),
        "{html}"
    );
}

#[test]
fn the_data_block_is_exactly_the_payload_element() {
    let source = document("Payload");
    let block = source.data_block();
    let html = source.to_html();

    assert!(block.starts_with("<script type=\"application/json\""));
    assert!(block.ends_with("</script>"));
    assert!(html.contains(&block));
    assert!(
        !block.contains("<html"),
        "the update fragment is not a page"
    );
    assert!(
        !block.contains("<style"),
        "the update fragment carries no stylesheet"
    );
}

#[test]
fn a_custom_stylesheet_replaces_the_default_emission() {
    let html = document("Custom")
        .with_stylesheet(".host .scienceview-figure{--sv-colour-axis:red}")
        .to_html();

    assert!(html.contains(".host .scienceview-figure{--sv-colour-axis:red}"));
    assert!(!html.contains("--sv-colour-background:#"), "{html}");
}
