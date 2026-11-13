//! Every public option that changes the figure changes it observably.

use super::SVJointPlot;
use super::test_support::{SVJointTestSource, sv_build_kind, sv_scene_text};
use super::{SVJointKind, SVJointMarginal, SVJointRegressionModel};
use crate::sv_backend::SVBackendKind;
#[cfg(feature = "render-svg")]
use crate::sv_mark::SVDotShape;

#[test]
fn a_series_column_groups_the_central_panel_and_adds_a_legend() {
    let source = SVJointTestSource::grouped();
    let scene = SVJointPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the grouped joint plot composes");

    assert_eq!(scene.number_of_series(), 3);
    assert_eq!(scene.legend_labels(), ["0", "1", "2"]);
}

#[test]
fn each_marginal_kind_changes_the_figure() {
    let source = SVJointTestSource::dense();
    let histogram = sv_build_kind(&source, SVJointKind::Scatter, SVJointMarginal::Histogram)
        .compose()
        .expect("the histogram marginals compose");
    let density = sv_build_kind(&source, SVJointKind::Scatter, SVJointMarginal::Density)
        .compose()
        .expect("the density marginals compose");
    let rug = sv_build_kind(&source, SVJointKind::Scatter, SVJointMarginal::Rug)
        .compose()
        .expect("the rug marginals compose");

    assert_ne!(
        histogram.number_of_mark_primitives(),
        density.number_of_mark_primitives()
    );
    assert_ne!(
        histogram.number_of_mark_primitives(),
        rug.number_of_mark_primitives()
    );
}

#[test]
fn the_title_and_axis_labels_reach_the_decorations() {
    let source = SVJointTestSource::dense();
    let scene = SVJointPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .title("Relationship")
        .x_label("Weight")
        .y_label("Height")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the joint plot composes");

    let text = sv_scene_text(&scene);
    assert!(text.contains("Relationship"));
    assert!(text.contains("Weight"));
    assert!(text.contains("Height"));
}

#[test]
fn the_regression_model_option_is_carried() {
    let source = SVJointTestSource::dense();
    for model in [
        SVJointRegressionModel::Linear,
        SVJointRegressionModel::Polynomial,
        SVJointRegressionModel::Exponential,
    ] {
        let plot = SVJointPlot::builder()
            .data_source(&source)
            .x("x")
            .y("y")
            .kind(SVJointKind::Regression)
            .regression_model(model)
            .backend(SVBackendKind::Svg)
            .build()
            .expect("the configuration is complete");
        assert!(plot.compose().is_ok());
    }
}

#[cfg(feature = "render-svg")]
#[test]
fn the_marker_and_alpha_options_change_the_painted_output() {
    let source = SVJointTestSource::dense();
    let circle = SVJointPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .marker(SVDotShape::Circle)
        .alpha(1.0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the plot renders");
    let square = SVJointPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .marker(SVDotShape::Square)
        .alpha(0.4)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the plot renders");

    assert_ne!(circle.bytes(), square.bytes());
}
