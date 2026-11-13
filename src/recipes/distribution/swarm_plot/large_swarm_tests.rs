//! A large sample stays linear in the number of drawn observations.

use super::SVSwarmPlot;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnRole, SVDataTableBuilder};

#[test]
fn a_large_swarm_draws_every_observation() {
    const POINT_COUNT: usize = 600;
    let values: Vec<f64> = (0..POINT_COUNT)
        .map(|index| ((index / 6) % 40) as f64)
        .collect();
    let categories: Vec<f64> = (0..POINT_COUNT).map(|index| (index % 6) as f64).collect();
    let table = SVDataTableBuilder::new()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &categories)
        .with_column("series", SVColumnRole::Grouping, &categories)
        .build()
        .expect("the large table is well formed");

    let scene = SVSwarmPlot::builder()
        .data_source(&table)
        .value("value")
        .category("category")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("a large swarm composes");

    assert_eq!(scene.number_of_marks(), POINT_COUNT);
    assert_eq!(scene.number_of_mark_primitives(), POINT_COUNT);
    assert_eq!(scene.number_of_series(), 6);
}
