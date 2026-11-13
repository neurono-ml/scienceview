//! Column resolution, the explicit configuration errors and missing values.

use super::super::SVECdfPlot;
use super::test_support::SVTestSource;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use scienceview_core::sv_error::SVError;

fn build<'a>(source: &'a SVTestSource<'a>) -> SVECdfPlot<'a> {
    SVECdfPlot::builder()
        .data_source(source)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
}

#[test]
fn a_missing_data_source_is_rejected_at_build_time() {
    let error = SVECdfPlot::builder()
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a data source is required");
    assert_eq!(error, SVError::MissingDataSource);
}

#[test]
fn a_missing_backend_selection_is_rejected_at_build_time() {
    let values = [1.0, 2.0];
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &values);
    let error = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .build()
        .expect_err("the backend is a required choice");
    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn a_missing_value_column_names_the_option() {
    let values = [1.0, 2.0];
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &values);
    let error = SVECdfPlot::builder()
        .data_source(&table)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the value column is required");
    assert!(
        error.to_string().contains("value"),
        "the message names the option: {error}"
    );
}

#[test]
fn an_unknown_value_column_is_rejected() {
    let values = [1.0, 2.0];
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &values);
    let error = SVECdfPlot::builder()
        .data_source(&table)
        .value("missing")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the column does not exist");
    assert!(matches!(error, SVError::ColumnNotFound { .. }), "{error}");
}

#[test]
fn a_label_role_value_column_is_rejected() {
    let values = [1.0, 2.0];
    let table = SVTestSource::new().with_dense("name", SVColumnRole::Label, &values);
    let error = SVECdfPlot::builder()
        .data_source(&table)
        .value("name")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("labels are not numeric observations");
    assert!(
        error.to_string().contains("labels"),
        "the message explains the role: {error}"
    );
}

#[test]
fn mismatched_series_and_value_lengths_are_rejected() {
    let values = [1.0, 2.0, 3.0];
    let series = [1.0, 2.0];
    let table = SVTestSource::new()
        .with_dense("value", SVColumnRole::Y, &values)
        .with_dense("scenario", SVColumnRole::Grouping, &series);
    let error = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .series("scenario")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the columns must align");
    assert!(
        matches!(error, SVError::MismatchedLengths { .. }),
        "{error}"
    );
}

#[test]
fn non_finite_observations_are_omitted() {
    let values = [1.0, f64::NAN, 3.0, f64::INFINITY];
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &values);
    let composition = build(&table).compose().expect("the finite values compose");

    assert_eq!(composition.point_count(), 2);
    assert_eq!(composition.series()[0].sample_horizontal(), &[1.0, 3.0]);
    assert_eq!(composition.series()[0].sample_vertical(), &[0.5, 1.0]);
}

#[test]
fn an_all_missing_column_is_an_empty_input() {
    let values = [f64::NAN, f64::INFINITY];
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &values);
    let error = build(&table).compose().expect_err("nothing can be drawn");
    assert_eq!(
        error,
        SVError::EmptyInput {
            operation: "empirical_cdf"
        }
    );
}

#[test]
fn a_non_finite_series_code_is_omitted() {
    let values = [1.0, 2.0, 3.0];
    let series = [1.0, f64::NAN, 2.0];
    let table = SVTestSource::new()
        .with_dense("value", SVColumnRole::Y, &values)
        .with_dense("scenario", SVColumnRole::Grouping, &series);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .series("scenario")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let composition = plot.compose().expect("the finite groups compose");

    assert_eq!(composition.number_of_series(), 2);
    assert_eq!(composition.point_count(), 2);
}
