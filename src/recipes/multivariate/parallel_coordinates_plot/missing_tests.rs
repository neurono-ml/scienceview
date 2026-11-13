//! The missing-value policies of the parallel-coordinates plot.

use super::test_support::{sv_build_default, sv_missing_source};
use super::{SVParallelCoordinatesMissingPolicy, SVParallelCoordinatesPlot};
use crate::sv_backend::SVBackendKind;

#[test]
fn the_missing_policies_choose_between_a_gap_a_bridge_and_a_drop() {
    let gap_source = sv_missing_source();
    let gap = sv_build_default(&gap_source)
        .compose()
        .expect("the default gap policy composes");
    assert_eq!(gap.lines().len(), 5, "the polyline is split at the gap");

    let bridge_source = sv_missing_source();
    let bridge = SVParallelCoordinatesPlot::builder()
        .data_source(&bridge_source)
        .variables(&["first", "second", "third"])
        .missing(SVParallelCoordinatesMissingPolicy::Bridge)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the bridge policy composes");
    assert_eq!(bridge.lines().len(), 4);

    let drop_source = sv_missing_source();
    let dropped = SVParallelCoordinatesPlot::builder()
        .data_source(&drop_source)
        .variables(&["first", "second", "third"])
        .missing(SVParallelCoordinatesMissingPolicy::DropObservation)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the drop policy composes");
    assert_eq!(dropped.lines().len(), 3);

    let error_source = sv_missing_source();
    let error = SVParallelCoordinatesPlot::builder()
        .data_source(&error_source)
        .variables(&["first", "second", "third"])
        .missing(SVParallelCoordinatesMissingPolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect_err("the error policy rejects the observation");
    assert!(matches!(
        error,
        crate::sv_error::SVError::NonFiniteValue { .. }
    ));
}
