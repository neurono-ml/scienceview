//! Large-sample and long-series smoke tests.

use super::SVStripPlot;
use super::test_support::sv_scene_to_svg;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnRole, SVDataTableBuilder};

#[test]
fn a_large_sample_composes_one_dot_per_observation() {
    const COUNT: usize = 10_000;
    let values: Vec<f64> = (0..COUNT).map(|index| (index % 100) as f64).collect();
    let categories: Vec<f64> = (0..COUNT).map(|index| (index % 10) as f64).collect();
    let table = SVDataTableBuilder::new()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &categories)
        .build()
        .expect("the table is well formed");

    let scene = SVStripPlot::builder()
        .data_source(&table)
        .value("value")
        .category("category")
        .jitter(0.15)
        .seed(1)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("a large sample composes");

    assert_eq!(scene.number_of_marks(), COUNT);
    assert_eq!(scene.number_of_mark_primitives(), COUNT);
    assert!(sv_scene_to_svg(&scene).starts_with("<svg"));
}
