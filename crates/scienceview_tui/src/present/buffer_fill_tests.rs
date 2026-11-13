//! The ratatui buffer conversion, exercised headless with a plain buffer.

use super::buffer_fill::sv_fill_buffer;
use crate::cell::SVCellFrame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use scienceview_core::sv_scale::SVColour;

#[test]
fn a_frame_is_copied_with_its_glyph_and_colours() {
    let mut frame = SVCellFrame::new(2, 1);
    if let Some(cell) = frame.cell_mut(0, 0) {
        cell.set_symbol('█');
        cell.set_foreground(SVColour::new(9, 105, 218, 255));
        cell.set_background(SVColour::new(255, 255, 255, 255));
    }
    if let Some(cell) = frame.cell_mut(1, 0) {
        cell.set_symbol('x');
    }

    let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
    sv_fill_buffer(&frame, &mut buffer);

    assert_eq!(buffer[(0, 0)].symbol(), "█");
    assert_eq!(buffer[(0, 0)].fg, Color::Rgb(9, 105, 218));
    assert_eq!(buffer[(0, 0)].bg, Color::Rgb(255, 255, 255));
    assert_eq!(buffer[(1, 0)].symbol(), "x");
}

#[test]
fn a_frame_larger_than_the_buffer_is_clipped() {
    let mut frame = SVCellFrame::new(4, 2);
    if let Some(cell) = frame.cell_mut(3, 1) {
        cell.set_symbol('z');
    }

    let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
    sv_fill_buffer(&frame, &mut buffer);

    assert_eq!(buffer[(0, 0)].symbol(), " ");
    assert_eq!(buffer[(1, 0)].symbol(), " ");
}

#[test]
fn a_continuation_column_keeps_the_buffer_default() {
    let mut frame = SVCellFrame::new(2, 1);
    if let Some(cell) = frame.cell_mut(0, 0) {
        cell.set_symbol('日');
    }
    if let Some(cell) = frame.cell_mut(1, 0) {
        cell.set_continuation();
    }

    let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
    sv_fill_buffer(&frame, &mut buffer);

    assert_eq!(buffer[(0, 0)].symbol(), "日");
    assert_eq!(buffer[(1, 0)].symbol(), " ");
}
