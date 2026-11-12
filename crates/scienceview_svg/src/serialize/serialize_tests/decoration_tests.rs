//! Decoration primitive snapshots and the closed mark vocabulary.

use crate::serialize::sv_serialize_decoration_primitive;
use crate::serialize::sv_serialize_mark_primitive;
use scienceview_core::sv_decoration::{
    SVDecorationPrimitive, SVPoint, SVRectanglePrimitive, SVStroke, SVTextAnchor,
};
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{SVDotShape, SVImageRaster, SVMark, SVMarkRequest};
use scienceview_core::sv_scale::{SVColour, SVScaleBuilder, SVScaleKind};

const FONT: &str = "system-ui";

fn colour(red: u8, green: u8, blue: u8, alpha: u8) -> SVColour {
    SVColour::new(red, green, blue, alpha)
}

#[test]
fn every_decoration_primitive_has_one_element() {
    let line = sv_serialize_decoration_primitive(
        &SVDecorationPrimitive::line(
            SVPoint::new(0.0, 0.0),
            SVPoint::new(10.0, 10.0),
            SVStroke::new(colour(224, 224, 224, 255), 1.0),
        ),
        FONT,
    );
    let rectangle = sv_serialize_decoration_primitive(
        &SVDecorationPrimitive::rectangle(SVRectanglePrimitive::filled(
            SVResolvedRectangle::new(1.0, 2.0, 3.0, 4.0),
            colour(171, 205, 239, 255),
        )),
        FONT,
    );
    let text = sv_serialize_decoration_primitive(
        &SVDecorationPrimitive::text(
            SVPoint::new(10.0, 20.0),
            "label <1>",
            12.0,
            400,
            colour(51, 51, 51, 255),
            SVTextAnchor::Start,
        ),
        FONT,
    );

    assert_eq!(
        line,
        "<line x1=\"0\" y1=\"0\" x2=\"10\" y2=\"10\" stroke=\"#e0e0e0\" stroke-width=\"1\"/>"
    );
    assert_eq!(
        rectangle,
        "<rect x=\"1\" y=\"2\" width=\"3\" height=\"4\" fill=\"#abcdef\"/>"
    );
    assert_eq!(
        text,
        "<text x=\"10\" y=\"20\" font-family=\"system-ui\" font-size=\"12\" font-weight=\"400\" text-anchor=\"start\" fill=\"#333333\">label &lt;1&gt;</text>"
    );
}

#[test]
fn a_rotated_decoration_text_carries_a_rotation_transform() {
    let rotated = sv_serialize_decoration_primitive(
        &SVDecorationPrimitive::rotated_text(
            SVPoint::new(10.0, 20.0),
            "tick",
            11.0,
            500,
            colour(1, 2, 3, 255),
            SVTextAnchor::End,
            90.0,
        ),
        FONT,
    );

    assert!(rotated.contains("text-anchor=\"end\""));
    assert!(rotated.contains("transform=\"rotate(90 10 20)\""));
}

#[test]
fn every_mark_variant_produces_at_least_one_element() {
    let horizontal_values = [0.0, 1.0, 2.0];
    let vertical_values = [1.0, 2.0, 1.0];
    let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 2.0)
        .range(0.0, 100.0)
        .build()
        .expect("the scale is valid");
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 2.0)
        .range(100.0, 0.0)
        .build()
        .expect("the scale is valid");
    let upper_values = [2.0, 3.0, 2.0];
    let labels = ["a", "b", "c"];
    let pixels = [0u8; 16];
    let raster = SVImageRaster::new(2, 2, &pixels);
    let request = SVMarkRequest::new(
        scienceview_core::sv_data::SVColumnView::Dense(&horizontal_values),
        scienceview_core::sv_data::SVColumnView::Dense(&vertical_values),
        &horizontal,
        &vertical,
        SVResolvedRectangle::new(0.0, 0.0, 100.0, 100.0),
    )
    .with_lower(scienceview_core::sv_data::SVColumnView::Dense(
        &vertical_values,
    ))
    .with_upper(scienceview_core::sv_data::SVColumnView::Dense(
        &upper_values,
    ))
    .with_labels(&labels)
    .with_image(raster);

    let marks: [SVMark; 9] = [
        SVMark::dot().shape(SVDotShape::Circle).into(),
        SVMark::line().into(),
        SVMark::area().into(),
        SVMark::bar().into(),
        SVMark::ribbon().into(),
        SVMark::text().into(),
        SVMark::image().into(),
        SVMark::wedge().into(),
        SVMark::error_bar().into(),
    ];

    for mark in marks {
        let geometry = mark
            .geometry(&request)
            .unwrap_or_else(|error| panic!("{} renders: {error}", mark.kind()));
        assert!(
            !geometry.is_empty(),
            "{} must produce a primitive",
            mark.kind()
        );
        for primitive in geometry.primitives() {
            let element = sv_serialize_mark_primitive(primitive, FONT);
            assert!(!element.is_empty(), "{} serializes", mark.kind());
        }
    }
}
