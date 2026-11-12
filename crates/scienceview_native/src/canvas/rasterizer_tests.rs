//! Rectangles and circles: fill, corner radius and anti-aliased edges.

use crate::canvas::SVRasterCanvas;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::SVGeometryCircle;
use scienceview_core::sv_mark::SVGeometryRectangle;
use scienceview_core::sv_mark::SVMarkPaint;
use scienceview_core::sv_mark::SVMarkPoint;
use scienceview_core::sv_mark::SVMarkPrimitive;
use scienceview_core::sv_mark::SVMarkShape;
use scienceview_core::sv_scale::SVColour;

const BLACK: SVColour = SVColour::new(0, 0, 0, 255);
const RED: SVColour = SVColour::new(255, 0, 0, 255);
const WHITE: SVColour = SVColour::new(255, 255, 255, 255);

fn sv_canvas(width: f64, height: f64) -> SVRasterCanvas {
    let mut canvas = SVRasterCanvas::new(&SVViewport::new(width, height));
    canvas.fill(WHITE);
    canvas.clear_dirty_region();
    canvas
}

#[test]
fn a_filled_rectangle_covers_its_interior_and_leaves_the_outside_alone() {
    let mut canvas = sv_canvas(20.0, 20.0);
    let rectangle = SVGeometryRectangle::new(SVResolvedRectangle::new(4.0, 4.0, 12.0, 8.0), 0.0);

    canvas.draw_mark_primitive(&SVMarkPrimitive::new(
        SVMarkShape::Rectangle(rectangle),
        SVMarkPaint::filled(BLACK),
    ));

    assert_eq!(canvas.pixel(10, 8), Some(BLACK));
    assert_eq!(canvas.pixel(0, 0), Some(WHITE));
    assert_eq!(canvas.pixel(19, 19), Some(WHITE));
}

#[test]
fn a_rectangle_edge_is_anti_aliased() {
    let mut canvas = sv_canvas(10.0, 10.0);
    canvas.fill_rectangle(
        &SVGeometryRectangle::new(SVResolvedRectangle::new(2.0, 2.0, 4.3, 4.0), 0.0),
        BLACK,
        1.0,
    );

    let edge = canvas.pixel(6, 3).expect("the edge is inside the frame");
    assert!(
        edge.red() > 0 && edge.red() < 255,
        "a fractional edge must produce an intermediate channel, received {edge:?}"
    );
}

#[test]
fn a_rounded_rectangle_clears_its_corners() {
    let mut canvas = sv_canvas(20.0, 20.0);
    canvas.fill_rectangle(
        &SVGeometryRectangle::new(SVResolvedRectangle::new(2.0, 2.0, 16.0, 16.0), 6.0),
        BLACK,
        1.0,
    );

    assert_eq!(canvas.pixel(10, 10), Some(BLACK));
    let corner = canvas.pixel(2, 2).expect("the corner is inside the frame");
    assert!(
        corner.red() > 200,
        "a rounded corner must stay background, received {corner:?}"
    );
}

#[test]
fn a_filled_circle_has_a_soft_edge_and_a_solid_centre() {
    let mut canvas = sv_canvas(20.0, 20.0);
    canvas.fill_circle(
        &SVGeometryCircle::new(SVMarkPoint::new(10.0, 10.0), 5.0),
        RED,
        1.0,
    );

    assert_eq!(canvas.pixel(10, 10), Some(RED));
    let mut partial = 0;
    for x in 0..20 {
        if let Some(colour) = canvas.pixel(x, 10) {
            if colour.red() == 255 && colour.green() > 0 && colour.green() < 255 {
                partial += 1;
            }
        }
    }
    assert!(
        partial > 0,
        "the circle rim must carry anti-aliased pixels, found {partial}"
    );
}

#[test]
fn a_stroked_circle_draws_a_ring_not_a_disc() {
    let mut canvas = sv_canvas(20.0, 20.0);
    canvas.stroke_circle(
        &SVGeometryCircle::new(SVMarkPoint::new(10.0, 10.0), 5.0),
        2.0,
        BLACK,
        1.0,
    );

    assert_eq!(canvas.pixel(10, 10), Some(WHITE));
    let rim = canvas.pixel(15, 10).expect("the rim is inside the frame");
    assert!(rim.red() < 40, "the ring is painted, received {rim:?}");
}
