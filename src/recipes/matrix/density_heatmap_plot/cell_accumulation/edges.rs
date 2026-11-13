//! Deriving the bin edges through the bin kernel and locating a cell.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_stat::{SVBinOptions, SVStat, SVStatInput, SVStatResult};

/// Derives the edges of one axis through [`SVStat::Bin`](crate::sv_stat::SVStat::Bin).
pub(super) fn sv_bin_edges(
    view: SVColumnView<'_>,
    bin_count: usize,
    operation: &'static str,
) -> Result<Vec<f64>, SVError> {
    let stat = SVStat::bin(SVBinOptions::new().bin_count(bin_count));
    match stat.apply(SVStatInput::new(view))? {
        SVStatResult::Bins(bins) => Ok(bins.edges().to_vec()),
        _ => Err(SVError::InvalidConfiguration {
            message: format!("the {operation} composition expects the bin kernel"),
        }),
    }
}

/// Returns the flat row-major index of the cell holding `(x_value, y_value)`.
pub(super) fn sv_cell_index(x_edges: &[f64], y_edges: &[f64], x_value: f64, y_value: f64) -> usize {
    let horizontal = x_edges.len() - 1;
    sv_bin_index(x_edges, x_value) + sv_bin_index(y_edges, y_value) * horizontal
}

fn sv_bin_index(edges: &[f64], value: f64) -> usize {
    let last = edges.len() - 2;
    edges
        .partition_point(|edge| *edge <= value)
        .saturating_sub(1)
        .min(last)
}
