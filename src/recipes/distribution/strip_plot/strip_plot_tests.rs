//! The built figure, its defaults and the composed scene.

use super::test_support::{
    CATEGORIES, SVTestStripSource, VALUES, sv_build_default, sv_dense_source, sv_dot_centres,
    sv_scene_text, sv_series_source,
};
use super::{SVStripPlot, SVStripScene};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema, SVStreamingView,
};
use crate::sv_error::SVError;
use crate::sv_mark::{SVBarOrientation, SVDotShape};
use crate::sv_move::SVJitterMode;

#[test]
fn the_default_configuration_draws_one_dot_per_observation() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the default strip plot composes");

    assert_eq!(scene.number_of_marks(), VALUES.len());
    assert_eq!(scene.number_of_series(), 1);
    assert_eq!(scene.orientation(), SVBarOrientation::Vertical);
    assert_eq!(scene.number_of_mark_primitives(), VALUES.len());
    assert_eq!(sv_dot_centres(&scene).len(), VALUES.len());
}

#[test]
fn the_backend_and_execution_mode_are_carried_by_the_built_figure() {
    let source = sv_dense_source();
    let plot = SVStripPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");

    assert_eq!(plot.backend_kind(), SVBackendKind::Native);
    assert_eq!(
        plot.execution_mode(),
        crate::sv_figure::SVExecutionMode::Automatic
    );
    assert_eq!(plot.orientation(), SVBarOrientation::Vertical);
    assert_eq!(plot.jitter(), 0.1);
    assert_eq!(plot.jitter_mode(), SVJitterMode::Uniform);
    assert_eq!(plot.seed(), None);
    assert_eq!(plot.marker(), SVDotShape::Circle);
    assert_eq!(plot.alpha(), 1.0);
    assert_eq!(plot.size(), None);
    assert_eq!(plot.observation_count(), VALUES.len());
}

#[test]
fn every_observation_is_drawn_exactly_once() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the strip plot composes");
    let centres = sv_dot_centres(&scene);

    assert_eq!(centres.len(), scene.number_of_marks());
    for value in VALUES {
        let drawn = centres
            .iter()
            .filter(|centre| centre.y() < 430.0 && centre.y() > 0.0)
            .count();
        assert!(drawn > 0, "the sample draws {value}");
    }
}

#[test]
fn a_seeded_jitter_is_reproducible_across_compositions() {
    let source = sv_dense_source();
    let plot = SVStripPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .jitter(0.2)
        .seed(7)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let first = sv_dot_centres(&plot.compose().expect("the first composition succeeds"));
    let second = sv_dot_centres(&plot.compose().expect("the second composition succeeds"));

    assert_eq!(first, second, "the seeded jitter never drifts");
}

#[test]
fn the_jitter_offset_follows_the_match_key_and_not_the_value() {
    // The same categories and the same row order, with different values: the
    // horizontal offsets must stay put because the jitter is keyed by the row.
    const OTHER_VALUES: [f64; 9] = [9.0, 8.0, 7.5, 6.5, 6.0, 5.0, 4.5, 4.0, 3.5];
    let first = SVTestStripSource::dense(&VALUES, &CATEGORIES);
    let second = SVTestStripSource::dense(&OTHER_VALUES, &CATEGORIES);
    let first_plot = SVStripPlot::builder()
        .data_source(&first)
        .value("value")
        .category("category")
        .jitter(0.3)
        .seed(11)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let second_plot = SVStripPlot::builder()
        .data_source(&second)
        .value("value")
        .category("category")
        .jitter(0.3)
        .seed(11)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let first_centres = sv_dot_centres(&first_plot.compose().expect("composes"));
    let second_centres = sv_dot_centres(&second_plot.compose().expect("composes"));

    for (first, second) in first_centres.iter().zip(second_centres.iter()) {
        assert!(
            (first.x() - second.x()).abs() < 1e-9,
            "the jitter offset depends on the match key, not the value"
        );
    }
}

#[test]
fn a_missing_category_or_value_composes_an_empty_scene_without_failing() {
    let values = [1.0, f64::NAN, 3.0];
    let categories = [0.0, 0.0, 0.0];
    let source = SVTestStripSource::dense(&values, &categories);
    let scene = sv_build_default(&source)
        .compose()
        .expect("a missing observation is omitted by default");

    assert_eq!(scene.number_of_marks(), 2);
}

#[test]
fn the_strict_missing_value_policy_rejects_a_non_finite_observation() {
    let values = [1.0, f64::INFINITY, 3.0];
    let categories = [0.0, 0.0, 0.0];
    let source = SVTestStripSource::dense(&values, &categories);
    let error = SVStripPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .missing_value_policy(super::SVStripMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect_err("the strict policy rejects the render");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "strip plot",
            position: 1,
        }
    );
}

#[test]
fn a_missing_required_column_fails_with_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVStripPlot::builder()
        .data_source(&source)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the category column is required");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_absent_column_fails_with_column_not_found() {
    let source = sv_dense_source();
    let error = SVStripPlot::builder()
        .data_source(&source)
        .value("absent")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a missing column fails at build time");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string(),
        }
    );
}

#[test]
fn columns_of_different_lengths_fail_at_build_time() {
    const SHORT: [f64; 3] = [0.0, 1.0, 2.0];
    let source = SVTestStripSource::dense(&VALUES, &SHORT);
    let error = SVStripPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("mismatched columns fail before rendering");

    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn a_series_column_groups_the_dots_and_labels_the_legend() {
    let source = sv_series_source();
    let scene = SVStripPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .series("series")
        .series_labels(&["first", "second"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the series strip plot composes");

    assert_eq!(scene.number_of_series(), 2);
    assert_eq!(scene.legend_labels(), ["first", "second"]);
    assert!(sv_scene_text(&scene).contains("first"));
}

#[test]
fn a_streaming_column_shows_only_its_current_window() {
    let window = &VALUES[4..];
    let categories = &CATEGORIES[4..];
    let source = SVTestStripSource {
        values: SVColumnView::Streaming(SVStreamingView::new(window, 9, 4)),
        categories: SVColumnView::Streaming(SVStreamingView::new(categories, 9, 4)),
        series: None,
    };
    let plot = sv_build_default(&source);
    let scene = plot.compose().expect("the current window composes");

    assert_eq!(scene.number_of_marks(), window.len());
    assert_eq!(plot.observation_count(), window.len());
}

#[test]
fn the_scene_only_serializes_through_its_configured_kind() {
    let source = sv_dense_source();
    let scene: SVStripScene = sv_build_default(&source)
        .compose()
        .expect("the SVG scene composes");
    assert_eq!(scene.backend_kind(), SVBackendKind::Svg);
}

/// A source whose column exists without a declared schema entry.
struct SVUndeclaredSource {
    values: SVColumnView<'static>,
    categories: SVColumnView<'static>,
}

impl SVDataSource for SVUndeclaredSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(Vec::new())
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        match name {
            "value" => Ok(SVColumnSnapshot::Borrowed(self.values)),
            "category" => Ok(SVColumnSnapshot::Borrowed(self.categories)),
            _ => Err(SVError::ColumnNotFound {
                name: name.to_string(),
            }),
        }
    }
}

#[test]
fn a_schema_without_the_configured_columns_fails_explicitly() {
    let source = SVUndeclaredSource {
        values: SVColumnView::Dense(&VALUES),
        categories: SVColumnView::Dense(&CATEGORIES),
    };
    let error = SVStripPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("an undeclared column is rejected by the schema check");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "value".to_string(),
        }
    );
}
