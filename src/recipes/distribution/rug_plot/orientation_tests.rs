//! The orientation and legend surface of the rug plot.

use super::SVRugPlot;
use super::test_support::{sv_build_default, sv_dense_source, sv_scene_text, sv_scene_to_svg};
use crate::sv_backend::SVBackendKind;
use crate::sv_mark::SVBarOrientation;

#[test]
fn a_rug_without_a_series_draws_no_legend_entries() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the rug composes");

    assert_eq!(scene.number_of_series(), 1);
    assert!(scene.legend_labels().is_empty());
}

#[test]
fn the_orientation_option_changes_the_document() {
    let source = sv_dense_source();
    let vertical = sv_build_default(&source)
        .compose()
        .expect("the vertical rug composes");
    let horizontal = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .orientation(SVBarOrientation::Horizontal)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the horizontal rug composes");

    assert_ne!(sv_scene_to_svg(&vertical), sv_scene_to_svg(&horizontal));
    assert!(
        sv_scene_text(&vertical).contains("value"),
        "the value column titles the axis it occupies"
    );
}
