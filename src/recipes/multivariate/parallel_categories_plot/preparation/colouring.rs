//! Assigning a ribbon colour from the count or from a category.
//!
//! The palette is the shared categorical palette of the scale kernel. `Count`
//! ranks the distinct aggregated counts and assigns one palette entry per
//! rank, so the heaviest ribbon is always the last entry; `Category` reads the
//! configured grouping column, or the left band when no grouping column is
//! set.

use super::prepared::SVPreparedRibbon;
use crate::sv_scale::{SVScalePalette, sv_default_scale_palette};

/// Assigns a colour to every ribbon.
pub(super) fn sv_assign_ribbon_colours(
    ribbons: &mut [SVPreparedRibbon],
    colour_by_count: bool,
    has_colour_column: bool,
) {
    let palette = sv_default_scale_palette();
    if colour_by_count {
        let mut counts: Vec<u64> = ribbons.iter().map(|ribbon| ribbon.count).collect();
        counts.sort_unstable();
        counts.dedup();
        for ribbon in ribbons.iter_mut() {
            let rank = counts
                .iter()
                .position(|count| *count == ribbon.count)
                .unwrap_or(0);
            ribbon.colour = palette.colour_at_index(rank);
        }
        return;
    }

    for ribbon in ribbons.iter_mut() {
        let index = if has_colour_column {
            ribbon.colour_index
        } else {
            ribbon.left_category
        };
        ribbon.colour = sv_palette_colour(&palette, index);
    }
}

fn sv_palette_colour(palette: &SVScalePalette, index: usize) -> crate::sv_scale::SVColour {
    palette.colour_at_index(index)
}
