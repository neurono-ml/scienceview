//! Tests for path grouping: a large point set becomes one element.
//!
//! The document size is what makes an SVG openable: ten thousand `<circle>`
//! elements cost ten thousand parse nodes and a large attribute repeat. One
//! grouped path keeps the geometry and drops the per-element overhead.

use crate::serialize::sv_serialize_mark_geometry;
use scienceview_core::sv_mark::{
    SVGeometryCircle, SVMarkGeometry, SVMarkPaint, SVMarkPoint, SVMarkPrimitive, SVMarkShape,
};
use scienceview_core::sv_scale::SVColour;

const FONT: &str = "system-ui";

fn circle(x: f64, y: f64, radius: f64, paint: &SVMarkPaint) -> SVMarkPrimitive<'static> {
    SVMarkPrimitive::new(
        SVMarkShape::Circle(SVGeometryCircle::new(SVMarkPoint::new(x, y), radius)),
        paint.clone(),
    )
}

#[test]
fn ten_thousand_equal_circles_become_one_path_element() {
    let paint = SVMarkPaint::filled(SVColour::new(9, 105, 218, 255));
    let primitives = (0..10_000)
        .map(|index| {
            let position = index as f64;
            circle(position % 500.0, position / 500.0 * 3.0, 2.0, &paint)
        })
        .collect();
    let geometry = SVMarkGeometry::new(primitives);

    let elements = sv_serialize_mark_geometry(&geometry, FONT);

    assert_eq!(elements.len(), 1, "the whole run is one path");
    assert!(elements[0].starts_with("<path d=\""));
    assert!(elements[0].ends_with(" fill=\"#0969da\"/>"));
}

#[test]
fn the_grouped_document_stays_small_per_point() {
    let paint = SVMarkPaint::filled(SVColour::new(255, 0, 0, 255));
    let primitives = (0..10_000)
        .map(|index| circle(index as f64 % 800.0, index as f64 / 800.0, 1.5, &paint))
        .collect();
    let geometry = SVMarkGeometry::new(primitives);

    let bytes: usize = sv_serialize_mark_geometry(&geometry, FONT)
        .iter()
        .map(String::len)
        .sum();

    assert!(
        bytes < 10_000 * 120,
        "grouping must keep the document below 120 bytes per point, got {bytes}"
    );
}

#[test]
fn two_paints_produce_two_paths() {
    let first = SVMarkPaint::filled(SVColour::new(0, 0, 0, 255));
    let second = SVMarkPaint::filled(SVColour::new(255, 255, 255, 255));
    let geometry = SVMarkGeometry::new(vec![
        circle(0.0, 0.0, 1.0, &first),
        circle(1.0, 0.0, 1.0, &first),
        circle(2.0, 0.0, 1.0, &second),
    ]);

    let elements = sv_serialize_mark_geometry(&geometry, FONT);

    assert_eq!(elements.len(), 2);
    assert!(elements[0].contains("fill=\"#000000\""));
    assert!(elements[1].contains("fill=\"#ffffff\""));
}

#[test]
fn a_non_circle_breaks_the_run() {
    let paint = SVMarkPaint::filled(SVColour::new(0, 0, 0, 255));
    let rectangle = SVMarkPrimitive::new(
        SVMarkShape::Rectangle(scienceview_core::sv_mark::SVGeometryRectangle::new(
            scienceview_core::sv_layout::SVResolvedRectangle::new(0.0, 0.0, 1.0, 1.0),
            0.0,
        )),
        paint.clone(),
    );
    let geometry = SVMarkGeometry::new(vec![
        circle(0.0, 0.0, 1.0, &paint),
        rectangle,
        circle(2.0, 0.0, 1.0, &paint),
    ]);

    let elements = sv_serialize_mark_geometry(&geometry, FONT);

    assert_eq!(elements.len(), 3);
    assert!(elements[0].starts_with("<path "));
    assert!(elements[1].starts_with("<rect "));
    assert!(elements[2].starts_with("<path "));
}

#[test]
fn grouping_is_deterministic() {
    let paint = SVMarkPaint::filled(SVColour::new(1, 2, 3, 255));
    let geometry = SVMarkGeometry::new(
        (0..100)
            .map(|index| circle(index as f64, index as f64, 1.0, &paint))
            .collect(),
    );

    assert_eq!(
        sv_serialize_mark_geometry(&geometry, FONT),
        sv_serialize_mark_geometry(&geometry, FONT)
    );
}
