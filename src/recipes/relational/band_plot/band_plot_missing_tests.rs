//! The missing-value policy: omit splits the band, error rejects the render.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;

use super::{SVBandMissingValuePolicy, SVBandPlotBuilder};

#[test]
fn missing_value_policy_omit_splits_the_band() {
    let lower = [0.5, f64::NAN, 1.2];
    let upper = [1.5, 2.2, 3.1];
    let x = [0.0, 1.0, 2.0];
    let data = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("lower", SVColumnRole::Y, &lower)
        .with_column("upper", SVColumnRole::Y, &upper)
        .build()
        .expect("the columns share one length");
    let output = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .lower("lower")
        .upper("upper")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");
    assert_eq!(output.geometries().len(), 2);
}

#[test]
fn missing_value_policy_error_rejects_the_render() {
    let lower = [0.5, f64::NAN, 1.2];
    let upper = [1.5, 2.2, 3.1];
    let x = [0.0, 1.0, 2.0];
    let data = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("lower", SVColumnRole::Y, &lower)
        .with_column("upper", SVColumnRole::Y, &upper)
        .build()
        .expect("the columns share one length");
    let result = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .lower("lower")
        .upper("upper")
        .missing_value_policy(SVBandMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render();
    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}

#[test]
fn derived_missing_value_is_omitted_and_split() {
    let x = [0.0, 0.0, 1.0, 1.0];
    let y = [1.0, f64::NAN, 2.0, 4.0];
    let data = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("y", SVColumnRole::Y, &y)
        .build()
        .expect("the columns share one length");
    let output = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .derive()
        .values("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the finite group still draws");
    assert_eq!(output.number_of_points(), 1);
}

#[test]
fn derived_missing_value_is_rejected_under_the_error_policy() {
    let x = [0.0, 0.0, 1.0, 1.0];
    let y = [1.0, f64::NAN, 2.0, 4.0];
    let data = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("y", SVColumnRole::Y, &y)
        .build()
        .expect("the columns share one length");
    let result = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .derive()
        .values("y")
        .missing_value_policy(SVBandMissingValuePolicy::Error)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render();
    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}
