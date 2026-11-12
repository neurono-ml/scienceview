//! Concurrency: the same built chart composed from several threads.

use super::SVRegressionPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};

const X: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
const Y: [f64; 5] = [1.0, 3.0, 5.0, 7.0, 9.0];

#[test]
fn the_same_chart_composes_concurrently_with_identical_results() {
    let table = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &X)
        .with_column("y", SVColumnRole::Y, &Y)
        .build()
        .expect("the test table is well formed");
    let chart = SVRegressionPlotBuilder::new()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is valid");

    let expected = chart.compose().expect("the reference fit runs");
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8).map(|_| scope.spawn(|| chart.compose())).collect();
        for handle in handles {
            let composition = handle
                .join()
                .expect("the thread does not panic")
                .expect("the fit runs");
            assert_eq!(composition, expected);
        }
    });
}
