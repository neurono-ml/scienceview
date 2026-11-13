//! The documented configuration surface: ordering, normalisation, width,
//! orientation, colour, series arrangements, labels and failure paths.

use super::SVCountMissingValuePolicy;
use super::SVCountMultiple;
use super::SVCountPlot;
use super::test_support::{
    CATEGORIES, SVTestSource, sv_build_default, sv_build_series, sv_dense_source, sv_scene_text,
    sv_series_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnRole, SVColumnView, SVDataSource, SVDataTableBuilder};
use crate::sv_error::SVError;
use crate::sv_mark::SVBarOrientation;
use crate::sv_scale::SVColour;
use crate::sv_stat::{SVCountOrder, SVNormalisation};

fn sv_build<'a>(
    source: &'a dyn SVDataSource,
    configure: impl FnOnce(super::SVCountPlotBuilder<'a>) -> super::SVCountPlotBuilder<'a>,
) -> SVCountPlot<'a> {
    let builder = SVCountPlot::builder()
        .data_source(source)
        .category("category")
        .backend(SVBackendKind::Svg);
    configure(builder)
        .build()
        .expect("the configuration is complete")
}

fn sv_rectangles(scene: &super::SVCountScene) -> Vec<crate::sv_layout::SVResolvedRectangle> {
    scene
        .mark_geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives().iter())
        .filter_map(|primitive| match primitive.shape() {
            crate::sv_mark::SVMarkShape::Rectangle(rectangle) => Some(rectangle.rectangle()),
            _ => None,
        })
        .collect()
}

#[test]
fn the_default_configuration_counts_every_category() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the default count plot composes");

    assert_eq!(scene.categories(), &[0.0, 1.0, 2.0]);
    assert_eq!(scene.counts(), &[1.0, 2.0, 3.0]);
    assert_eq!(
        scene.category_labels(),
        &["category 0", "category 1", "category 2"]
    );
    assert_eq!(scene.number_of_categories(), 3);
    assert_eq!(scene.number_of_series(), 1);
    assert_eq!(scene.number_of_bars(), 3);
    assert_eq!(scene.number_of_mark_primitives(), 3);
    assert_eq!(scene.backend_kind(), SVBackendKind::Svg);
}

#[test]
fn supplied_category_labels_replace_the_codes() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| {
        builder.category_labels(&["setosa", "versicolor", "virginica"])
    })
    .compose()
    .expect("the labelled count plot composes");

    assert_eq!(
        scene.category_labels(),
        &["setosa", "versicolor", "virginica"]
    );
    let text = sv_scene_text(&scene);
    assert!(text.contains("setosa"));
    assert!(text.contains("virginica"));
}

#[test]
fn the_ascending_order_sorts_the_categories() {
    const UNSORTED: [f64; 6] = [2.0, 0.0, 2.0, 1.0, 1.0, 2.0];
    let source = SVTestSource::dense(&UNSORTED);
    let scene = sv_build(&source, |builder| builder.order(SVCountOrder::Ascending))
        .compose()
        .expect("the ascending count plot composes");

    assert_eq!(scene.categories(), &[0.0, 1.0, 2.0]);
    assert_eq!(scene.counts(), &[1.0, 2.0, 3.0]);
}

#[test]
fn the_descending_order_reverses_the_categories() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| builder.order(SVCountOrder::Descending))
        .compose()
        .expect("the descending count plot composes");

    assert_eq!(scene.categories(), &[2.0, 1.0, 0.0]);
    assert_eq!(scene.counts(), &[3.0, 2.0, 1.0]);
}

#[test]
fn probability_normalisation_divides_by_the_grand_total() {
    let source = sv_dense_source();
    let plot = sv_build(&source, |builder| {
        builder.normalise(SVNormalisation::Probability)
    });
    let scene = plot.compose().expect("the normalised count plot composes");

    assert_eq!(plot.normalisation(), SVNormalisation::Probability);
    assert_eq!(scene.counts(), &[1.0 / 6.0, 2.0 / 6.0, 3.0 / 6.0]);
    let total: f64 = scene.counts().iter().sum();
    assert!((total - 1.0).abs() < 1.0e-12);
    let text = sv_scene_text(&scene);
    assert!(text.contains("probability"));
}

#[test]
fn the_width_changes_the_bar_geometry() {
    let source = sv_dense_source();
    let wide = sv_build(&source, |builder| builder.width(1.0))
        .compose()
        .expect("the wide count plot composes");
    let narrow = sv_build(&source, |builder| builder.width(0.4))
        .compose()
        .expect("the narrow count plot composes");

    let wide_width: f64 = sv_rectangles(&wide)
        .iter()
        .map(|rectangle| rectangle.width)
        .sum();
    let narrow_width: f64 = sv_rectangles(&narrow)
        .iter()
        .map(|rectangle| rectangle.width)
        .sum();
    assert!(wide_width > narrow_width);
}

#[test]
fn the_horizontal_orientation_swaps_the_axes() {
    let source = sv_dense_source();
    let vertical = sv_build(&source, |builder| builder)
        .compose()
        .expect("the vertical count plot composes");
    let scene = sv_build(&source, |builder| {
        builder.orient(SVBarOrientation::Horizontal)
    })
    .compose()
    .expect("the horizontal count plot composes");

    assert_ne!(vertical.number_of_bars(), 0);
    assert_ne!(
        super::test_support::sv_scene_to_svg(&vertical),
        super::test_support::sv_scene_to_svg(&scene)
    );
    let text = sv_scene_text(&scene);
    assert!(text.contains("category"));
    assert!(text.contains("count"));
    assert_eq!(scene.counts(), &[1.0, 2.0, 3.0]);
    let plot = scene.plot_area();
    assert!(plot.width > plot.height);
}

#[test]
fn a_configured_colour_overrides_the_palette() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| {
        builder.colour(SVColour::new(200, 30, 60, 255))
    })
    .compose()
    .expect("the coloured count plot composes");
    let svg = super::test_support::sv_scene_to_svg(&scene);

    assert!(svg.contains("#c81e3c"));
}

#[test]
fn a_layered_series_draws_one_bar_per_category_and_series() {
    let source = sv_series_source();
    let plot = sv_build_series(&source);
    let scene = plot.compose().expect("the layered count plot composes");

    assert_eq!(scene.number_of_series(), 2);
    assert_eq!(scene.number_of_bars(), 6);
    assert_eq!(scene.counts(), &[1.0, 2.0, 3.0]);
    assert_eq!(scene.series()[0].counts(), &[1.0, 1.0, 1.0]);
    assert_eq!(scene.series()[1].counts(), &[0.0, 1.0, 2.0]);
}

#[test]
fn a_stacked_series_grows_from_the_previous_one() {
    let source = sv_series_source();
    let scene = sv_build_series(&source)
        .compose()
        .expect("the layered scene composes");
    let stacked = sv_build(&source, |builder| {
        builder.series("series").multiple(SVCountMultiple::Stack)
    })
    .compose()
    .expect("the stacked count plot composes");

    assert_eq!(stacked.counts(), scene.counts());
    assert_eq!(stacked.series()[1].counts(), &[0.0, 1.0, 2.0]);
    assert_ne!(
        super::test_support::sv_scene_to_svg(&stacked),
        super::test_support::sv_scene_to_svg(&scene)
    );
}

#[test]
fn a_dodged_series_narrows_every_bar() {
    let source = sv_series_source();
    let layered = sv_build_series(&source)
        .compose()
        .expect("the layered scene composes");
    let dodged = sv_build(&source, |builder| {
        builder.series("series").multiple(SVCountMultiple::Dodge)
    })
    .compose()
    .expect("the dodged count plot composes");

    let layered_width: f64 = sv_rectangles(&layered)
        .iter()
        .map(|rectangle| rectangle.width)
        .sum();
    let dodged_width: f64 = sv_rectangles(&dodged)
        .iter()
        .map(|rectangle| rectangle.width)
        .sum();
    assert!(dodged_width < layered_width);
}

#[test]
fn a_non_finite_category_is_omitted_by_default() {
    const WITH_MISSING: [f64; 4] = [0.0, f64::NAN, 1.0, 1.0];
    let source = SVTestSource::dense(&WITH_MISSING);
    let scene = sv_build_default(&source)
        .compose()
        .expect("the count plot omits the missing category");

    assert_eq!(scene.categories(), &[0.0, 1.0]);
    assert_eq!(scene.counts(), &[1.0, 2.0]);
}

#[test]
fn the_error_policy_rejects_a_non_finite_category() {
    const WITH_MISSING: [f64; 4] = [0.0, f64::NAN, 1.0, 1.0];
    let source = SVTestSource::dense(&WITH_MISSING);
    let error = sv_build(&source, |builder| {
        builder.missing_value_policy(SVCountMissingValuePolicy::Error)
    })
    .compose()
    .expect_err("the error policy rejects the render");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "count_category",
            position: 1,
        }
    );
}

#[test]
fn a_configured_weight_is_rejected_with_the_kernel_gap() {
    let source = sv_dense_source();
    let error = SVCountPlot::builder()
        .data_source(&source)
        .category("category")
        .weight("weight")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the count kernel does not consume weights");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
    assert!(error.to_string().contains("SVStat::Count"));
}

#[test]
fn a_missing_category_option_fails_at_build_time() {
    let source = sv_dense_source();
    let error = SVCountPlot::builder()
        .data_source(&source)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a category is required");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_missing_category_column_fails_at_build_time() {
    let source = sv_dense_source();
    let error = SVCountPlot::builder()
        .data_source(&source)
        .category("absent")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the column does not exist");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

#[test]
fn a_category_column_with_the_wrong_role_fails_at_build_time() {
    let values = [0.0, 1.0, 2.0];
    let table = SVDataTableBuilder::new()
        .with_column("category", SVColumnRole::Y, &values)
        .build()
        .expect("the table is well formed");
    let error = SVCountPlot::builder()
        .data_source(&table)
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a value column is not a category axis");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_missing_backend_fails_at_build_time() {
    let source = sv_dense_source();
    let error = SVCountPlot::builder()
        .data_source(&source)
        .category("category")
        .build()
        .expect_err("a backend is required");

    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn a_missing_data_source_fails_at_build_time() {
    let error = SVCountPlot::builder()
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a data source is required");

    assert_eq!(error, SVError::MissingDataSource);
}

#[test]
fn mismatched_category_and_series_lengths_fail_at_build_time() {
    const SHORT_SERIES: [f64; 3] = [0.0, 1.0, 0.0];
    let source = SVTestSource {
        category: SVColumnView::Dense(&CATEGORIES),
        series: Some(SVColumnView::Dense(&SHORT_SERIES)),
    };
    let error = SVCountPlot::builder()
        .data_source(&source)
        .category("category")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the columns disagree in length");

    assert_eq!(
        error,
        SVError::MismatchedLengths {
            name: "series".to_string(),
            expected: 6,
            actual: 3,
        }
    );
}

#[test]
fn an_empty_category_column_fails_at_compose_time() {
    const EMPTY: [f64; 0] = [];
    let source = SVTestSource::dense(&EMPTY);
    let plot = sv_build_default(&source);
    let error = plot.compose().expect_err("nothing to count");

    assert_eq!(error, SVError::EmptyInput { operation: "count" });
}

#[test]
fn the_title_and_axis_labels_reach_the_decorations() {
    let source = sv_dense_source();
    let scene = sv_build(&source, |builder| {
        builder
            .title("Observations per species")
            .x_label("species")
            .y_label("number of observations")
    })
    .compose()
    .expect("the labelled count plot composes");
    let text = sv_scene_text(&scene);

    assert!(text.contains("Observations per species"));
    assert!(text.contains("species"));
    assert!(text.contains("number of observations"));
}

#[test]
fn the_order_accessor_reports_the_configured_rule() {
    let source = sv_dense_source();
    let plot = sv_build(&source, |builder| builder.order(SVCountOrder::Descending));
    assert_eq!(plot.order(), SVCountOrder::Descending);
}

#[test]
fn every_default_bar_is_a_rectangle_primitive() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the default count plot composes");
    let rectangles = sv_rectangles(&scene);
    assert_eq!(rectangles.len(), 3);
    assert!(
        rectangles
            .iter()
            .all(|rectangle| rectangle.width > 0.0 && rectangle.height > 0.0)
    );
}
