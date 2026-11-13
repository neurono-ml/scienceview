//! Violin-plot example: the smoothed distribution of three categories, written
//! to `docs/figures/violin_plot.svg`.
//!
//! Run with `cargo run --example violin_plot --features render-svg` from the
//! repository root. The figure is produced by the tool itself: the real
//! `SVViolinPlotBuilder` estimates each density through `SVStat::KernelDensity`
//! and the public `save` operation serializes the resulting geometry.

use scienceview::sv_recipes::sv_distribution::sv_violin_plot::SVViolinPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_layout::SVViewport;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SAMPLE_COUNT: usize = 180;
const CATEGORY_COUNT: usize = 3;

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

fn sample_value(category: usize, sampler: &mut SVDeterministicSampler) -> f64 {
    let centre = 3.0 + category as f64 * 2.2;
    match category {
        1 => {
            let side = if sampler.uniform() < 0.5 { -1.0 } else { 1.0 };
            centre + side * 1.4 + 0.35 * sampler.normal()
        }
        2 => centre + 1.1 * sampler.normal() + 1.8 * (sampler.normal().abs() - 0.8),
        _ => centre + 1.0 * sampler.normal(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sampler = SVDeterministicSampler::new(0x05EE_D117);
    let mut values = Vec::with_capacity(SAMPLE_COUNT * CATEGORY_COUNT);
    let mut categories = Vec::with_capacity(SAMPLE_COUNT * CATEGORY_COUNT);
    for category in 0..CATEGORY_COUNT {
        for _ in 0..SAMPLE_COUNT {
            values.push(sample_value(category, &mut sampler));
            categories.push(category as f64);
        }
    }

    let table = SVDataTable::builder()
        .with_column("measurement", SVColumnRole::Y, &values)
        .with_column("station", SVColumnRole::Grouping, &categories)
        .build()?;

    let plot = SVViolinPlotBuilder::new()
        .data_source(&table)
        .value("measurement")
        .category("station")
        .bandwidth_adjustment(0.9)
        .width(0.72)
        .show_box(true)
        .title("Measurement distribution by station")
        .x_title("station")
        .y_title("measurement")
        .viewport(SVViewport::new(640.0, 400.0))
        .backend(SVBackendKind::Svg)
        .build()?;

    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let figures = root.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("violin_plot.svg");
    plot.save(&destination)?;

    println!("wrote {}", destination.display());
    Ok(())
}
