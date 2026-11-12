//! Companion tests for the user-supplied estimator trait.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_stat::{SVFitEstimator, SVStatInput, sv_fit_with_estimator};

/// A hand-written least-squares line, used as the trait's reference estimator.
struct SVReferenceLineEstimator;

impl SVFitEstimator for SVReferenceLineEstimator {
    fn estimate(&self, independent: &[f64], values: &[f64]) -> Result<Vec<f64>, SVError> {
        let count = independent.len() as f64;
        let mean_x = independent.iter().sum::<f64>() / count;
        let mean_y = values.iter().sum::<f64>() / count;
        let covariance: f64 = independent
            .iter()
            .zip(values)
            .map(|(x, y)| (x - mean_x) * (y - mean_y))
            .sum();
        let variance: f64 = independent
            .iter()
            .map(|x| (x - mean_x) * (x - mean_x))
            .sum();

        if variance == 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: "the reference estimator needs two distinct independent values"
                    .to_string(),
            });
        }

        let slope = covariance / variance;
        Ok(vec![mean_y - slope * mean_x, slope])
    }

    fn evaluate(&self, coefficients: &[f64], point: f64) -> f64 {
        coefficients[0] + coefficients[1] * point
    }
}

#[test]
fn a_custom_estimator_recovers_the_line_it_fits() {
    let independent = [0.0, 1.0, 2.0, 3.0, 4.0];
    let values = [1.0, 3.0, 5.0, 7.0, 9.0];
    let input = SVStatInput::new(SVColumnView::Dense(&values))
        .with_secondary(SVColumnView::Dense(&independent));

    let curve = sv_fit_with_estimator(&SVReferenceLineEstimator, input)
        .expect("the reference estimator fits the line");

    assert_eq!(curve.coefficients().len(), 2);
    assert!((curve.coefficients()[0] - 1.0).abs() < 1e-12);
    assert!((curve.coefficients()[1] - 2.0).abs() < 1e-12);
    assert_eq!(curve.grid().len(), 128);
    assert!((curve.predicted()[0] - 1.0).abs() < 1e-12);
    assert!((curve.predicted()[127] - 9.0).abs() < 1e-12);
}

#[test]
fn a_custom_estimator_needs_the_independent_column() {
    let error = sv_fit_with_estimator(
        &SVReferenceLineEstimator,
        SVStatInput::new(SVColumnView::Dense(&[1.0, 2.0])),
    )
    .expect_err("a fit needs two columns");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_custom_estimator_rejects_mismatched_lengths() {
    let error = sv_fit_with_estimator(
        &SVReferenceLineEstimator,
        SVStatInput::new(SVColumnView::Dense(&[1.0, 2.0, 3.0]))
            .with_secondary(SVColumnView::Dense(&[0.0, 1.0])),
    )
    .expect_err("a paired fit needs equal lengths");

    assert_eq!(
        error,
        SVError::MismatchedStatisticLengths {
            operation: "fit",
            first: 3,
            second: 2,
        }
    );
}

#[test]
fn a_custom_estimator_rejects_an_empty_column() {
    let error = sv_fit_with_estimator(
        &SVReferenceLineEstimator,
        SVStatInput::new(SVColumnView::Dense(&[])).with_secondary(SVColumnView::Dense(&[])),
    )
    .expect_err("there is nothing to fit");

    assert_eq!(error, SVError::EmptyInput { operation: "fit" });
}

#[test]
fn a_streaming_custom_fit_reports_that_it_is_windowed() {
    let independent = [0.0, 1.0, 2.0, 3.0];
    let values = [1.0, 3.0, 5.0, 7.0];
    let x = crate::sv_data::SVStreamingView::new(&independent, 9, 5);
    let y = crate::sv_data::SVStreamingView::new(&values, 9, 5);
    let input =
        SVStatInput::new(SVColumnView::Streaming(y)).with_secondary(SVColumnView::Streaming(x));

    let curve = sv_fit_with_estimator(&SVReferenceLineEstimator, input)
        .expect("the reference estimator fits the window");

    assert!(curve.is_windowed());
}

#[test]
fn a_dense_custom_fit_is_not_windowed() {
    let independent = [0.0, 1.0, 2.0, 3.0];
    let values = [1.0, 3.0, 5.0, 7.0];
    let input = SVStatInput::new(SVColumnView::Dense(&values))
        .with_secondary(SVColumnView::Dense(&independent));

    let curve = sv_fit_with_estimator(&SVReferenceLineEstimator, input)
        .expect("the reference estimator fits the line");

    assert!(!curve.is_windowed());
}

#[test]
fn a_non_finite_observation_is_rejected_with_its_position() {
    let error = sv_fit_with_estimator(
        &SVReferenceLineEstimator,
        SVStatInput::new(SVColumnView::Dense(&[1.0, f64::NAN, 5.0]))
            .with_secondary(SVColumnView::Dense(&[0.0, 1.0, 2.0])),
    )
    .expect_err("NaN is not an observation");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "fit",
            position: 1,
        }
    );
}
