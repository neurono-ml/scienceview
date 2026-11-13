//! The documented composition and data contract of `SVJointPlot`.

use super::test_support::{SVJointTestSource, sv_build_default};
use super::{SVJointKind, SVJointMarginal, SVJointMissingValuePolicy, SVJointPlot};
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;

#[test]
fn the_default_composition_builds_the_three_panels() {
    let source = SVJointTestSource::dense();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the default joint plot composes");

    assert_eq!(scene.kind(), SVJointKind::Scatter);
    assert_eq!(scene.marginal_kind(), SVJointMarginal::Histogram);
    assert!(scene.number_of_marks() > 0);
    assert!(scene.number_of_mark_primitives() >= 8);
    assert!(scene.horizontal_marginal_panel().y < scene.central_panel().y);
    assert!(scene.vertical_marginal_panel().x > scene.central_panel().right());
}

#[test]
fn the_three_panels_share_the_central_axes() {
    let source = SVJointTestSource::dense();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the joint plot composes");

    let central = scene.central_panel();
    let horizontal = scene.horizontal_scale();
    let vertical = scene.vertical_scale();
    assert!((horizontal.range().start() - central.x).abs() < 1.0e-9);
    assert!((horizontal.range().end() - central.right()).abs() < 1.0e-9);
    assert!((vertical.range().start() - central.bottom()).abs() < 1.0e-9);
    assert!((vertical.range().end() - central.y).abs() < 1.0e-9);
}

#[test]
fn a_missing_required_column_reports_an_explicit_error() {
    let source = SVJointTestSource::dense();
    let error = SVJointPlot::builder()
        .data_source(&source)
        .x("x")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the y column is required");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_missing_backend_reports_an_explicit_error() {
    let source = SVJointTestSource::dense();
    let error = SVJointPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .build()
        .expect_err("the backend is required");

    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn a_missing_series_column_reports_column_not_found() {
    let source = SVJointTestSource::dense();
    let plot = SVJointPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the series column is not served");

    assert!(matches!(plot, SVError::ColumnNotFound { .. }));
}

#[test]
fn the_omit_policy_drops_a_non_finite_observation() {
    const X: [f64; 4] = [0.0, 1.0, f64::NAN, 3.0];
    const Y: [f64; 4] = [1.0, 2.0, 4.0, 5.0];
    let source = SVJointTestSource {
        horizontal: crate::sv_data::SVColumnView::Dense(&X),
        vertical: crate::sv_data::SVColumnView::Dense(&Y),
        series: None,
    };
    let scene = sv_build_default(&source)
        .compose()
        .expect("the omitted observation does not poison the render");
    assert!(scene.number_of_marks() > 0);
}

#[test]
fn the_error_policy_rejects_a_non_finite_observation() {
    const X: [f64; 4] = [0.0, 1.0, f64::NAN, 3.0];
    const Y: [f64; 4] = [1.0, 2.0, 4.0, 5.0];
    let source = SVJointTestSource {
        horizontal: crate::sv_data::SVColumnView::Dense(&X),
        vertical: crate::sv_data::SVColumnView::Dense(&Y),
        series: None,
    };
    let error = SVJointPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .missing_value_policy(SVJointMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the build is complete")
        .compose()
        .expect_err("the missing policy rejects the render");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn every_central_kind_composes() {
    let source = SVJointTestSource::dense();
    for kind in [
        SVJointKind::Scatter,
        SVJointKind::Hexbin,
        SVJointKind::DensityContour,
        SVJointKind::Regression,
    ] {
        let scene = super::test_support::sv_build_kind(&source, kind, SVJointMarginal::Auto)
            .compose()
            .expect("the central kind composes");
        assert_eq!(scene.kind(), kind);
        assert!(scene.number_of_marks() > 0);
    }
}
