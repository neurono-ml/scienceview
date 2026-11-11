//! Primitives are backend-agnostic geometry.

use super::{SVDecorationPrimitive, SVPoint, SVRectanglePrimitive, SVStroke, SVTextAnchor};
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_scale::SVColour;

fn sv_colour() -> SVColour {
    SVColour::new(9, 105, 218, 255)
}

#[test]
fn a_line_keeps_its_geometry_and_stroke() {
    let line = SVDecorationPrimitive::line(
        SVPoint::new(1.0, 2.0),
        SVPoint::new(3.0, 4.0),
        SVStroke::new(sv_colour(), 2.0),
    );

    assert_eq!(line.kind(), "Line");
    let SVDecorationPrimitive::Line { start, end, stroke } = line else {
        panic!("the constructor built a line");
    };
    assert_eq!(start, SVPoint::new(1.0, 2.0));
    assert_eq!(end, SVPoint::new(3.0, 4.0));
    assert_eq!(stroke.colour, sv_colour());
    assert_eq!(stroke.width, 2.0);
}

#[test]
fn a_filled_rectangle_has_a_fill_and_no_stroke() {
    let rectangle =
        SVRectanglePrimitive::filled(SVResolvedRectangle::new(0.0, 0.0, 10.0, 20.0), sv_colour());

    assert_eq!(rectangle.fill, Some(sv_colour()));
    assert!(rectangle.stroke.is_none());
}

#[test]
fn an_outlined_rectangle_has_a_stroke_and_no_fill() {
    let rectangle = SVRectanglePrimitive::outlined(
        SVResolvedRectangle::new(0.0, 0.0, 10.0, 20.0),
        SVStroke::new(sv_colour(), 1.0),
    );

    assert!(rectangle.fill.is_none());
    assert_eq!(rectangle.stroke.map(|stroke| stroke.width), Some(1.0));
}

#[test]
fn a_text_primitive_is_horizontal_by_default() {
    let text = SVDecorationPrimitive::text(
        SVPoint::new(0.0, 0.0),
        "label",
        10.0,
        400,
        sv_colour(),
        SVTextAnchor::Middle,
    );

    assert_eq!(text.kind(), "Text");
    let SVDecorationPrimitive::Text {
        rotation_degrees,
        anchor,
        ..
    } = text
    else {
        panic!("the constructor built a text primitive");
    };
    assert_eq!(rotation_degrees, 0.0);
    assert_eq!(anchor, SVTextAnchor::Middle);
}

#[test]
fn a_rotated_text_carries_its_rotation() {
    let text = SVDecorationPrimitive::rotated_text(
        SVPoint::new(0.0, 0.0),
        "axis title",
        10.0,
        400,
        sv_colour(),
        SVTextAnchor::Middle,
        -90.0,
    );

    let SVDecorationPrimitive::Text {
        rotation_degrees, ..
    } = text
    else {
        panic!("the constructor built a text primitive");
    };
    assert_eq!(rotation_degrees, -90.0);
}
