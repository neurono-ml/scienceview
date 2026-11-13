//! The documented defaults, the composition and the collision guarantee.

use super::test_support::{
    CATEGORY, SVTestSource, VALUES, circle_centres, default_builder, dense_source, render, scene,
    scene_svg, scene_text,
};
use super::{SVSwarmPlot, SVSwarmPlotBuilder};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnRole, SVColumnView, SVDataTableBuilder};
use crate::sv_error::SVError;
use crate::sv_mark::SVBarOrientation;

#[test]
fn the_default_configuration_draws_one_dot_per_observation() {
    let source = dense_source();
    let scene = default_builder(&source)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");

    assert_eq!(scene.number_of_marks(), VALUES.len());
    assert_eq!(scene.number_of_mark_primitives(), VALUES.len());
    assert_eq!(scene.category_labels().len(), 2);
    assert_eq!(scene.orientation(), SVBarOrientation::Vertical);
}

#[test]
fn well_separated_values_stay_on_their_category_axis() {
    let scene = scene(&dense_source());
    let centres = circle_centres(&scene);

    for (index, (_, y)) in centres.iter().enumerate() {
        let value = VALUES[index];
        assert!(y.is_finite(), "every dot has a position for value {value}");
    }
    // The five observations of a category share one x when nothing collides.
    let first: Vec<f64> = centres[..5].iter().map(|centre| centre.0).collect();
    assert!(first.iter().all(|x| (x - first[0]).abs() < 1e-9));
    let second: Vec<f64> = centres[5..].iter().map(|centre| centre.0).collect();
    assert!(second.iter().all(|x| (x - second[0]).abs() < 1e-9));
    assert!(first[0] < second[0], "the categories are ordered");
}

#[test]
fn no_two_dots_of_a_category_overlap() {
    const CLUSTER: [f64; 9] = [5.0; 9];
    const ONE_CATEGORY: [f64; 9] = [0.0; 9];
    let table = SVDataTableBuilder::new()
        .with_column("value", SVColumnRole::Y, &CLUSTER)
        .with_column("category", SVColumnRole::Grouping, &ONE_CATEGORY)
        .build()
        .expect("the table is well formed");
    let scene = scene(&table);
    let centres = circle_centres(&scene);
    let diameter = 6.0;

    assert_eq!(centres.len(), CLUSTER.len());
    for left in 0..centres.len() {
        for right in (left + 1)..centres.len() {
            let horizontal = (centres[left].0 - centres[right].0).abs();
            let vertical = (centres[left].1 - centres[right].1).abs();
            assert!(
                !(horizontal < diameter - 1e-9 && vertical < diameter - 1e-9),
                "dots {left} and {right} overlap: dx={horizontal}, dy={vertical}"
            );
        }
    }
}

#[test]
fn the_layout_is_deterministic() {
    let source = dense_source();
    let plot = default_builder(&source)
        .build()
        .expect("the configuration is complete");

    let first = scene_svg(&plot.compose().expect("the first compose succeeds"));
    let second = scene_svg(&plot.compose().expect("the second compose succeeds"));
    assert_eq!(first, second);
}

#[test]
fn the_configured_width_bounds_the_displacement() {
    const CLUSTER: [f64; 5] = [5.0; 5];
    const ONE_CATEGORY: [f64; 5] = [0.0; 5];
    let table = SVDataTableBuilder::new()
        .with_column("value", SVColumnRole::Y, &CLUSTER)
        .with_column("category", SVColumnRole::Grouping, &ONE_CATEGORY)
        .build()
        .expect("the table is well formed");
    let scene = SVSwarmPlot::builder()
        .data_source(&table)
        .value("value")
        .category("category")
        .swarm_width(0.1)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the bounded swarm lays out");
    let centres = circle_centres(&scene);
    let slot = 468.0;
    let width = slot * 0.1;
    let minimum = centres
        .iter()
        .map(|centre| centre.0)
        .fold(f64::INFINITY, f64::min);
    let maximum = centres
        .iter()
        .map(|centre| centre.0)
        .fold(f64::NEG_INFINITY, f64::max);
    let centre = (minimum + maximum) / 2.0;

    assert!(
        maximum - minimum <= width + 1e-6,
        "the swarm stays inside the configured fraction"
    );
    assert!(minimum < centre && centre < maximum);
}

#[test]
fn a_width_below_two_marker_extents_is_the_kernel_typed_error() {
    const CLUSTER: [f64; 2] = [5.0; 2];
    const ONE_CATEGORY: [f64; 2] = [0.0; 2];
    let table = SVDataTableBuilder::new()
        .with_column("value", SVColumnRole::Y, &CLUSTER)
        .with_column("category", SVColumnRole::Grouping, &ONE_CATEGORY)
        .build()
        .expect("the table is well formed");
    let error = SVSwarmPlot::builder()
        .data_source(&table)
        .value("value")
        .category("category")
        .swarm_width(0.001)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect_err("the kernel rejects an impossible width");

    match error {
        SVError::InvalidConfiguration { message } => {
            assert!(message.contains("two marker extents"), "message: {message}");
        }
        other => panic!("expected the kernel's typed width error, received {other:?}"),
    }
}

#[test]
fn the_documented_columns_and_defaults_are_reported() {
    let source = dense_source();
    let plot = default_builder(&source)
        .build()
        .expect("the configuration is complete");

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(
        plot.execution_mode(),
        crate::sv_figure::SVExecutionMode::Automatic
    );
    assert_eq!(plot.point_count(), VALUES.len());
    let debug = format!("{plot:?}");
    assert!(debug.contains("SVSwarmPlot"));
}

#[test]
fn a_missing_required_column_fails_with_an_explicit_error() {
    let source = dense_source();
    let error = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a category column is required");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));

    let error = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .series("absent")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a missing series column fails");
    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

#[test]
fn a_wrong_column_role_is_rejected_at_build_time() {
    let source = SVTestSource::new(vec![
        ("value", SVColumnRole::Label, SVColumnView::Dense(&VALUES)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Dense(&CATEGORY),
        ),
    ]);
    let error = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the value column has the wrong role");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_impossible_swarm_width_is_rejected_at_build_time() {
    let source = dense_source();
    let error = SVSwarmPlotBuilder::new()
        .data_source(&source)
        .value("value")
        .category("category")
        .swarm_width(1.5)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a fraction above one is impossible");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_non_finite_observation_obeys_the_missing_policy() {
    const WITH_MISSING: [f64; 5] = [1.0, f64::NAN, 3.0, 4.0, 5.0];
    const ONE_CATEGORY: [f64; 5] = [0.0; 5];
    let table = SVDataTableBuilder::new()
        .with_column("value", SVColumnRole::Y, &WITH_MISSING)
        .with_column("category", SVColumnRole::Grouping, &ONE_CATEGORY)
        .build()
        .expect("the table is well formed");

    let omitted = scene(&table);
    assert_eq!(omitted.number_of_marks(), 4);

    let error = SVSwarmPlot::builder()
        .data_source(&table)
        .value("value")
        .category("category")
        .missing_value_policy(super::SVSwarmMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect_err("the strict policy rejects the missing observation");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_figure_title_and_axis_labels_reach_the_scene() {
    let source = dense_source();
    let scene = default_builder(&source)
        .title("Body mass by species")
        .x_label("species")
        .y_label("body mass")
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");

    assert_eq!(scene.title(), Some("Body mass by species"));
    let text = scene_text(&scene);
    assert!(text.contains("species"));
    assert!(text.contains("body mass"));
}

#[test]
fn the_public_render_produces_the_configured_artifact() {
    let output = render(&dense_source());
    assert_eq!(output.backend_kind(), SVBackendKind::Svg);
    assert!(output.as_svg().is_some());
}
