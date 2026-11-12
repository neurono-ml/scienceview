//! Tests for the document surface, its metadata and its escaping.

use super::SVSvgDocument;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_scale::SVColour;

#[test]
fn a_document_carries_the_viewport_in_width_height_and_view_box() {
    let document = SVSvgDocument::new(SVViewport::new(640.0, 400.0)).to_svg();

    assert!(document.contains("width=\"640\" height=\"400\" viewBox=\"0 0 640 400\""));
}

#[test]
fn every_document_carries_its_accessibility_metadata() {
    let document = SVSvgDocument::new(SVViewport::new(100.0, 50.0))
        .with_title("Weights")
        .with_description("A scatter of weights")
        .to_svg();

    assert!(document.contains("role=\"img\""));
    assert!(document.contains("aria-label=\"Weights\""));
    assert!(document.contains("<title>Weights</title>"));
    assert!(document.contains("<desc>A scatter of weights</desc>"));
}

#[test]
fn the_accessibility_label_falls_back_to_the_title_and_then_to_a_generic_name() {
    let titled = SVSvgDocument::new(SVViewport::new(10.0, 10.0)).with_title("Titled");
    assert_eq!(titled.accessibility_label(), "Titled");

    let untitled = SVSvgDocument::new(SVViewport::new(10.0, 10.0));
    assert_eq!(untitled.accessibility_label(), "scienceview figure");
}

#[test]
fn the_background_is_the_first_painted_element() {
    let document = SVSvgDocument::new(SVViewport::new(20.0, 10.0))
        .with_background(SVColour::new(255, 255, 255, 255))
        .to_svg();

    let background = document
        .find("<rect")
        .expect("the background rectangle exists");
    let desc = document.find("</desc>").expect("the description exists");
    assert!(desc < background, "the background follows the metadata");
    assert!(document.contains("fill=\"#ffffff\""));
}

#[test]
fn titles_and_descriptions_are_escaped() {
    let document = SVSvgDocument::new(SVViewport::new(10.0, 10.0))
        .with_title("a < b & \"c\" 'd' > e")
        .with_description("x < y")
        .to_svg();

    assert!(document.contains("<title>a &lt; b &amp; &quot;c&quot; &apos;d&apos; &gt; e</title>"));
    assert!(document.contains("<desc>x &lt; y</desc>"));
    assert!(!document.contains("a < b"));
}

#[test]
fn an_empty_element_is_never_pushed() {
    let mut document = SVSvgDocument::new(SVViewport::new(10.0, 10.0));
    document.push_element("");
    document.push_element("<rect x=\"0\" y=\"0\" width=\"1\" height=\"1\"/>");

    assert_eq!(document.number_of_elements(), 1);
}

#[test]
fn serializing_twice_is_byte_identical() {
    let document = SVSvgDocument::new(SVViewport::new(320.0, 200.0))
        .with_title("Stable")
        .with_background(SVColour::new(1, 2, 3, 255));

    assert_eq!(document.to_svg(), document.to_svg());
    assert_eq!(document.to_bytes(), document.to_svg().into_bytes());
}
