//! The documented defaults of the parallel-coordinates plot.

use super::test_support::{sv_build_default, sv_dense_source, sv_output_svg};
use super::{SVParallelCoordinatesMissingPolicy, SVParallelCoordinatesNormalization};
use crate::sv_backend::SVBackendKind;
use crate::sv_mark::SVMarkShape;

#[test]
fn the_default_configuration_draws_one_axis_per_variable_and_one_line_per_observation() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    let output = plot
        .compose()
        .expect("the default parallel coordinates composes");

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(output.number_of_observations(), 4);
    assert_eq!(output.axes().len(), 3);
    assert_eq!(output.lines().len(), 4);
    assert_eq!(output.lines()[0].vertices().len(), 3);
    assert_eq!(output.title(), None);
    assert_eq!(
        output.normalization(),
        SVParallelCoordinatesNormalization::PerAxis
    );
    assert_eq!(
        output.missing_policy(),
        SVParallelCoordinatesMissingPolicy::Gap
    );
    assert!(!sv_output_svg(&output).is_empty());
}

#[test]
fn a_polyline_is_a_line_mark_path() {
    let source = sv_dense_source();
    let output = sv_build_default(&source)
        .compose()
        .expect("the chart composes");

    for geometry in output.geometries() {
        assert_eq!(geometry.primitives().len(), 1);
        assert!(matches!(
            geometry.primitives()[0].shape(),
            SVMarkShape::Path(_)
        ));
    }
}
