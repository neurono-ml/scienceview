//! The grid the pair cells occupy, solved by the layout engine.
//!
//! The tree is one vertical row per `y_variables` entry and one horizontal cell
//! per `x_variables` entry; every cell fills its share of the space, so the
//! grid is solved by [`SVLayoutTree`](crate::sv_layout::SVLayoutTree) instead of
//! a hand-computed rectangle loop.

use super::configuration::{
    SV_PAIR_CELL_SPACING, SV_PAIR_LEGEND_HEIGHT, SV_PAIR_MARGIN, SV_PAIR_TITLE_HEIGHT,
    SV_PAIR_VIEWPORT_HEIGHT, SV_PAIR_VIEWPORT_WIDTH,
};
use crate::sv_layout::{
    SVLayoutDirection, SVLayoutNode, SVLayoutTree, SVLength, SVResolvedRectangle, SVViewport,
};

pub(super) struct SVPairGrid {
    pub(super) viewport: SVViewport,
    pub(super) grid_area: SVResolvedRectangle,
    pub(super) cells: Vec<Vec<SVResolvedRectangle>>,
}

pub(super) fn sv_solve_grid(number_of_rows: usize, number_of_columns: usize) -> SVPairGrid {
    let viewport = SVViewport::new(SV_PAIR_VIEWPORT_WIDTH, SV_PAIR_VIEWPORT_HEIGHT);
    let rows: Vec<SVLayoutNode> = (0..number_of_rows)
        .map(|row| {
            let cells: Vec<SVLayoutNode> = (0..number_of_columns)
                .map(|column| {
                    SVLayoutNode::new(format!("cell-{row}-{column}"))
                        .with_width(SVLength::fill())
                        .with_height(SVLength::fill())
                })
                .collect();
            SVLayoutNode::new(format!("row-{row}"))
                .with_direction(SVLayoutDirection::Horizontal)
                .with_spacing(SV_PAIR_CELL_SPACING)
                .with_width(SVLength::fill())
                .with_height(SVLength::fill())
                .with_children(cells)
        })
        .collect();

    let grid = SVLayoutNode::new("pair-grid")
        .with_direction(SVLayoutDirection::Vertical)
        .with_spacing(SV_PAIR_CELL_SPACING)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .with_children(rows);

    let title = SVLayoutNode::new("pair-title")
        .with_width(SVLength::fill())
        .with_height(SVLength::absolute(SV_PAIR_TITLE_HEIGHT));

    let legend = SVLayoutNode::new("pair-legend")
        .with_width(SVLength::fill())
        .with_height(SVLength::absolute(SV_PAIR_LEGEND_HEIGHT));

    let root = SVLayoutNode::new("pair-root")
        .with_direction(SVLayoutDirection::Vertical)
        .with_padding(SV_PAIR_MARGIN)
        .with_spacing(SV_PAIR_CELL_SPACING)
        .with_width(SVLength::fill())
        .with_height(SVLength::fill())
        .child(title)
        .child(grid)
        .child(legend);

    let tree = SVLayoutTree::new(root).with_viewport(viewport);
    let resolved = tree.solve(&viewport);
    let grid_area = resolved
        .find("pair-grid")
        .map(|node| node.rectangle())
        .unwrap_or_else(|| {
            SVResolvedRectangle::new(
                SV_PAIR_MARGIN,
                SV_PAIR_MARGIN,
                SV_PAIR_VIEWPORT_WIDTH - 2.0 * SV_PAIR_MARGIN,
                SV_PAIR_VIEWPORT_HEIGHT - 2.0 * SV_PAIR_MARGIN - SV_PAIR_LEGEND_HEIGHT,
            )
        });

    let cells = (0..number_of_rows)
        .map(|row| {
            (0..number_of_columns)
                .map(|column| {
                    resolved
                        .find(&format!("cell-{row}-{column}"))
                        .map(|node| node.rectangle())
                        .unwrap_or(grid_area)
                })
                .collect()
        })
        .collect();

    SVPairGrid {
        viewport,
        grid_area,
        cells,
    }
}
