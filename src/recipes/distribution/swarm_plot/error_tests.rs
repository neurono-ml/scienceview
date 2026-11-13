//! The typed errors a swarm reports instead of degrading silently.

use super::SVSwarmPlot;
use super::test_support::dense_source;
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;

#[test]
fn a_build_without_a_backend_selection_fails() {
    let source = dense_source();
    let error = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .build()
        .expect_err("a backend is required");
    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn a_build_without_a_source_fails() {
    let error = SVSwarmPlot::builder()
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a source is required");
    assert_eq!(error, SVError::MissingDataSource);
}

#[test]
fn mismatched_columns_fail_at_build_time() {
    const NINE: [f64; 9] = [0.0; 9];
    let source = super::test_support::SVTestSource::new(vec![
        (
            "value",
            crate::sv_data::SVColumnRole::Y,
            crate::sv_data::SVColumnView::Dense(&super::test_support::VALUES),
        ),
        (
            "category",
            crate::sv_data::SVColumnRole::Grouping,
            crate::sv_data::SVColumnView::Dense(&NINE),
        ),
    ]);
    let error = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the columns disagree");
    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn a_backend_without_a_serializer_fails_explicitly() {
    let source = dense_source();
    let error = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Html)
        .build()
        .expect("the backend can host dots")
        .render()
        .expect_err("the recipe has no HTML serializer yet");
    assert!(matches!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            ..
        }
    ));
}

#[test]
fn an_invalid_opacity_fails_at_build_time() {
    let source = dense_source();
    let error = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .alpha(2.0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("opacity is a fraction");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}
