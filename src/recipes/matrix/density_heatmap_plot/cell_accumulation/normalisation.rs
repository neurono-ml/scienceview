//! Normalising the accumulated cell weight.

use crate::sv_stat::SVNormalisation;

pub(super) fn sv_normalise_cells(
    raw: &[f64],
    total: f64,
    x_edges: &[f64],
    y_edges: &[f64],
    normalisation: SVNormalisation,
) -> Vec<f64> {
    if total <= 0.0 {
        return vec![0.0; raw.len()];
    }
    let horizontal = x_edges.len() - 1;
    raw.iter()
        .enumerate()
        .map(|(position, value)| {
            let x_index = position % horizontal;
            let y_index = position / horizontal;
            let width = x_edges[x_index + 1] - x_edges[x_index];
            let height = y_edges[y_index + 1] - y_edges[y_index];
            match normalisation {
                SVNormalisation::Count => *value,
                SVNormalisation::Probability => value / total,
                SVNormalisation::Density => value / (total * width * height),
            }
        })
        .collect()
}
