//! The document emitter and its stylesheet.

mod html_document;
mod serialization;
mod stylesheet;

pub use html_document::SV_HTML_CAPTION_ELEMENT_ID;
pub use html_document::SV_HTML_DATA_ELEMENT_ID;
pub use html_document::SVHtmlDocument;
pub use stylesheet::sv_html_stylesheet;
pub use stylesheet::sv_stylesheet_for_tokens;

#[cfg(test)]
#[path = "document_tests.rs"]
mod document_tests;
#[cfg(test)]
#[path = "stylesheet_tests.rs"]
mod stylesheet_tests;
