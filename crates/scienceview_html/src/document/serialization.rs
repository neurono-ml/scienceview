//! Serializing the document to HTML text.

use super::SVHtmlDocument;
use super::html_document::{SV_HTML_CAPTION_ELEMENT_ID, SV_RUNTIME_MARKUP, sv_data_block};

/// Serializes a document with a fixed element order and no external resource.
pub(super) fn sv_serialize_document(source: &SVHtmlDocument) -> String {
    let title = source.title_markup();
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("<meta name=\"generator\" content=\"scienceview\">\n");
    html.push_str("<title>");
    html.push_str(&title);
    html.push_str("</title>\n<style>\n");
    html.push_str(source.stylesheet_markup());
    html.push_str("</style>\n</head>\n<body>\n");

    html.push_str("<figure class=\"scienceview-figure\" data-scienceview-figure=\"true\" ");
    html.push_str("aria-labelledby=\"");
    html.push_str(SV_HTML_CAPTION_ELEMENT_ID);
    html.push_str("\">\n<div class=\"scienceview-figure__canvas\" ");
    html.push_str("data-scienceview-canvas=\"true\">\n");
    html.push_str(source.svg_markup());
    html.push_str("</div>\n<figcaption class=\"scienceview-figure__caption\" id=\"");
    html.push_str(SV_HTML_CAPTION_ELEMENT_ID);
    html.push_str("\">");
    html.push_str(&title);
    html.push_str("</figcaption>\n");
    html.push_str(&sv_data_block(source.payload()));
    html.push_str("\n</figure>\n<script>\n");
    html.push_str(SV_RUNTIME_MARKUP);
    html.push_str("</script>\n</body>\n</html>\n");

    html
}
