//! The self-contained HTML document.
//!
//! A document owns notation only: the SVG markup and the payload arrive
//! already rendered by the Rust model, and the stylesheet is a compiled token
//! snapshot. Serializing twice yields identical bytes, which is what lets the
//! documentation embed the tool's own output without drift.

use crate::payload::SVHtmlDataPayload;
use crate::runtime::SV_HTML_RUNTIME;
use scienceview_svg::sv_escape_text;

use super::stylesheet::sv_html_stylesheet;

/// The id of the payload element the runtime observes.
pub const SV_HTML_DATA_ELEMENT_ID: &str = "scienceview-data";
/// The id of the caption that labels the figure.
pub const SV_HTML_CAPTION_ELEMENT_ID: &str = "scienceview-caption";

/// A complete, self-contained HTML page around one figure.
#[derive(Clone, Debug, PartialEq)]
pub struct SVHtmlDocument {
    title: String,
    payload: SVHtmlDataPayload,
    stylesheet: String,
}

impl SVHtmlDocument {
    /// Starts a document around an already captured payload.
    pub fn new(payload: SVHtmlDataPayload) -> Self {
        Self {
            title: "scienceview figure".to_string(),
            payload,
            stylesheet: sv_html_stylesheet(),
        }
    }

    /// Sets the page title and the visible caption.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Replaces the emitted stylesheet.
    ///
    /// A caller that compiled a custom theme passes its own stylesheet; the
    /// host page can still override individual custom properties.
    #[must_use]
    pub fn with_stylesheet(mut self, stylesheet: impl Into<String>) -> Self {
        self.stylesheet = stylesheet.into();
        self
    }

    /// The page title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The payload the document embeds.
    pub fn payload(&self) -> &SVHtmlDataPayload {
        &self.payload
    }

    /// Serializes the document to HTML text.
    pub fn to_html(&self) -> String {
        super::serialization::sv_serialize_document(self)
    }

    /// Serializes the document to bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_html().into_bytes()
    }

    /// Serializes only the payload block.
    ///
    /// This is what `update` returns: replacing exactly this element in a live
    /// page changes the data and nothing else, so the document keeps its view
    /// state and its host page.
    pub fn data_block(&self) -> String {
        sv_data_block(&self.payload)
    }

    pub(super) fn title_markup(&self) -> String {
        sv_escape_text(&self.title)
    }

    pub(super) fn stylesheet_markup(&self) -> &str {
        &self.stylesheet
    }

    pub(super) fn svg_markup(&self) -> &str {
        self.payload.markup()
    }
}

/// Serializes the payload element, which the runtime observes and applies.
pub(super) fn sv_data_block(payload: &SVHtmlDataPayload) -> String {
    format!(
        "<script type=\"application/json\" data-scienceview-payload=\"true\" \
id=\"{SV_HTML_DATA_ELEMENT_ID}\">{}</script>",
        payload.to_json()
    )
}

/// The inlined runtime source, exposed so tests can prove it is embedded.
pub(super) const SV_RUNTIME_MARKUP: &str = SV_HTML_RUNTIME;
