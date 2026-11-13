//! Hexbin recipe example.
//!
//! Bins a large synthetic sample into hexagonal cells and writes
//! `docs/figures/hexbin_plot.svg`. The sample is generated deterministically
//! from a small linear congruential generator, so the checked-in figure never
//! drifts. The example composes the public builder and serializes the
//! backend-agnostic scene with the SVG document surface.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_data::{SVColumnRole, SVDataTable};
use scienceview::sv_recipes::sv_matrix::sv_hexbin_plot::SVHexbinPlot;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const SAMPLE_COUNT: usize = 1_200;

fn manifest_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?))
}

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut generator = SVLinearCongruential::new(0x5EED_0BEE);
    let mut horizontal = Vec::with_capacity(SAMPLE_COUNT);
    let mut vertical = Vec::with_capacity(SAMPLE_COUNT);
    for index in 0..SAMPLE_COUNT {
        if index % 3 == 0 {
            horizontal.push(2.0 + 0.9 * generator.next_normal());
            vertical.push(2.0 + 0.9 * generator.next_normal());
        } else if index % 3 == 1 {
            horizontal.push(5.5 + 0.7 * generator.next_normal());
            vertical.push(4.8 + 1.1 * generator.next_normal());
        } else {
            horizontal.push(4.0 + 1.3 * generator.next_normal());
            vertical.push(6.5 + 0.6 * generator.next_normal());
        }
    }

    let table = SVDataTable::builder()
        .with_column("first measurement", SVColumnRole::X, &horizontal)
        .with_column("second measurement", SVColumnRole::Y, &vertical)
        .build()?;

    let plot = SVHexbinPlot::builder()
        .data_source(&table)
        .x("first measurement")
        .y("second measurement")
        .grid_size(28)
        .min_count(2)
        .title("Hexagonal density of two measurements")
        .x_label("first measurement")
        .y_label("second measurement")
        .backend(SVBackendKind::Svg)
        .build()?;

    let document = plot.compose()?.to_svg()?;
    let figures = manifest_directory()?.join("docs").join("figures");
    fs::create_dir_all(&figures)?;
    let destination = figures.join("hexbin_plot.svg");
    fs::write(&destination, document)?;
    println!("Wrote {}", destination.display());
    Ok(())
}
