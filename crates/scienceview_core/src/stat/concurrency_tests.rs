//! Companion tests for the stat kernel under concurrency.

use std::thread;

use crate::sv_data::SVColumnView;
use crate::sv_stat::{
    SVAggregateOptions, SVBinOptions, SVCountOptions, SVEmpiricalCdfOptions, SVFitOptions,
    SVKernelDensityOptions, SVPercentileOptions, SVStat, SVStatInput, SVStatResult,
};

fn dataset() -> Vec<f64> {
    (0..256).map(|position| (position % 17) as f64).collect()
}

#[test]
fn every_stat_runs_concurrently_over_the_same_borrowed_columns() {
    let values = dataset();
    let secondary = dataset();
    let stats = [
        SVStat::bin(SVBinOptions::new().bin_count(8)),
        SVStat::count(SVCountOptions::new()),
        SVStat::aggregate(SVAggregateOptions::new()),
        SVStat::kernel_density(SVKernelDensityOptions::new().grid_count(32)),
        SVStat::empirical_cdf(SVEmpiricalCdfOptions::new()),
        SVStat::percentile(SVPercentileOptions::new()),
    ];

    let expected: Vec<SVStatResult> = stats
        .iter()
        .map(|stat| {
            stat.apply(
                SVStatInput::new(SVColumnView::Dense(&values))
                    .with_secondary(SVColumnView::Dense(&secondary)),
            )
            .expect("the calling thread applies the stat")
        })
        .collect();

    thread::scope(|scope| {
        for _ in 0..4 {
            let stats = &stats;
            let expected = &expected;
            let values = values.as_slice();
            let secondary = secondary.as_slice();
            scope.spawn(move || {
                for (stat, expected) in stats.iter().zip(expected.iter()) {
                    let input = SVStatInput::new(SVColumnView::Dense(values))
                        .with_secondary(SVColumnView::Dense(secondary));
                    let result = stat.apply(input).expect("each thread applies the stat");
                    assert_eq!(
                        &result, expected,
                        "every thread produces the same result as the calling thread"
                    );
                }
            });
        }
    });
}

#[test]
fn a_heavy_stat_can_run_on_a_scoped_compute_thread() {
    let values = dataset();
    let stat = SVStat::kernel_density(SVKernelDensityOptions::new().grid_count(64));

    assert!(stat.is_heavy());

    thread::scope(|scope| {
        let handle = stat.spawn_on(scope, SVStatInput::new(SVColumnView::Dense(&values)));
        let direct = stat
            .apply(SVStatInput::new(SVColumnView::Dense(&values)))
            .expect("the calling thread applies the same stat");
        let spawned = handle
            .join()
            .expect("the compute thread did not panic")
            .expect("the compute thread applied the stat");

        assert_eq!(direct, spawned);
    });
}

#[test]
fn a_fit_runs_concurrently_and_off_the_calling_thread() {
    let independent = dataset();
    let values: Vec<f64> = independent.iter().map(|x| 2.0 * x + 1.0).collect();
    let stat = SVStat::fit(SVFitOptions::new());

    assert!(stat.is_heavy());

    thread::scope(|scope| {
        let handle = stat.spawn_on(
            scope,
            SVStatInput::new(SVColumnView::Dense(&values))
                .with_secondary(SVColumnView::Dense(&independent)),
        );
        let spawned = handle
            .join()
            .expect("the compute thread did not panic")
            .expect("the compute thread applied the fit");

        assert!(!spawned.is_windowed());
    });
}
