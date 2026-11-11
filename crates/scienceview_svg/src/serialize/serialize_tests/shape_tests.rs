//! Shape-level snapshots: geometry, paint and escaping.

use crate::serialize::sv_serialize_mark_primitive;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{
    SVDashPattern, SVDashSequence, SVGeometryCircle, SVGeometryImage, SVGeometryPath,
    SVGeometryRectangle, SVGeometryText, SVImageInterpolation, SVLineCap, SVLineJoin, SVMarkPaint,
    SVMarkPoint, SVMarkPrimitive, SVMarkShape, SVPathCommand, SVTextAlign, SVTextBaseline,
};
use scienceview_core::sv_scale::SVColour;

const FONT: &str = "system-ui";

fn point(x: f64, y: f64) -> SVMarkPoint {
    SVMarkPoint::new(x, y)
}

fn colour(red: u8, green: u8, blue: u8, alpha: u8) -> SVColour {
    SVColour::new(red, green, blue, alpha)
}

fn serialize(shape: SVMarkShape<'_>, paint: SVMarkPaint) -> String {
    sv_serialize_mark_primitive(&SVMarkPrimitive::new(shape, paint), FONT)
}

#[test]
fn a_filled_circle_serializes_its_geometry_and_fill() {
    let element = serialize(
        SVMarkShape::Circle(SVGeometryCircle::new(point(10.0, 20.0), 3.0)),
        SVMarkPaint::filled(colour(9, 105, 218, 255)),
    );

    assert_eq!(
        element,
        "<circle cx=\"10\" cy=\"20\" r=\"3\" fill=\"#0969da\"/>"
    );
}

#[test]
fn a_stroked_circle_serializes_every_stroke_attribute() {
    let element = serialize(
        SVMarkShape::Circle(SVGeometryCircle::new(point(0.0, 0.0), 1.5)),
        SVMarkPaint::stroked(colour(255, 255, 255, 255), 1.5)
            .with_cap(SVLineCap::Round)
            .with_join(SVLineJoin::Bevel)
            .with_dash(SVDashPattern::Custom(
                SVDashSequence::new(vec![4.0, 2.0]).expect("the sequence is valid"),
            ))
            .with_opacity(0.5),
    );

    assert_eq!(
        element,
        "<circle cx=\"0\" cy=\"0\" r=\"1.5\" fill=\"none\" stroke=\"#ffffff\" stroke-width=\"1.5\" stroke-linecap=\"round\" stroke-linejoin=\"bevel\" stroke-dasharray=\"4 2\" opacity=\"0.5\"/>"
    );
}

#[test]
fn a_rectangle_only_carries_a_corner_radius_when_it_has_one() {
    let square = serialize(
        SVMarkShape::Rectangle(SVGeometryRectangle::new(
            SVResolvedRectangle::new(1.0, 2.0, 3.0, 4.0),
            0.0,
        )),
        SVMarkPaint::filled(colour(255, 0, 0, 255)),
    );
    let rounded = serialize(
        SVMarkShape::Rectangle(SVGeometryRectangle::new(
            SVResolvedRectangle::new(1.0, 2.0, 3.0, 4.0),
            2.0,
        )),
        SVMarkPaint::filled(colour(255, 0, 0, 255)),
    );

    assert_eq!(
        square,
        "<rect x=\"1\" y=\"2\" width=\"3\" height=\"4\" fill=\"#ff0000\"/>"
    );
    assert_eq!(
        rounded,
        "<rect x=\"1\" y=\"2\" width=\"3\" height=\"4\" rx=\"2\" fill=\"#ff0000\"/>"
    );
}

#[test]
fn a_path_maps_every_command_to_one_svg_instruction() {
    let path = SVGeometryPath::new(vec![
        SVPathCommand::MoveTo(point(0.0, 0.0)),
        SVPathCommand::LineTo(point(10.0, 10.0)),
        SVPathCommand::ArcTo(scienceview_core::sv_mark::SVArcTo::new(
            point(20.0, 20.0),
            5.0,
            0.0,
            std::f64::consts::FRAC_PI_2,
        )),
        SVPathCommand::Close,
    ]);
    let element = serialize(
        SVMarkShape::Path(path),
        SVMarkPaint::stroked(colour(0, 0, 0, 255), 1.0),
    );

    assert_eq!(
        element,
        "<path d=\"M 0 0 L 10 10 A 5 5 0 0 1 20 25 Z\" fill=\"none\" stroke=\"#000000\" stroke-width=\"1\" stroke-linecap=\"butt\" stroke-linejoin=\"round\"/>"
    );
}

#[test]
fn a_text_run_maps_alignment_and_baseline_to_svg_keywords() {
    let element = serialize(
        SVMarkShape::Text(SVGeometryText::new(
            point(5.0, 6.0),
            "hello & <world>",
            "serif",
            12.0,
            SVTextAlign::Right,
            SVTextBaseline::Top,
        )),
        SVMarkPaint::filled(colour(17, 17, 17, 255)),
    );

    assert_eq!(
        element,
        "<text x=\"5\" y=\"6\" font-family=\"serif\" font-size=\"12\" text-anchor=\"end\" dominant-baseline=\"hanging\" fill=\"#111111\">hello &amp; &lt;world&gt;</text>"
    );
}

#[test]
fn an_image_embeds_a_png_data_uri() {
    let pixels = [
        255u8, 0, 0, 255, 0, 0, 255, 255, 0, 255, 0, 255, 255, 255, 0, 255,
    ];
    let element = serialize(
        SVMarkShape::Image(SVGeometryImage::new(
            SVResolvedRectangle::new(0.0, 0.0, 2.0, 2.0),
            2,
            2,
            &pixels,
            SVImageInterpolation::Nearest,
        )),
        SVMarkPaint::new().with_opacity(0.75),
    );

    assert!(
        element.starts_with(
            "<image x=\"0\" y=\"0\" width=\"2\" height=\"2\" href=\"data:image/png;base64,"
        ),
        "{element}"
    );
    assert!(element.contains("preserveAspectRatio=\"none\""));
    assert!(element.contains("image-rendering=\"pixelated\""));
    assert!(element.ends_with("opacity=\"0.75\"/>"), "{element}");
}
