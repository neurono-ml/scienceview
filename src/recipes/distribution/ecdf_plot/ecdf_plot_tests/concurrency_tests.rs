//! Rendering the same figure from several threads.

use super::super::SVECdfPlot;
use super::test_support::SVTestSource;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;

#[test]
fn concurrent_compositions_agree() {
    let values: Vec<f64> = (0..2_000)
        .map(|index| (index as f64 / 17.0).sin())
        .collect();
    let series: Vec<f64> = (0..2_000)
        .map(|index| if index % 2 == 0 { 1.0 } else { 2.0 })
        .collect();
    let table = SVTestSource::new()
        .with_dense("value", SVColumnRole::Y, &values)
        .with_dense("scenario", SVColumnRole::Grouping, &series);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .series("scenario")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds");

    let expected = plot.compose().expect("the plot composes");

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| plot.compose().expect("a concurrent composition succeeds")))
            .collect();

        for handle in handles {
            assert_eq!(handle.join().expect("the thread does not panic"), expected);
        }
    });
}
