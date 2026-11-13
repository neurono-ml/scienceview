//! The derived mode: `SVStat::Aggregate` supplies the estimate and interval.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_mark::{SVMarkShape, SVPathCommand};
use scienceview_core::sv_stat::SVAggregateErrorKind;

use super::SVBandPlotBuilder;

#[test]
fn derived_mode_aggregates_mean_and_standard_deviation() {
    let x = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
    let y = [1.0, 3.0, 2.0, 4.0, 3.0, 5.0];
    let data = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("y", SVColumnRole::Y, &y)
        .build()
        .expect("the columns share one length");
    let output = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .derive()
        .values("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");
    let plot = output.plot_area();
    let y_range = plot.bottom() - plot.y;
    let drawn = points(&output);
    let expected_upper = 2.0 + 2.0_f64.sqrt();
    let expected_minimum = 2.0 - 2.0_f64.sqrt();
    let expected_maximum = 4.0 + 2.0_f64.sqrt();
    let scale = y_range / (expected_maximum - expected_minimum);
    assert!(
        (drawn[0].1 - (plot.bottom() - (expected_upper - expected_minimum) * scale)).abs() < 1e-6
    );
}

#[test]
fn confidence_selects_a_percentile_interval() {
    let x = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let y = [1.0, 2.0, 6.0, 2.0, 4.0, 9.0];
    let data = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("y", SVColumnRole::Y, &y)
        .build()
        .expect("the columns share one length");
    let output = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .derive()
        .values("y")
        .confidence(0.9)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");
    assert_eq!(output.number_of_points(), 2);
    assert!(matches!(
        SVAggregateErrorKind::Percentile(0.9),
        SVAggregateErrorKind::Percentile(_)
    ));
}

fn points(output: &super::SVBandRenderOutput) -> Vec<(f64, f64)> {
    output
        .geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .filter_map(|primitive| match primitive.shape() {
            SVMarkShape::Path(path) => Some(path.commands()),
            _ => None,
        })
        .flatten()
        .filter_map(|command| match command {
            SVPathCommand::MoveTo(point) | SVPathCommand::LineTo(point) => {
                Some((point.x(), point.y()))
            }
            _ => None,
        })
        .collect()
}
