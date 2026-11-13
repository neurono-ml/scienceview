//! The joint grid of one density-contour cell, derived through `SVStat::Bin`.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_stat::{SVBinOptions, SVNormalisation, SVStat, SVStatInput, SVStatResult};

const SV_PAIR_CONTOUR_BINS: usize = 10;

pub(super) type SVContourGrid = (Vec<f64>, Vec<f64>, Vec<Vec<f64>>);

pub(super) fn sv_joint_grid(xs: &[f64], ys: &[f64]) -> Result<SVContourGrid, SVError> {
    let x_bins = SVStat::bin(
        SVBinOptions::new()
            .bin_count(SV_PAIR_CONTOUR_BINS)
            .normalise(SVNormalisation::Count),
    );
    let y_bins = SVStat::bin(
        SVBinOptions::new()
            .bin_count(SV_PAIR_CONTOUR_BINS)
            .normalise(SVNormalisation::Count),
    );
    let SVStatResult::Bins(x_bins) = x_bins.apply(SVStatInput::new(SVColumnView::Dense(xs)))?
    else {
        return Err(sv_grid_error());
    };
    let SVStatResult::Bins(y_bins) = y_bins.apply(SVStatInput::new(SVColumnView::Dense(ys)))?
    else {
        return Err(sv_grid_error());
    };
    let x_edges = x_bins.edges().to_vec();
    let y_edges = y_bins.edges().to_vec();
    let columns = x_edges.len().saturating_sub(1);
    let rows = y_edges.len().saturating_sub(1);
    let mut counts = vec![vec![0.0; columns]; rows];
    for (x, y) in xs.iter().zip(ys.iter()) {
        let Some(column) = sv_bin_index(&x_edges, *x) else {
            continue;
        };
        let Some(row) = sv_bin_index(&y_edges, *y) else {
            continue;
        };
        if let Some(values) = counts.get_mut(row) {
            if let Some(count) = values.get_mut(column) {
                *count += 1.0;
            }
        }
    }
    Ok((x_edges, y_edges, counts))
}

fn sv_grid_error() -> SVError {
    SVError::InvalidConfiguration {
        message: "the density-contour cell did not produce bins".to_string(),
    }
}

fn sv_bin_index(edges: &[f64], value: f64) -> Option<usize> {
    if value < *edges.first()? || value > *edges.last()? {
        return None;
    }
    let index = edges.partition_point(|edge| *edge <= value);
    Some(index.saturating_sub(1).min(edges.len() - 2))
}
