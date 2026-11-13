//! Density-plot example: two smoothed distributions rendered to
//! `docs/figures/density_plot.svg`.
//!
//! Run with `cargo run --example density_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the recipe
//! composes the data, `SVStat::KernelDensity`, the scales and the marks, and
//! the SVG backend serializes the resulting primitives. The example calls
//! `SVDensityPlot::save`, the same public operation a user program calls, so
//! the figure is never hand-drawn.

use scienceview::sv_recipes::sv_distribution::sv_density_plot::SVDensityPlot;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SAMPLE_COUNT_PER_GROUP: usize = 160;
const GROUP_COUNT: usize = 2;

/// A small deterministic generator, so the figure is identical on every run.
struct SVDeterministicSampler(u64);

impl SVDeterministicSampler {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn uniform(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }

    fn normal(&mut self) -> f64 {
        let first = self.uniform().max(f64::MIN_POSITIVE);
        let second = self.uniform();
        (-2.0 * first.ln()).sqrt() * (2.0 * std::f64::consts::PI * second).cos()
    }
}

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    let directory = std::env::var("CARGO_MANIFEST_DIR")?;
    Ok(PathBuf::from(directory))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sampler = SVDeterministicSampler::new(0x5EED_1234);
    let mut values = Vec::with_capacity(SAMPLE_COUNT_PER_GROUP * GROUP_COUNT);
    let mut groups = Vec::with_capacity(SAMPLE_COUNT_PER_GROUP * GROUP_COUNT);
    for group in 0..GROUP_COUNT {
        let (centre, spread) = match group {
            0 => (0.0, 1.0),
            _ => (2.4, 0.8),
        };
        for _ in 0..SAMPLE_COUNT_PER_GROUP {
            values.push(centre + spread * sampler.normal());
            groups.push(group as f64);
        }
    }

    let table = SVDataTable::builder()
        .with_column("measurement", SVColumnRole::Y, &values)
        .with_column("group", SVColumnRole::Grouping, &groups)
        .build()?;

    let plot = SVDensityPlot::new()
        .data_source(&table)
        .value("measurement")
        .series("group")
        .fill(true)
        .bandwidth_adjustment(1.2)
        .title("Two smoothed distributions")
        .x_title("Measurement")
        .y_title("Density")
        .backend(SVBackendKind::Svg)
        .build()?;

    let root = manifest_directory()?;
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("density_plot.svg");
    plot.save(&destination)?;

    println!("wrote {}", destination.display());
    Ok(())
}
