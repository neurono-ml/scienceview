//! The comparable frame: text output, wide glyphs and clipping.

use super::SVCellFrame;
use super::sv_cell_display_width;

#[test]
fn a_frame_keeps_its_grid_in_plain_text() {
    let mut frame = SVCellFrame::new(4, 2);
    if let Some(cell) = frame.cell_mut(0, 0) {
        cell.set_symbol('a');
    }
    if let Some(cell) = frame.cell_mut(3, 1) {
        cell.set_symbol('z');
    }

    assert_eq!(frame.to_text(), "a   \n   z");
    assert_eq!(frame.row_text(7), "");
    assert!(!frame.is_empty());
    assert!(SVCellFrame::new(0, 0).is_empty());
}

#[test]
fn a_wide_glyph_takes_two_columns_and_keeps_alignment() {
    let mut frame = SVCellFrame::new(4, 1);
    if let Some(cell) = frame.cell_mut(0, 0) {
        cell.set_symbol('日');
    }
    if let Some(cell) = frame.cell_mut(1, 0) {
        cell.set_continuation();
    }
    if let Some(cell) = frame.cell_mut(2, 0) {
        cell.set_symbol('x');
    }

    assert_eq!(frame.row_text(0), "日x ");
    assert_eq!(
        frame.cell(1, 0).map(|cell| cell.is_continuation()),
        Some(true)
    );
    assert_eq!(sv_cell_display_width('日'), 2);
    assert_eq!(sv_cell_display_width('x'), 1);
    assert_eq!(sv_cell_display_width('\u{0301}'), 0);
}

#[test]
fn ansi_output_skips_continuation_columns() {
    let mut frame = SVCellFrame::new(3, 1);
    if let Some(cell) = frame.cell_mut(0, 0) {
        cell.set_symbol('日');
    }
    if let Some(cell) = frame.cell_mut(1, 0) {
        cell.set_continuation();
    }
    if let Some(cell) = frame.cell_mut(2, 0) {
        cell.set_symbol('x');
    }

    assert_eq!(
        frame.to_ansi(super::SVAnsiColourDepth::Ansi256),
        "日x\u{1b}[0m\n"
    );
}
