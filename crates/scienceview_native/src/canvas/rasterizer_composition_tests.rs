//! Images, decorations and the dirty region the rasterizer reports.

use crate::canvas::SVRasterCanvas;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_decoration::SVPoint;
use scienceview_core::sv_decoration::SVStroke;
use scienceview_core::sv_decoration::SVTextAnchor;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::SVGeometryCircle;
use scienceview_core::sv_mark::SVGeometryImage;
use scienceview_core::sv_mark::SVImageInterpolation;
use scienceview_core::sv_mark::SVMarkPoint;
use scienceview_core::sv_scale::SVColour;

const BLACK: SVColour = SVColour::new(0, 0, 0, 255);
const RED: SVColour = SVColour::new(255, 0, 0, 255);

fn sv_canvas(width: f64, height: f64) -> SVRasterCanvas {
    let mut canvas = SVRasterCanvas::new(&SVViewport::new(width, height));
    canvas.fill(SVColour::new(255, 255, 255, 255));
    canvas.clear_dirty_region();
    canvas
}

#[test]
fn a_nearest_image_copies_its_cells() {
    let pixels = [
        255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255,
    ];
    let image = SVGeometryImage::new(
        SVResolvedRectangle::new(0.0, 0.0, 4.0, 4.0),
        2,
        2,
        &pixels,
        SVImageInterpolation::Nearest,
    );
    let mut canvas = sv_canvas(4.0, 4.0);

    canvas.draw_image(&image, 1.0);

    assert_eq!(canvas.pixel(0, 0), Some(RED));
    assert_eq!(canvas.pixel(3, 0), Some(SVColour::new(0, 255, 0, 255)));
    assert_eq!(canvas.pixel(0, 3), Some(SVColour::new(0, 0, 255, 255)));
    assert_eq!(canvas.pixel(3, 3), Some(SVColour::new(255, 255, 0, 255)));
}

#[test]
fn a_bilinear_image_blends_the_neighbours() {
    let pixels = [
        0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ];
    let image = SVGeometryImage::new(
        SVResolvedRectangle::new(0.0, 0.0, 4.0, 4.0),
        2,
        2,
        &pixels,
        SVImageInterpolation::Bilinear,
    );
    let mut canvas = sv_canvas(4.0, 4.0);

    canvas.draw_image(&image, 1.0);

    let blended = canvas.pixel(1, 1).expect("the pixel is inside the frame");
    assert!(
        blended.red() > 0 && blended.red() < 255,
        "bilinear sampling blends across the cell, received {blended:?}"
    );
}

#[test]
fn decorations_draw_lines_rectangles_and_labels() {
    let mut canvas = sv_canvas(40.0, 20.0);
    let primitives = [
        SVDecorationPrimitive::line(
            SVPoint::new(2.0, 2.5),
            SVPoint::new(38.0, 2.5),
            SVStroke::new(BLACK, 1.0),
        ),
        SVDecorationPrimitive::rectangle(
            scienceview_core::sv_decoration::SVRectanglePrimitive::filled(
                SVResolvedRectangle::new(2.0, 6.0, 10.0, 8.0),
                RED,
            ),
        ),
        SVDecorationPrimitive::text(
            SVPoint::new(20.0, 15.0),
            "5",
            6.0,
            400,
            BLACK,
            SVTextAnchor::Middle,
        ),
    ];

    canvas.draw_decoration_primitives(&primitives);

    assert_eq!(canvas.pixel(20, 2), Some(BLACK));
    assert_eq!(canvas.pixel(6, 10), Some(RED));
    assert!(
        canvas
            .pixels()
            .chunks_exact(4)
            .any(|pixel| pixel[0] < 128 && pixel[1] < 128)
    );
}

#[test]
fn drawing_marks_only_dirty_their_region() {
    let mut canvas = sv_canvas(40.0, 40.0);
    canvas.fill_circle(
        &SVGeometryCircle::new(SVMarkPoint::new(10.0, 10.0), 3.0),
        RED,
        1.0,
    );

    let bounds = canvas
        .dirty_region()
        .bounds()
        .expect("the circle dirties its bounding box");
    assert!(bounds.maximum_x() <= 16);
    assert!(bounds.maximum_y() <= 16);
    assert!(bounds.width() < 40);
}
