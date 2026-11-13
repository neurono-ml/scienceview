//! Block glyphs for filled shapes and fractional heights.

/// The eight partial-block glyphs, from one eighth to a full cell.
const PARTIAL_BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Returns the block glyph for a vertical coverage fraction in `0.0..=1.0`.
///
/// A fraction that covers less than one sixteenth of the cell returns a blank,
/// and a full cell returns the solid block. Everything in between keeps its
/// one-eighth granularity instead of being rounded to a full or empty cell.
pub(crate) fn sv_partial_block(fraction: f64) -> char {
    if !fraction.is_finite() || fraction <= 0.0 {
        return ' ';
    }
    let eighths = (fraction.clamp(0.0, 1.0) * 8.0).round() as usize;
    if eighths == 0 {
        return ' ';
    }
    PARTIAL_BLOCKS[eighths.min(8) - 1]
}

/// Returns the box-drawing glyph for the dominant direction of a segment.
pub(crate) fn sv_line_glyph(start: (f64, f64), end: (f64, f64)) -> char {
    let horizontal = (end.0 - start.0).abs();
    let vertical = (end.1 - start.1).abs();

    if horizontal >= vertical * 2.0 {
        '─'
    } else if vertical >= horizontal * 2.0 {
        '│'
    } else if (end.0 - start.0) * (end.1 - start.1) >= 0.0 {
        '╲'
    } else {
        '╱'
    }
}

/// Returns the corner glyph for a rectangle outline.
pub(crate) fn sv_corner_glyph(top: bool, left: bool) -> char {
    match (top, left) {
        (true, true) => '┌',
        (true, false) => '┐',
        (false, true) => '└',
        (false, false) => '┘',
    }
}
