//! How many terminal columns one glyph occupies.
//!
//! East Asian wide characters take two columns, combining marks take none and
//! everything else takes one. The table covers the ranges a scientific label or
//! a categorical name actually contains; an unlisted character is treated as
//! narrow, which keeps the grid deterministic for every other script.

/// Returns the number of terminal columns `symbol` occupies.
pub fn sv_cell_display_width(symbol: char) -> usize {
    let code = symbol as u32;

    if sv_is_zero_width(code) {
        return 0;
    }
    if sv_is_wide(code) {
        return 2;
    }
    1
}

fn sv_is_zero_width(code: u32) -> bool {
    matches!(
        code,
        0x0300..=0x036F
            | 0x1AB0..=0x1AFF
            | 0x1DC0..=0x1DFF
            | 0x20D0..=0x20FF
            | 0xFE00..=0xFE0F
            | 0xFE20..=0xFE2F
    )
}

fn sv_is_wide(code: u32) -> bool {
    matches!(
        code,
        0x1100..=0x115F
            | 0x2E80..=0x303E
            | 0x3041..=0x33FF
            | 0x3400..=0x4DBF
            | 0x4E00..=0x9FFF
            | 0xA000..=0xA4CF
            | 0xA960..=0xA97F
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF00..=0xFF60
            | 0xFFE0..=0xFFE6
            | 0x1F300..=0x1F64F
            | 0x1F900..=0x1F9FF
            | 0x20000..=0x3FFFD
    )
}
