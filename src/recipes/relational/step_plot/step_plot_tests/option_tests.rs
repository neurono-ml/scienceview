//! The documented defaults and each option that changes the output.

use super::super::{SVStepPlot, SVStepPosition};
use super::test_sources::SVTestSource;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::SVDashPattern;
use scienceview_core::sv_scale::SVScaleKind;

const HORIZONTAL: [f64; 3] = [1.0, 2.0, 3.0];
const VERTICAL: [f64; 3] = [10.0, 20.0, 30.0];

fn source<'a>(horizontal: &'a [f64], vertical: &'a [f64]) -> SVTestSource<'a> {
    SVTestSource::new()
        .with_dense("time", SVColumnRole::X, horizontal)
        .with_dense("value", SVColumnRole::Y, vertical)
}

fn build<'a>(source: &'a SVTestSource<'a>) -> SVStepPlot<'a> {
    SVStepPlot::builder()
        .data_source(source)
        .x("time")
        .y("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
}

#[test]
fn the_documented_defaults_are_applied() {
    let table = source(&HORIZONTAL, &VERTICAL);
    let plot = build(&table);

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.execution_mode(), SVExecutionMode::Automatic);
    assert_eq!(plot.step_position(), SVStepPosition::After);
    assert!(plot.markers());
    assert!(plot.line_width().is_none());
    assert!(plot.dash().is_none());
    assert!(plot.aggregate().is_none());
    assert_eq!(plot.viewport().width(), 640.0);
    assert_eq!(plot.viewport().height(), 400.0);
}

#[test]
fn each_step_position_expands_different_vertices() {
    let table = source(&HORIZONTAL, &VERTICAL);

    for position in SVStepPosition::ALL {
        let horizontal = match position {
            SVStepPosition::After => vec![1.0, 2.0, 2.0, 3.0, 3.0],
            SVStepPosition::Before => vec![1.0, 1.0, 2.0, 2.0, 3.0],
            SVStepPosition::Center => vec![1.0, 1.5, 1.5, 2.5, 2.5, 3.0],
        };
        let plot = SVStepPlot::builder()
            .data_source(&table)
            .x("time")
            .y("value")
            .step(position)
            .backend(SVBackendKind::Svg)
            .build()
            .expect("the documented configuration builds");
        let composition = plot.compose().expect("the plot composes");
        assert_eq!(
            composition.series()[0].horizontal(),
            horizontal,
            "{position}"
        );
    }
}

#[test]
fn markers_change_the_number_of_geometries() {
    let table = source(&HORIZONTAL, &VERTICAL);

    let with_markers = build(&table).compose().expect("the plot composes");
    assert_eq!(
        with_markers
            .mark_geometries()
            .expect("lines and dots")
            .len(),
        2
    );

    let without_markers = SVStepPlot::builder()
        .data_source(&table)
        .x("time")
        .y("value")
        .markers(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
        .compose()
        .expect("the plot composes");
    assert_eq!(
        without_markers.mark_geometries().expect("lines only").len(),
        1
    );
}

#[test]
fn line_width_and_dash_reach_the_mark_paint() {
    let table = source(&HORIZONTAL, &VERTICAL);
    let composition = SVStepPlot::builder()
        .data_source(&table)
        .x("time")
        .y("value")
        .line_width(SVLength::absolute(3.0))
        .dash(SVDashPattern::Dashed)
        .markers(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
        .compose()
        .expect("the plot composes");

    let geometries = composition.mark_geometries().expect("the line draws");
    let paint = geometries[0].primitives()[0].paint();
    assert_eq!(paint.stroke_width(), 3.0);
    assert_eq!(paint.dash(), &SVDashPattern::Dashed);
}

#[test]
fn markers_sit_on_the_samples_not_the_expanded_corners() {
    let table = source(&HORIZONTAL, &VERTICAL);
    let composition = build(&table).compose().expect("the plot composes");
    assert_eq!(composition.series()[0].sample_horizontal(), &HORIZONTAL[..]);

    let geometries = composition.mark_geometries().expect("lines and dots");
    assert_eq!(geometries[1].primitives().len(), HORIZONTAL.len());
}

#[test]
fn a_temporal_x_scale_is_accepted() {
    let table = source(&HORIZONTAL, &VERTICAL);
    let plot = SVStepPlot::builder()
        .data_source(&table)
        .x("time")
        .y("value")
        .x_scale(SVScaleKind::Temporal)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("a temporal x axis is documented");
    assert_eq!(plot.x_scale_kind(), SVScaleKind::Temporal);
    let composition = plot.compose().expect("the temporal plot composes");
    assert_eq!(composition.horizontal_scale_kind(), SVScaleKind::Continuous);
    assert_eq!(composition.vertical_scale_kind(), SVScaleKind::Continuous);
    assert!(
        !composition
            .mark_geometries()
            .expect("the temporal projection draws")
            .is_empty()
    );
    #[cfg(feature = "render-svg")]
    assert!(plot.render().is_ok());
}

#[test]
fn a_title_is_carried_by_the_composition() {
    let table = source(&HORIZONTAL, &VERTICAL);
    let composition = SVStepPlot::builder()
        .data_source(&table)
        .x("time")
        .y("value")
        .title("A sampled signal")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
        .compose()
        .expect("the plot composes");

    assert_eq!(composition.title(), Some("A sampled signal"));
    assert!(!composition.decorations_before_marks().is_empty());
    assert!(!composition.decorations_after_marks().is_empty());
}
