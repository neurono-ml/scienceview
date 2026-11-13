//! Scatterplot-matrix example: four body measurements rendered to
//! `docs/figures/splom_plot.svg`.
//!
//! Run with `cargo run --example splom_plot --features render-svg` from the
//! repository root. The sample is generated deterministically, the public
//! builder composes the matrix and the SVG backend serializes the resulting
//! scene, so the checked-in figure is tool-generated and never hand-drawn.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTable};
use scienceview::sv_recipes::sv_multivariate::sv_splom_plot::{
    SVSplomDiagonal, SVSplomPlot, SVSplomScaleMode,
};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SAMPLE_COUNT: usize = 240;
const GROUP_COUNT: usize = 3;

/// A tiny deterministic generator; the figure must not depend on entropy.
struct SVLinearCongruential {
    state: u64,
}

impl SVLinearCongruential {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_unit(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.state >> 11) as f64) / ((1u64 << 53) as f64)
    }

    fn next_normal(&mut self) -> f64 {
        let first = self.next_unit().max(f64::MIN_POSITIVE);
        let second = self.next_unit();
        (-2.0 * first.ln()).sqrt() * (std::f64::consts::TAU * second).cos()
    }
}

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut generator = SVLinearCongruential::new(0x5EED_5A1A);
    let mut height = Vec::with_capacity(SAMPLE_COUNT);
    let mut weight = Vec::with_capacity(SAMPLE_COUNT);
    let mut age = Vec::with_capacity(SAMPLE_COUNT);
    let mut pulse = Vec::with_capacity(SAMPLE_COUNT);
    let mut group = Vec::with_capacity(SAMPLE_COUNT);

    for index in 0..SAMPLE_COUNT {
        let group_index = index % GROUP_COUNT;
        let group_shift = group_index as f64 * 4.0;
        let person_height = 168.0 + group_shift + 8.0 * generator.next_normal();
        let person_weight = 0.42 * person_height + 12.0 * generator.next_normal();
        let person_age = 28.0 + group_shift * 2.0 + 7.0 * generator.next_normal();
        let person_pulse = 74.0 - 0.18 * (person_height - 168.0) + 6.0 * generator.next_normal();
        height.push(person_height);
        weight.push(person_weight);
        age.push(person_age);
        pulse.push(person_pulse);
        group.push(group_index as f64);
    }

    let table = SVDataTable::builder()
        .with_column("height", SVColumnRole::Y, &height)
        .with_column("weight", SVColumnRole::Y, &weight)
        .with_column("age", SVColumnRole::Y, &age)
        .with_column("resting pulse", SVColumnRole::Y, &pulse)
        .with_column("group", SVColumnRole::Grouping, &group)
        .build()?;

    let plot = SVSplomPlot::builder()
        .data_source(&table)
        .variables(&["height", "weight", "age", "resting pulse"])
        .colour("group")
        .diagonal(SVSplomDiagonal::Density)
        .scale_mode(SVSplomScaleMode::Independent)
        .cell_aspect(1.0)
        .opacity(0.72)
        .title("Body measurements by group")
        .backend(SVBackendKind::Svg)
        .build()?;

    let document = plot.compose()?.to_svg()?;
    let figures = manifest_directory()?.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("splom_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
