//! Rendering the same figure from several threads.

use super::super::SVStepPlot;
use super::test_sources::SVTestSource;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;

#[test]
fn concurrent_compositions_agree() {
    let horizontal: Vec<f64> = (0..2_000).map(|index| index as f64).collect();
    let vertical: Vec<f64> = (0..2_000)
        .map(|index| (index as f64 / 17.0).sin())
        .collect();
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &horizontal)
        .with_dense("y", SVColumnRole::Y, &vertical);
    let plot = SVStepPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds");

    let expected = plot.compose().expect("the plot composes").series().to_vec();

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    plot.compose()
                        .expect("a concurrent composition succeeds")
                        .series()
                        .to_vec()
                })
            })
            .collect();

        for handle in handles {
            assert_eq!(handle.join().expect("the thread does not panic"), expected);
        }
    });
}
