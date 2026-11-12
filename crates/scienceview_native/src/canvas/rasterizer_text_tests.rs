//! Text is drawn as boxes with alignment, baseline and rotation.

use super::text_rasterizer::SVTextHorizontalAnchor;
use crate::canvas::SVRasterCanvas;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::SVMarkPaint;
use scienceview_core::sv_mark::SVMarkPoint;
use scienceview_core::sv_mark::SVMarkPrimitive;
use scienceview_core::sv_mark::SVMarkShape;
use scienceview_core::sv_mark::SVTextAlign;
use scienceview_core::sv_mark::SVTextBaseline;
use scienceview_core::sv_scale::SVColour;

const BLACK: SVColour = SVColour::new(0, 0, 0, 255);

fn sv_canvas(width: f64, height: f64) -> SVRasterCanvas {
    let mut canvas = SVRasterCanvas::new(&SVViewport::new(width, height));
    canvas.fill(SVColour::new(255, 255, 255, 255));
    canvas.clear_dirty_region();
    canvas
}

#[test]
fn text_is_drawn_as_boxes_with_alignment_and_baseline() {
    let mut canvas = sv_canvas(40.0, 20.0);
    canvas.draw_text_run(
        20.0,
        10.0,
        "abc",
        8.0,
        400,
        BLACK,
        SVTextHorizontalAnchor::Middle,
        SVTextBaseline::Middle,
        0.0,
        1.0,
    );

    assert!(canvas.pixels().chunks_exact(4).any(|pixel| pixel[0] < 128));
    let mut left = sv_canvas(40.0, 20.0);
    left.draw_text_run(
        20.0,
        10.0,
        "abc",
        8.0,
        700,
        BLACK,
        SVTextHorizontalAnchor::Start,
        SVTextBaseline::Top,
        0.0,
        1.0,
    );
    assert_ne!(canvas.pixels(), left.pixels());
}

#[test]
fn a_mark_text_run_is_drawn_with_its_alignment() {
    let text = scienceview_core::sv_mark::SVGeometryText::new(
        SVMarkPoint::new(20.0, 10.0),
        "12",
        "sans",
        8.0,
        SVTextAlign::Center,
        SVTextBaseline::Middle,
    );

    let mut canvas = sv_canvas(40.0, 20.0);
    canvas.draw_mark_primitive(&SVMarkPrimitive::new(
        SVMarkShape::Text(text),
        SVMarkPaint::filled(BLACK),
    ));

    assert!(canvas.pixels().chunks_exact(4).any(|pixel| pixel[0] < 128));
}

#[test]
fn rotation_moves_the_boxes() {
    let mut upright = sv_canvas(40.0, 40.0);
    upright.draw_text_run(
        20.0,
        20.0,
        "abcd",
        8.0,
        400,
        BLACK,
        SVTextHorizontalAnchor::Middle,
        SVTextBaseline::Middle,
        0.0,
        1.0,
    );
    let mut rotated = sv_canvas(40.0, 40.0);
    rotated.draw_text_run(
        20.0,
        20.0,
        "abcd",
        8.0,
        400,
        BLACK,
        SVTextHorizontalAnchor::Middle,
        SVTextBaseline::Middle,
        90.0,
        1.0,
    );

    assert_ne!(upright.pixels(), rotated.pixels());
}
