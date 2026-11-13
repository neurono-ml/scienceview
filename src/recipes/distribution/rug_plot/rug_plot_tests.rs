//! The built rug plot: defaults, tick placement, missing values and errors.

use super::SVRugPlot;
use super::test_support::{
    SVTestRugSource, VALUES, sv_build_default, sv_dense_source, sv_scene_text, sv_series_source,
    sv_tick_segments,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema};
use crate::sv_error::SVError;
use crate::sv_mark::SVBarOrientation;

#[test]
fn the_default_configuration_draws_one_tick_per_observation() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.orientation(), SVBarOrientation::Vertical);
    assert!((plot.height() - 0.05).abs() < 1e-9);
    assert!((plot.alpha() - 1.0).abs() < 1e-9);
    assert!(plot.line_width().is_none());
    assert_eq!(plot.observation_count(), VALUES.len());

    let scene = plot.compose().expect("the default rug composes");
    assert_eq!(scene.number_of_marks(), VALUES.len());
    assert_eq!(sv_tick_segments(&scene).len(), VALUES.len());
}

#[test]
fn vertical_ticks_sit_in_the_bottom_margin_and_never_enter_the_plot() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the rug composes");
    let plot = scene.plot_area();
    let viewport = scene.viewport();
    let expected_length = 0.05 * plot.height;

    let segments = sv_tick_segments(&scene);
    assert_eq!(segments.len(), VALUES.len());
    for (start, end) in segments {
        assert!(
            (start.x() - end.x()).abs() < 1e-9,
            "a vertical rug tick is a vertical segment"
        );
        assert!((start.y() - (viewport.height() - 6.0)).abs() < 1e-9);
        assert!((start.y() - end.y() - expected_length).abs() < 1e-6);
        assert!(start.x() >= plot.x && start.x() <= plot.right());
        assert!(
            end.y() > plot.bottom(),
            "the tick stays below the plot's data region"
        );
    }
}

#[test]
fn the_ticks_are_ordered_by_value() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the rug composes");
    let positions: Vec<f64> = sv_tick_segments(&scene)
        .iter()
        .map(|segment| segment.0.x())
        .collect();

    assert!(
        positions.windows(2).all(|window| window[0] <= window[1]),
        "the ticks keep the observation order and VALUES is ascending"
    );
}

#[test]
fn non_finite_observations_are_skipped() {
    let values = [1.0, f64::NAN, 2.0, f64::INFINITY, 3.0, f64::NEG_INFINITY];
    let source = SVTestRugSource::dense(&values);
    let scene = sv_build_default(&source)
        .compose()
        .expect("missing observations do not fail the rug");

    assert_eq!(scene.number_of_marks(), 3);
    assert_eq!(sv_tick_segments(&scene).len(), 3);
}

#[test]
fn an_empty_source_renders_an_empty_scene() {
    let values: [f64; 0] = [];
    let source = SVTestRugSource::dense(&values);
    let scene = sv_build_default(&source)
        .compose()
        .expect("an empty rug composes");

    assert_eq!(scene.number_of_marks(), 0);
    assert!(
        scene
            .mark_geometries()
            .iter()
            .all(|geometry| geometry.is_empty())
    );
}

#[test]
fn the_title_and_value_label_become_decorations() {
    let source = sv_dense_source();
    let scene = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .title("Body mass")
        .value_label("grams")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the rug composes");

    assert_eq!(scene.title(), Some("Body mass"));
    let text = sv_scene_text(&scene);
    assert!(text.contains("Body mass"));
    assert!(text.contains("grams"));
}

#[test]
fn the_series_column_splits_the_ticks_into_coloured_groups() {
    let source = sv_series_source();
    let scene = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .series("series")
        .series_labels(&["low", "middle", "high"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the rug composes");

    assert_eq!(scene.number_of_series(), 3);
    assert_eq!(scene.legend_labels(), &["low", "middle", "high"]);
    let text = sv_scene_text(&scene);
    assert!(text.contains("low"));
    assert!(text.contains("high"));
}

#[test]
fn a_missing_value_column_fails_at_build_time() {
    let source = sv_dense_source();
    let error = SVRugPlot::builder()
        .data_source(&source)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a rug needs a value column");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_missing_backend_fails_at_build_time() {
    let source = sv_dense_source();
    let error = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .build()
        .expect_err("a rug needs an explicit backend");

    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn an_unknown_column_fails_at_build_time() {
    let source = sv_dense_source();
    let error = SVRugPlot::builder()
        .data_source(&source)
        .value("absent")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("an unknown column fails");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

struct SVMismatchedSource;

impl SVDataSource for SVMismatchedSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the rug resolves columns by name")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        const VALUES: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
        const SERIES: [f64; 2] = [0.0, 1.0];
        match name {
            "value" => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(&VALUES))),
            "series" => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(&SERIES))),
            _ => Err(SVError::ColumnNotFound {
                name: name.to_string(),
            }),
        }
    }
}

#[test]
fn a_series_of_another_length_fails_at_build_time() {
    let source = SVMismatchedSource;
    let error = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("mismatched columns fail before rendering");

    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}
