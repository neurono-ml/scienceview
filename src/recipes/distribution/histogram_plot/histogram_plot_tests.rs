//! The documented configuration surface: boundaries, normalisation, series,
//! elements, arrangements, orientation and failure paths.

use super::SVBarElement;
use super::SVHistogramMissingValuePolicy;
use super::SVHistogramMultiple;
use super::SVHistogramPlot;
use super::test_support::{
    SVTestSource, VALUES, sv_build_default, sv_dense_source, sv_scene_text, sv_series_source,
    sv_sparse_series_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnView, SVDataTableBuilder, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_mark::{SVBarOrientation, SVMarkPrimitive, SVMarkShape};
use crate::sv_stat::SVNormalisation;

fn sv_build(
    source: &dyn crate::sv_data::SVDataSource,
    configure: impl FnOnce(super::SVHistogramPlotBuilder<'_>) -> super::SVHistogramPlotBuilder<'_>,
) -> SVHistogramPlot<'_> {
    let builder = SVHistogramPlot::builder()
        .data_source(source)
        .value("value")
        .backend(SVBackendKind::Svg);
    configure(builder)
        .build()
        .expect("the configuration is complete")
}

fn sv_rectangles(scene: &super::SVHistogramScene) -> Vec<crate::sv_layout::SVResolvedRectangle> {
    scene
        .mark_geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives().iter())
        .filter_map(SVMarkPrimitive::shape_rectangle)
        .map(|rectangle| rectangle.rectangle())
        .collect()
}

trait SVShapeAccess {
    fn shape_rectangle(&self) -> Option<&crate::sv_mark::SVGeometryRectangle>;
}

impl SVShapeAccess for SVMarkPrimitive<'_> {
    fn shape_rectangle(&self) -> Option<&crate::sv_mark::SVGeometryRectangle> {
        match self.shape() {
            SVMarkShape::Rectangle(rectangle) => Some(rectangle),
            _ => None,
        }
    }
}

#[test]
fn the_default_configuration_bins_the_value_column() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the default histogram composes");

    assert_eq!(scene.number_of_bins(), 3);
    let expected_edges = [1.0, 1.0 + 4.0 / 3.0, 1.0 + 8.0 / 3.0, 5.0];
    assert_eq!(scene.bin_edges().len(), expected_edges.len());
    for (actual, expected) in scene.bin_edges().iter().zip(expected_edges.iter()) {
        assert!((actual - expected).abs() < 1.0e-12);
    }
    assert_eq!(scene.bin_heights(), &[3.0, 3.0, 2.0]);
    assert_eq!(scene.number_of_bars(), 3);
    assert_eq!(scene.number_of_mark_primitives(), 3);
}

#[test]
fn an_explicit_bin_count_sets_the_number_of_equal_bins() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| builder.bin_count(4).common_bins(true))
        .compose()
        .expect("four bins compose");

    assert_eq!(scene.number_of_bins(), 4);
    assert_eq!(scene.bin_edges().len(), 5);
    assert_eq!(scene.bin_edges()[0], 1.0);
    assert_eq!(scene.bin_edges()[4], 5.0);
    assert!((scene.bin_edges()[1] - 2.0).abs() < 1.0e-12);
}

#[test]
fn an_explicit_bin_width_sets_the_edges() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| builder.bin_width(1.0))
        .compose()
        .expect("unit-width bins compose");

    assert_eq!(scene.bin_edges(), &[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(scene.bin_heights(), &[1.0, 2.0, 3.0, 2.0]);
}

#[test]
fn explicit_bin_edges_are_used_verbatim() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| builder.bin_edges(vec![1.0, 3.0, 5.0]))
        .compose()
        .expect("explicit edges compose");

    assert_eq!(scene.bin_edges(), &[1.0, 3.0, 5.0]);
    assert_eq!(scene.bin_heights(), &[3.0, 5.0]);
}

#[test]
fn probability_heights_sum_to_one() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| {
        builder.bin_count(3).normalise(SVNormalisation::Probability)
    })
    .compose()
    .expect("the probability histogram composes");
    let total: f64 = scene.bin_heights().iter().sum();
    assert!((total - 1.0).abs() < 1.0e-12);
}

#[test]
fn density_heights_integrate_to_one() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| {
        builder.bin_count(3).normalise(SVNormalisation::Density)
    })
    .compose()
    .expect("the density histogram composes");
    let integral: f64 = scene
        .bin_heights()
        .iter()
        .zip(scene.bin_edges().windows(2))
        .map(|(height, edges)| height * (edges[1] - edges[0]))
        .sum();
    assert!((integral - 1.0).abs() < 1.0e-12);
}

#[test]
fn cumulative_heights_are_monotone_and_end_at_the_total() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| builder.bin_count(3).cumulative(true))
        .compose()
        .expect("the cumulative histogram composes");

    assert_eq!(scene.bin_heights(), &[3.0, 6.0, 8.0]);
    assert!(
        scene
            .bin_heights()
            .windows(2)
            .all(|pair| pair[0] <= pair[1])
    );
}

#[test]
fn a_series_column_draws_one_group_of_bins_per_value() {
    let source = sv_series_source();
    let scene = sv_build(&source, |builder| {
        builder
            .series("series")
            .series_labels(&["control", "treatment"])
            .bin_count(2)
    })
    .compose()
    .expect("the grouped histogram composes");

    assert_eq!(scene.number_of_series(), 2);
    assert_eq!(scene.number_of_bars(), 4);
    assert_eq!(scene.series()[0].heights(), &[2.0, 2.0]);
    assert_eq!(scene.series()[1].heights(), &[1.0, 3.0]);
    let text = sv_scene_text(&scene);
    assert!(text.contains("control"));
    assert!(text.contains("treatment"));
}

#[test]
fn common_normalisation_shares_one_denominator_across_series() {
    let source = sv_series_source();
    let divided = sv_build(&source, |builder| {
        builder
            .series("series")
            .bin_count(2)
            .normalise(SVNormalisation::Probability)
    })
    .compose()
    .expect("per-series probability composes");
    let shared = sv_build(&source, |builder| {
        builder
            .series("series")
            .bin_count(2)
            .normalise(SVNormalisation::Probability)
            .common_normalisation(true)
    })
    .compose()
    .expect("common probability composes");

    let per_series_total: f64 = divided.series()[0].heights().iter().sum();
    assert!((per_series_total - 1.0).abs() < 1.0e-12);
    let shared_total: f64 = shared
        .series()
        .iter()
        .flat_map(super::SVHistogramSeriesSummary::heights)
        .sum();
    assert!((shared_total - 1.0).abs() < 1.0e-12);
}

#[test]
fn stacking_accumulates_the_series_inside_each_bin() {
    let source = sv_series_source();
    let scene = sv_build(&source, |builder| {
        builder
            .series("series")
            .bin_count(2)
            .multiple(SVHistogramMultiple::Stack)
    })
    .compose()
    .expect("the stacked histogram composes");

    assert_eq!(scene.series()[0].heights(), &[2.0, 2.0]);
    assert_eq!(scene.series()[1].heights(), &[3.0, 5.0]);
}

#[test]
fn dodging_narrows_the_bars_and_shifts_them_sideways() {
    let source = sv_series_source();
    let layered = sv_build(&source, |builder| builder.series("series").bin_count(2))
        .compose()
        .expect("the layered histogram composes");
    let dodged = sv_build(&source, |builder| {
        builder
            .series("series")
            .bin_count(2)
            .multiple(SVHistogramMultiple::Dodge)
    })
    .compose()
    .expect("the dodged histogram composes");

    let layered_rectangles = sv_rectangles(&layered);
    let dodged_rectangles = sv_rectangles(&dodged);
    assert_eq!(layered_rectangles.len(), dodged_rectangles.len());
    let layered_width = layered_rectangles[0].width;
    let dodged_width = dodged_rectangles[0].width;
    assert!(
        dodged_width < layered_width,
        "a dodged bar is narrower than a layered one"
    );
    assert_ne!(
        dodged_rectangles[0].x, dodged_rectangles[2].x,
        "the two dodged series are displaced relative to each other"
    );
    assert_eq!(
        layered_rectangles[0].x, layered_rectangles[2].x,
        "layered series share their slot"
    );
}

#[test]
fn shrink_narrows_every_bar() {
    let source = sv_dense_source();
    let full = sv_build(&source, |builder| builder.bin_count(3))
        .compose()
        .expect("the full histogram composes");
    let shrunk = sv_build(&source, |builder| builder.bin_count(3).shrink(0.5))
        .compose()
        .expect("the shrunk histogram composes");

    let full_width = sv_rectangles(&full)[0].width;
    let shrunk_width = sv_rectangles(&shrunk)[0].width;
    assert!((shrunk_width - full_width / 2.0).abs() < 1.0e-9);
}

#[test]
fn the_step_element_draws_a_path_instead_of_rectangles() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| {
        builder.bin_count(3).element(SVBarElement::Step)
    })
    .compose()
    .expect("the stepped histogram composes");

    assert!(sv_rectangles(&scene).is_empty());
    assert!(
        scene.mark_geometries()[0]
            .primitives()
            .iter()
            .any(|primitive| matches!(primitive.shape(), SVMarkShape::Path(_)))
    );
}

#[test]
fn the_polygon_element_draws_a_filled_area() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| {
        builder.bin_count(3).element(SVBarElement::Poly)
    })
    .compose()
    .expect("the polygon histogram composes");

    assert!(sv_rectangles(&scene).is_empty());
    let primitive = &scene.mark_geometries()[0].primitives()[0];
    assert!(matches!(primitive.shape(), SVMarkShape::Path(_)));
    assert!(primitive.paint().fill().is_some());
}

#[test]
fn a_horizontal_orientation_swaps_the_axes() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| {
        builder
            .bin_count(3)
            .orientation(SVBarOrientation::Horizontal)
    })
    .compose()
    .expect("the horizontal histogram composes");

    assert_eq!(scene.number_of_bars(), 3);
    assert_eq!(sv_rectangles(&scene).len(), 3);
    assert!(sv_scene_text(&scene).contains("count"));
}

#[test]
fn non_finite_values_are_omitted_by_default_and_reported_on_request() {
    const WITH_MISSING: [f64; 4] = [1.0, f64::NAN, 2.0, 3.0];
    let source = SVTestSource::dense(&WITH_MISSING);
    let omitted = sv_build(&source, |builder| builder.bin_count(2))
        .compose()
        .expect("the default policy omits the missing value");
    assert_eq!(omitted.bin_heights().iter().sum::<f64>(), 3.0);

    let error = sv_build(&source, |builder| {
        builder
            .bin_count(2)
            .missing_value_policy(SVHistogramMissingValuePolicy::Error)
    })
    .compose()
    .expect_err("the strict policy rejects the missing value");
    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "histogram_values",
            position: 1,
        }
    );
}

#[test]
fn a_configured_weight_fails_with_the_kernel_gap_named() {
    let source = sv_dense_source();
    let error = SVHistogramPlot::builder()
        .data_source(&source)
        .value("value")
        .weight("weight")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the shipped bin kernel does not consume weights");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
    assert!(format!("{error}").contains("weight"));
}

#[test]
fn impossible_boundaries_fail_at_build_time() {
    let source = sv_dense_source();
    let both = SVHistogramPlot::builder()
        .data_source(&source)
        .value("value")
        .bin_count(2)
        .bin_width(1.0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a width and a count conflict");
    assert!(matches!(both, SVError::InvalidConfiguration { .. }));

    let edges = SVHistogramPlot::builder()
        .data_source(&source)
        .value("value")
        .bin_edges(vec![2.0, 1.0])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("edges must increase");
    assert!(matches!(edges, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_missing_value_column_fails_with_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVHistogramPlot::builder()
        .data_source(&source)
        .value("absent")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the value column is missing");
    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

#[test]
fn the_sparse_series_path_keeps_its_representation() {
    let source = sv_sparse_series_source();
    let plot = sv_build(&source, |builder| builder.series("series").bin_count(2));
    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot
        .compose()
        .expect("the sparse series histogram composes");
    assert_eq!(scene.number_of_bars(), 4);
}

#[test]
fn an_empty_source_reports_an_empty_bin_input() {
    const EMPTY: [f64; 0] = [];
    let table = SVDataTableBuilder::new()
        .with_column("value", crate::sv_data::SVColumnRole::X, &EMPTY)
        .build()
        .expect("the empty table is well formed");
    let plot = SVHistogramPlot::builder()
        .data_source(&table)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    assert_eq!(
        plot.compose().expect_err("no observations to bin"),
        SVError::EmptyInput { operation: "bin" }
    );
}

#[test]
fn a_non_finite_series_code_is_rejected() {
    const BAD_SERIES: [f64; 8] = [0.0, 0.0, f64::INFINITY, 1.0, 0.0, 1.0, 0.0, 1.0];
    let source = SVTestSource {
        value: SVColumnView::Dense(&VALUES),
        series: Some(SVColumnView::Dense(&BAD_SERIES)),
    };
    let error = sv_build(&source, |builder| builder.series("series").bin_count(2))
        .compose()
        .expect_err("a non-finite group code has no identity");
    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "histogram_series",
            position: 2,
        }
    );
}
