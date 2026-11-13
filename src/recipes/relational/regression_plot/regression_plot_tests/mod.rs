//! Companion tests for the builder, the configuration and the composition.
//!
//! The module is split by what each file proves: the configuration surface,
//! the composition and its data boundary, and the model coverage.

use super::{SVRegressionPlot, SVRegressionPlotBuilder};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;

const X: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
const Y: [f64; 5] = [1.0, 3.0, 5.0, 7.0, 9.0];

fn table<'a>(x: &'a [f64], y: &'a [f64]) -> SVDataTable<'a> {
    SVDataTable::builder()
        .with_column("x", SVColumnRole::X, x)
        .with_column("y", SVColumnRole::Y, y)
        .build()
        .expect("the test table is well formed")
}

fn plot<'a>(table: &'a SVDataTable<'a>) -> SVRegressionPlot<'a> {
    SVRegressionPlotBuilder::new()
        .data_source(table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is valid")
}

fn build_error(
    builder: SVRegressionPlotBuilder<'_, super::SVWithDataSource, super::SVWithBackend>,
) -> SVError {
    match builder.build() {
        Err(error) => error,
        Ok(_) => panic!("the configuration must fail"),
    }
}

#[path = "composition_tests.rs"]
mod composition_tests;
#[path = "configuration_tests.rs"]
mod configuration_tests;
#[path = "model_tests.rs"]
mod model_tests;
