//! Braille sub-cell dots.
//!
//! One character cell holds a 2 by 4 dot matrix, so a curve keeps four times
//! the vertical resolution of the coarse cell grid. A mark stamps dots into a
//! cell and the drawing order decides whether a later text glyph replaces them.

use super::cell_definition::SVCell;

/// The first braille pattern; adding the dot bits yields the glyph.
const BRAILLE_BASE: u32 = 0x2800;

/// Returns the bit of the dot at `sub_column` (0 or 1) and `sub_row` (0–3).
pub(crate) fn sv_braille_dot_bit(sub_column: usize, sub_row: usize) -> u8 {
    match (sub_column, sub_row) {
        (0, 0) => 0x01,
        (0, 1) => 0x02,
        (0, 2) => 0x04,
        (1, 0) => 0x08,
        (1, 1) => 0x10,
        (1, 2) => 0x20,
        (0, 3) => 0x40,
        (1, 3) => 0x80,
        _ => 0,
    }
}

/// Returns the glyph of a dot pattern, or a blank when no dot is set.
pub(crate) fn sv_braille_symbol(bits: u8) -> char {
    if bits == 0 {
        return ' ';
    }
    char::from_u32(BRAILLE_BASE + u32::from(bits)).unwrap_or(' ')
}

/// Returns the dot pattern already stored in `cell`, when it is a braille
/// glyph.
fn sv_braille_bits(cell: &SVCell) -> Option<u8> {
    let code = cell.symbol() as u32;
    let offset = code.checked_sub(BRAILLE_BASE)?;
    (offset <= 0xFF).then_some(offset as u8)
}

/// Adds one dot to a cell, keeping the dots already there.
pub(crate) fn sv_stamp_braille_dot(
    cell: &mut SVCell,
    sub_column: usize,
    sub_row: usize,
    colour: scienceview_core::sv_scale::SVColour,
) {
    let existing = sv_braille_bits(cell).unwrap_or(0);
    let bits = existing | sv_braille_dot_bit(sub_column, sub_row);
    cell.set_symbol(sv_braille_symbol(bits));
    cell.set_foreground(colour);
}
