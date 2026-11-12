//! Paths: filling, stroking, arcs, dashes, caps and joins.

use crate::canvas::SVRasterCanvas;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::SVArcTo;
use scienceview_core::sv_mark::SVGeometryPath;
use scienceview_core::sv_mark::SVLineCap;
use scienceview_core::sv_mark::SVLineJoin;
use scienceview_core::sv_mark::SVMarkPaint;
use scienceview_core::sv_mark::SVMarkPoint;
use scienceview_core::sv_mark::SVMarkPrimitive;
use scienceview_core::sv_mark::SVMarkShape;
use scienceview_core::sv_mark::SVPathCommand;
use scienceview_core::sv_scale::SVColour;

const BLACK: SVColour = SVColour::new(0, 0, 0, 255);

fn sv_canvas(width: f64, height: f64) -> SVRasterCanvas {
    let mut canvas = SVRasterCanvas::new(&SVViewport::new(width, height));
    canvas.fill(SVColour::new(255, 255, 255, 255));
    canvas.clear_dirty_region();
    canvas
}

#[test]
fn a_filled_path_uses_the_non_zero_winding_rule() {
    let mut canvas = sv_canvas(20.0, 20.0);
    let path = SVGeometryPath::new(vec![
        SVPathCommand::MoveTo(SVMarkPoint::new(4.0, 4.0)),
        SVPathCommand::LineTo(SVMarkPoint::new(16.0, 4.0)),
        SVPathCommand::LineTo(SVMarkPoint::new(16.0, 16.0)),
        SVPathCommand::LineTo(SVMarkPoint::new(4.0, 16.0)),
        SVPathCommand::Close,
    ]);

    canvas.draw_mark_primitive(&SVMarkPrimitive::new(
        SVMarkShape::Path(path),
        SVMarkPaint::filled(BLACK),
    ));

    assert_eq!(canvas.pixel(10, 10), Some(BLACK));
    assert_eq!(canvas.pixel(2, 10), Some(SVColour::new(255, 255, 255, 255)));
}

#[test]
fn a_stroked_path_follows_every_segment() {
    let mut canvas = sv_canvas(20.0, 20.0);
    let path = SVGeometryPath::new(vec![
        SVPathCommand::MoveTo(SVMarkPoint::new(2.0, 10.0)),
        SVPathCommand::LineTo(SVMarkPoint::new(10.0, 10.0)),
        SVPathCommand::LineTo(SVMarkPoint::new(10.0, 18.0)),
    ]);

    canvas.draw_mark_primitive(&SVMarkPrimitive::new(
        SVMarkShape::Path(path),
        SVMarkPaint::stroked(BLACK, 2.0),
    ));

    assert_eq!(canvas.pixel(6, 10), Some(BLACK));
    assert_eq!(canvas.pixel(10, 14), Some(BLACK));
}

#[test]
fn an_arc_is_flattened_into_the_path() {
    let mut canvas = sv_canvas(20.0, 20.0);
    let path = SVGeometryPath::new(vec![
        SVPathCommand::MoveTo(SVMarkPoint::new(10.0, 10.0)),
        SVPathCommand::ArcTo(SVArcTo::new(
            SVMarkPoint::new(10.0, 10.0),
            8.0,
            0.0,
            std::f64::consts::PI,
        )),
    ]);

    canvas.draw_mark_primitive(&SVMarkPrimitive::new(
        SVMarkShape::Path(path),
        SVMarkPaint::stroked(BLACK, 2.0),
    ));

    let right = canvas
        .pixel(18, 10)
        .expect("the arc start is inside the frame");
    let left = canvas
        .pixel(2, 10)
        .expect("the arc end is inside the frame");
    assert!(
        right.red() < 40,
        "the arc starts at its first angle, received {right:?}"
    );
    assert!(
        left.red() < 40,
        "the arc ends at its last angle, received {left:?}"
    );
}

#[test]
fn the_cap_and_the_dash_pattern_change_the_output() {
    let path = SVGeometryPath::new(vec![
        SVPathCommand::MoveTo(SVMarkPoint::new(4.0, 10.0)),
        SVPathCommand::LineTo(SVMarkPoint::new(16.0, 10.0)),
    ]);
    let paint = SVMarkPaint::stroked(BLACK, 4.0)
        .with_dash(scienceview_core::sv_mark::SVDashPattern::Dashed);

    let mut dashed = sv_canvas(20.0, 20.0);
    dashed.draw_mark_primitive(&SVMarkPrimitive::new(
        SVMarkShape::Path(path.clone()),
        paint.clone(),
    ));
    let mut solid = sv_canvas(20.0, 20.0);
    solid.draw_mark_primitive(&SVMarkPrimitive::new(
        SVMarkShape::Path(path),
        SVMarkPaint::stroked(BLACK, 4.0),
    ));

    assert_ne!(dashed.pixels(), solid.pixels());
}

#[test]
fn every_cap_and_join_style_is_accepted() {
    let path = SVGeometryPath::new(vec![
        SVPathCommand::MoveTo(SVMarkPoint::new(4.0, 6.0)),
        SVPathCommand::LineTo(SVMarkPoint::new(10.0, 14.0)),
        SVPathCommand::LineTo(SVMarkPoint::new(16.0, 6.0)),
    ]);

    for cap in [SVLineCap::Butt, SVLineCap::Round, SVLineCap::Square] {
        for join in [SVLineJoin::Miter, SVLineJoin::Round, SVLineJoin::Bevel] {
            let mut canvas = sv_canvas(20.0, 20.0);
            let paint = SVMarkPaint::stroked(BLACK, 2.0)
                .with_cap(cap)
                .with_join(join);
            canvas.draw_mark_primitive(&SVMarkPrimitive::new(
                SVMarkShape::Path(path.clone()),
                paint,
            ));
            assert!(
                canvas.pixels().chunks_exact(4).any(|pixel| pixel[0] < 128),
                "the {cap:?} cap with the {join:?} join draws something"
            );
        }
    }
}
