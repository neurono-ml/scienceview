//! ANSI colour mapping and the cell frame's escape sequences.

use super::SVAnsiColourDepth;
use super::SVCell;
use super::SVCellFrame;
use super::sv_ansi_16_index;
use super::sv_ansi_256_index;
use super::sv_ansi_colour_code;
use scienceview_core::sv_scale::SVColour;

#[test]
fn true_colour_emits_the_exact_channels() {
    let colour = SVColour::new(9, 105, 218, 255);

    assert_eq!(
        sv_ansi_colour_code(colour, SVAnsiColourDepth::TrueColor).as_deref(),
        Some("38;2;9;105;218")
    );
}

#[test]
fn the_256_palette_maps_primaries_to_their_cube_entries() {
    assert_eq!(sv_ansi_256_index(SVColour::new(255, 0, 0, 255)), 196);
    assert_eq!(sv_ansi_256_index(SVColour::new(0, 255, 0, 255)), 46);
    assert_eq!(sv_ansi_256_index(SVColour::new(0, 0, 255, 255)), 21);
    assert_eq!(
        sv_ansi_colour_code(SVColour::new(255, 0, 0, 255), SVAnsiColourDepth::Ansi256).as_deref(),
        Some("38;5;196")
    );
}

#[test]
fn the_sixteen_palette_uses_bright_and_normal_system_colours() {
    assert_eq!(sv_ansi_16_index(SVColour::new(255, 0, 0, 255)), 9);
    assert_eq!(
        sv_ansi_colour_code(SVColour::new(255, 0, 0, 255), SVAnsiColourDepth::Ansi16).as_deref(),
        Some("91")
    );
    assert_eq!(
        sv_ansi_colour_code(SVColour::new(0, 0, 0, 255), SVAnsiColourDepth::Ansi16).as_deref(),
        Some("30")
    );
}

#[test]
fn a_fully_transparent_colour_emits_nothing() {
    assert_eq!(
        sv_ansi_colour_code(SVColour::new(1, 2, 3, 0), SVAnsiColourDepth::TrueColor),
        None
    );
}

#[test]
fn a_frame_with_a_coloured_cell_emits_foreground_and_background_codes() {
    let mut frame = SVCellFrame::new(1, 1);
    if let Some(cell) = frame.cell_mut(0, 0) {
        cell.set_symbol('█');
        cell.set_foreground(SVColour::new(9, 105, 218, 255));
        cell.set_background(SVColour::new(255, 255, 255, 255));
    }

    let ansi = frame.to_ansi(SVAnsiColourDepth::TrueColor);

    assert!(ansi.contains("\u{1b}[38;2;9;105;218m"));
    assert!(ansi.contains("\u{1b}[48;2;255;255;255m"));
    assert!(ansi.ends_with("\u{1b}[0m\n"));
}

#[test]
fn a_blank_frame_is_spaces_with_a_reset_per_row() {
    let frame = SVCellFrame::new(3, 2);

    assert_eq!(
        frame.to_ansi(SVAnsiColourDepth::Ansi256),
        "   \u{1b}[0m\n   \u{1b}[0m\n"
    );
    assert_eq!(frame.to_text(), "   \n   ");
    assert_eq!(frame.cell(0, 0), Some(&SVCell::blank()));
    assert_eq!(frame.cell(3, 0), None);
}
