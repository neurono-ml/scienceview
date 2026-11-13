//! Clustered heatmap example: builds `SVClusteredHeatmapPlot` and writes
//! `docs/figures/clustered_heatmap_plot.svg`.
//!
//! Run with `cargo run --example clustered_heatmap_plot --features render-svg`.
//!
//! The clustering is a precomputed input. In a real pipeline it comes from
//! `sciencekit`, SciPy, R or a database job; here the small merge trees are
//! written by hand so the example is deterministic and self-contained. The
//! recipe validates the trees and renders them; it never computes them.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_recipes::sv_matrix::sv_clustered_heatmap_plot::{
    SVClusterMerge, SVClusterTree, SVClusteredHeatmapPlot, SVHeatmapMatrix,
};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const ROW_LABELS: [&str; 6] = ["TP53", "EGFR", "BRCA1", "MYC", "KRAS", "PTEN"];
const COLUMN_LABELS: [&str; 5] = ["S1", "S2", "S3", "S4", "S5"];

/// A small, deterministic expression matrix with two clear row groups.
fn expression_value(row: usize, column: usize) -> f64 {
    let first_group = row < 3;
    let trend = column as f64 * 0.4;
    let wave = ((row * 5 + column * 7) % 4) as f64 * 0.15;
    if first_group {
        2.0 + trend + wave
    } else {
        6.5 - trend + wave
    }
}

/// The precomputed row merge tree:
/// `((0, 1), (2, 3)), (4, 5)` — three guessed gene families.
const ROW_MERGES: [SVClusterMerge; 5] = [
    SVClusterMerge::new(0, 1, 0.9, 2),
    SVClusterMerge::new(2, 3, 1.4, 2),
    SVClusterMerge::new(6, 7, 2.6, 4),
    SVClusterMerge::new(4, 5, 3.1, 2),
    SVClusterMerge::new(8, 9, 5.2, 6),
];

/// The precomputed column merge tree: `((0, 1), (2, 3)), 4`.
const COLUMN_MERGES: [SVClusterMerge; 4] = [
    SVClusterMerge::new(0, 1, 0.7, 2),
    SVClusterMerge::new(2, 3, 1.1, 2),
    SVClusterMerge::new(5, 6, 2.4, 4),
    SVClusterMerge::new(7, 4, 4.0, 5),
];

/// The leaf order the precomputed clustering produced, one entry per axis.
const ROW_ORDER: [usize; 6] = [1, 0, 3, 2, 5, 4];
const COLUMN_ORDER: [usize; 5] = [1, 0, 3, 2, 4];

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut values = Vec::with_capacity(ROW_LABELS.len() * COLUMN_LABELS.len());
    for row in 0..ROW_LABELS.len() {
        for column in 0..COLUMN_LABELS.len() {
            values.push(expression_value(row, column));
        }
    }
    let matrix = SVHeatmapMatrix::new(&values, ROW_LABELS.len(), COLUMN_LABELS.len())?;

    let scene = SVClusteredHeatmapPlot::builder()
        .matrix(matrix)
        .row_labels(&ROW_LABELS)
        .column_labels(&COLUMN_LABELS)
        .row_order(&ROW_ORDER)
        .column_order(&COLUMN_ORDER)
        .row_linkage(SVClusterTree::new(&ROW_MERGES))
        .column_linkage(SVClusterTree::new(&COLUMN_MERGES))
        .centre(4.25)
        .colour_bar(true)
        .title("Expression matrix reordered by a supplied clustering")
        .x_label("sample")
        .y_label("gene")
        .backend(SVBackendKind::Svg)
        .build()?
        .compose()?;

    let document = scene.to_svg()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("clustered_heatmap_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
