//! The headless canvas: sizing, the device pixel ratio, pixel access and resize.

use crate::canvas::SVNativeFrame;
use crate::canvas::SVRasterCanvas;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_scale::SVColour;

#[test]
fn a_new_canvas_is_transparent_and_sized_from_the_logical_viewport() {
    let canvas = SVRasterCanvas::new(&SVViewport::new(20.0, 10.0));

    assert_eq!(canvas.width(), 20);
    assert_eq!(canvas.height(), 10);
    assert_eq!(canvas.device_pixel_ratio(), 1.0);
    assert_eq!(canvas.pixels().len(), 20 * 10 * 4);
    assert_eq!(canvas.pixel(0, 0), Some(SVColour::new(0, 0, 0, 0)));
    assert_eq!(canvas.pixel(20, 0), None);
    assert_eq!(canvas.pixel(0, 10), None);
}

#[test]
fn the_device_pixel_ratio_scales_the_physical_size() {
    let viewport = SVViewport::new(20.0, 10.0).with_device_pixel_ratio(2.0);
    let canvas = SVRasterCanvas::new(&viewport);

    assert_eq!(canvas.width(), 40);
    assert_eq!(canvas.height(), 20);
    assert_eq!(canvas.device_pixel_ratio(), 2.0);
    assert_eq!(canvas.logical_width(), 20.0);
    assert_eq!(canvas.logical_height(), 10.0);
}

#[test]
fn filling_sets_every_pixel_and_marks_the_whole_frame_dirty() {
    let mut canvas = SVRasterCanvas::new(&SVViewport::new(4.0, 3.0));
    canvas.clear_dirty_region();
    assert!(canvas.dirty_region().is_empty());

    let white = SVColour::new(255, 255, 255, 255);
    canvas.fill(white);

    for y in 0..3 {
        for x in 0..4 {
            assert_eq!(canvas.pixel(x, y), Some(white));
        }
    }
    let bounds = canvas
        .dirty_region()
        .bounds()
        .expect("a fill dirties the frame");
    assert_eq!((bounds.width(), bounds.height()), (4, 3));
}

#[test]
fn clearing_resets_the_frame_to_transparent() {
    let mut canvas = SVRasterCanvas::new(&SVViewport::new(2.0, 2.0));
    canvas.fill(SVColour::new(10, 20, 30, 255));
    canvas.clear_dirty_region();

    canvas.clear();

    for y in 0..2 {
        for x in 0..2 {
            assert_eq!(canvas.pixel(x, y), Some(SVColour::new(0, 0, 0, 0)));
        }
    }
    let bounds = canvas
        .dirty_region()
        .bounds()
        .expect("a clear dirties the frame");
    assert_eq!((bounds.width(), bounds.height()), (2, 2));
}

#[test]
fn resizing_reallocates_and_dirties_the_frame() {
    let mut canvas = SVRasterCanvas::new(&SVViewport::new(4.0, 3.0));
    canvas.clear_dirty_region();

    let changed = canvas.resize(&SVViewport::new(8.0, 6.0));

    assert!(changed);
    assert_eq!((canvas.width(), canvas.height()), (8, 6));
    assert_eq!(canvas.pixels().len(), 8 * 6 * 4);
    assert_eq!(canvas.pixel(7, 5), Some(SVColour::new(0, 0, 0, 0)));
    assert!(!canvas.dirty_region().is_empty());
}

#[test]
fn resizing_to_the_same_viewport_reports_no_change() {
    let mut canvas = SVRasterCanvas::new(&SVViewport::new(4.0, 3.0));
    canvas.clear_dirty_region();

    assert!(!canvas.resize(&SVViewport::new(4.0, 3.0)));
    assert!(canvas.dirty_region().is_empty());
}

#[test]
fn a_frame_carries_its_dimensions_and_pixels() {
    let mut canvas = SVRasterCanvas::new(&SVViewport::new(2.0, 2.0));
    canvas.fill(SVColour::new(1, 2, 3, 255));

    let frame: SVNativeFrame = canvas.to_frame();

    assert_eq!((frame.width(), frame.height()), (2, 2));
    assert_eq!(frame.pixel(1, 1), Some(SVColour::new(1, 2, 3, 255)));
    assert_eq!(frame.pixels().len(), 2 * 2 * 4);
    assert!(!frame.is_empty());
}

#[test]
fn a_frame_rejects_a_buffer_that_does_not_match_its_dimensions() {
    let frame = SVNativeFrame::new(2, 2, 1.0, vec![0; 3]);

    assert!(frame.is_empty());
    assert_eq!((frame.width(), frame.height()), (0, 0));
}
