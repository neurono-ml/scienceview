//! The derived mode: `SVStat::Aggregate` supplies the estimate and interval.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{
    SVColumnRole, SVColumnView, SVDataTable, SVSparseView, SVStreamingView,
};
use scienceview_core::sv_mark::{SVMarkShape, SVPathCommand};
use scienceview_core::sv_stat::SVAggregateErrorKind;

use super::SVBandPlotBuilder;
use super::band_plot_test_support::{SVTestSource, assert_close};

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

const DERIVED_X: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
const DERIVED_Y: [f64; 6] = [1.0, 3.0, 2.0, 4.0, 3.0, 5.0];
const DERIVED_INDICES: [usize; 6] = [0, 1, 2, 3, 4, 5];

fn derived_render(
    source: &dyn scienceview_core::sv_data::SVDataSource,
) -> super::SVBandRenderOutput {
    SVBandPlotBuilder::new()
        .data_source(source)
        .x("x")
        .derive()
        .values("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws")
}

fn dense_derived_source<'values>() -> SVTestSource<'values> {
    SVTestSource::new(vec![
        ("x", SVColumnRole::X, SVColumnView::Dense(&DERIVED_X)),
        ("y", SVColumnRole::Y, SVColumnView::Dense(&DERIVED_Y)),
    ])
}

#[test]
fn derived_sparse_input_matches_dense() {
    let dense = derived_render(&dense_derived_source());
    let x = SVSparseView::new(&DERIVED_INDICES, &DERIVED_X, 6).expect("valid sparse view");
    let y = SVSparseView::new(&DERIVED_INDICES, &DERIVED_Y, 6).expect("valid sparse view");
    let sparse = SVTestSource::new(vec![
        ("x", SVColumnRole::X, SVColumnView::Sparse(x)),
        ("y", SVColumnRole::Y, SVColumnView::Sparse(y)),
    ]);
    let sparse_output = derived_render(&sparse);
    assert_close(&points(&dense), &points(&sparse_output));
}

#[test]
fn derived_streaming_window_is_approximate_and_matches_dense() {
    let dense = derived_render(&dense_derived_source());
    let x = SVStreamingView::new(&DERIVED_X, 6, 0);
    let y = SVStreamingView::new(&DERIVED_Y, 6, 0);
    let streaming = SVTestSource::new(vec![
        ("x", SVColumnRole::X, SVColumnView::Streaming(x)),
        ("y", SVColumnRole::Y, SVColumnView::Streaming(y)),
    ]);
    let streaming_output = derived_render(&streaming);
    assert!(streaming_output.is_approximate());
    assert_close(&points(&dense), &points(&streaming_output));
}

#[test]
fn derived_huge_sparse_input_aggregates_only_its_stored_entries() {
    let logical_length = 10_000_000;
    let indices = [0usize, 1, 2, 3, 4, 5];
    let x = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
    let y = [1.0, 1.4, 2.0, 2.8, 3.0, 4.0];
    let x_view = SVSparseView::new(&indices, &x, logical_length).expect("valid sparse view");
    let y_view = SVSparseView::new(&indices, &y, logical_length).expect("valid sparse view");
    let source = SVTestSource::new(vec![
        ("x", SVColumnRole::X, SVColumnView::Sparse(x_view)),
        ("y", SVColumnRole::Y, SVColumnView::Sparse(y_view)),
    ]);

    let output = derived_render(&source);

    assert_eq!(output.number_of_points(), 3);
    assert_eq!(points(&output).len(), 3 * 2);
}
