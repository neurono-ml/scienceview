//! Overplotting: a large scatter stays one compound path per paint.

use super::SVScatterPlot;
use crate::sv_backend::SVBackendKind;

#[test]
fn a_large_scatter_groups_its_circles_into_one_path() {
    const POINT_COUNT: usize = 2_000;
    let horizontal: Vec<f64> = (0..POINT_COUNT).map(|index| index as f64).collect();
    let vertical: Vec<f64> = (0..POINT_COUNT)
        .map(|index| (index as f64 * 0.37).sin())
        .collect();
    let table = crate::sv_data::SVDataTableBuilder::new()
        .with_column("x", crate::sv_data::SVColumnRole::X, &horizontal)
        .with_column("y", crate::sv_data::SVColumnRole::Y, &vertical)
        .build()
        .expect("the large table is well formed");

    let scene = SVScatterPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("a large scatter renders");

    assert_eq!(scene.number_of_marks(), POINT_COUNT);
    assert_eq!(
        scene.number_of_mark_primitives(),
        1,
        "the overplotting policy keeps one compound path per paint"
    );
}
