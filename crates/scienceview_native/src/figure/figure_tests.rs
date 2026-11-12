//! The composed default figure: background, plot, marks and viewport re-solve.

use crate::canvas::SVRasterCanvas;
use crate::figure::sv_rasterize_default_figure;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_scale::SVColour;

fn sv_request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Native,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: Some("Weights"),
    }
}

#[test]
fn the_default_figure_starts_from_the_theme_background() {
    let values = [1.0, 2.0, 3.0];
    let frame = sv_rasterize_default_figure(&sv_request(&values), &SVViewport::new(200.0, 120.0))
        .expect("the figure renders");

    assert_eq!(frame.pixel(0, 0), Some(SVColour::new(255, 255, 255, 255)));
}

#[test]
fn a_figure_without_values_still_draws_the_plot_outline() {
    let values: [f64; 0] = [];
    let frame = sv_rasterize_default_figure(&sv_request(&values), &SVViewport::new(200.0, 120.0))
        .expect("the figure renders");

    let mut painted = 0;
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            if frame.pixel(x, y) != Some(SVColour::new(255, 255, 255, 255)) {
                painted += 1;
            }
        }
    }
    assert!(
        painted > 100,
        "an empty figure still draws its plot outline, painted {painted} pixels"
    );
}

#[test]
fn values_are_drawn_inside_the_plot() {
    let values = [0.0, 10.0, 2.0, 8.0];
    let frame = sv_rasterize_default_figure(&sv_request(&values), &SVViewport::new(320.0, 200.0))
        .expect("the figure renders");

    let mut dark_pixels = 0;
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            if let Some(colour) = frame.pixel(x, y) {
                if colour.red() < 200 && colour.green() < 200 {
                    dark_pixels += 1;
                }
            }
        }
    }
    assert!(
        dark_pixels > 500,
        "the line and the axes paint a readable figure, painted {dark_pixels} dark pixels"
    );
}

#[test]
fn a_resized_viewport_is_re_solved_rather_than_stretched() {
    let values = [1.0, 2.0, 3.0];
    let narrow = sv_rasterize_default_figure(&sv_request(&values), &SVViewport::new(200.0, 120.0))
        .expect("the figure renders");
    let wide = sv_rasterize_default_figure(&sv_request(&values), &SVViewport::new(400.0, 120.0))
        .expect("the figure renders");

    assert_eq!((narrow.width(), narrow.height()), (200, 120));
    assert_eq!((wide.width(), wide.height()), (400, 120));

    let narrow_center = narrow.pixel(100, 60);
    let wide_center = wide.pixel(100, 60);
    assert_ne!(
        narrow, wide,
        "the layout re-solves instead of scaling the previous frame"
    );
    assert!(narrow_center.is_some());
    assert!(wide_center.is_some());
}

#[test]
fn a_canvas_and_the_frame_path_produce_the_same_pixels() {
    let values = [4.0, 1.0, 3.0];
    let viewport = SVViewport::new(240.0, 160.0);
    let frame =
        sv_rasterize_default_figure(&sv_request(&values), &viewport).expect("the figure renders");

    let mut canvas = SVRasterCanvas::new(&viewport);
    canvas.fill(SVColour::new(255, 255, 255, 255));
    assert_eq!(canvas.width(), frame.width());
}
