//! Companion tests for the fit kernel.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_stat::{SVFitModel, SVFitOptions, SVFitResult, SVStat, SVStatInput, SVStatResult};

fn apply_fit(options: SVFitOptions, independent: &[f64], values: &[f64]) -> SVFitResult {
    let input = SVStatInput::new(SVColumnView::Dense(values))
        .with_secondary(SVColumnView::Dense(independent));

    match SVStat::fit(options)
        .apply(input)
        .expect("the fit configuration is valid")
    {
        SVStatResult::Fit(result) => result,
        other => panic!("the fit stat must produce a fit, received {other:?}"),
    }
}

fn assert_coefficients(result: &SVFitResult, expected: &[f64]) {
    assert_eq!(result.coefficients().len(), expected.len());
    for (actual, expected) in result.coefficients().iter().zip(expected) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "coefficient {actual} differs from {expected}"
        );
    }
}

#[test]
fn a_linear_fit_recovers_the_line_that_generated_the_data() {
    let result = apply_fit(
        SVFitOptions::new(),
        &[0.0, 1.0, 2.0, 3.0, 4.0],
        &[1.0, 3.0, 5.0, 7.0, 9.0],
    );

    assert_eq!(result.model(), SVFitModel::Linear);
    assert_coefficients(&result, &[1.0, 2.0]);
    assert_eq!(result.grid().len(), 128);
    assert!((result.predicted()[0] - 1.0).abs() < 1e-9);
    assert!((result.predicted()[127] - 9.0).abs() < 1e-9);
}

#[test]
fn a_polynomial_fit_recovers_the_quadratic_that_generated_the_data() {
    let independent = [-2.0, -1.0, 0.0, 1.0, 2.0];
    let values: Vec<f64> = independent.iter().map(|x| x * x).collect();
    let result = apply_fit(
        SVFitOptions::new().model(SVFitModel::Polynomial).order(2),
        &independent,
        &values,
    );

    assert_coefficients(&result, &[0.0, 0.0, 1.0]);
}

#[test]
fn an_exponential_fit_recovers_the_growth_rate() {
    let independent: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
    let values: Vec<f64> = independent.iter().map(|x| 2.0 * (0.5 * *x).exp()).collect();
    let result = apply_fit(
        SVFitOptions::new().model(SVFitModel::Exponential),
        &independent,
        &values,
    );

    assert_coefficients(&result, &[2.0, 0.5]);
    assert!((result.predicted()[0] - 2.0).abs() < 1e-9);
}

#[test]
fn a_logarithmic_fit_recovers_its_coefficients() {
    let independent: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let values: Vec<f64> = independent.iter().map(|x| 1.0 + 2.0 * (*x).ln()).collect();
    let result = apply_fit(
        SVFitOptions::new().model(SVFitModel::Logarithmic),
        &independent,
        &values,
    );

    assert_coefficients(&result, &[1.0, 2.0]);
}

#[test]
fn a_power_fit_recovers_its_exponent() {
    let independent: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let values: Vec<f64> = independent.iter().map(|x| 2.0 * (*x).powf(1.5)).collect();
    let result = apply_fit(
        SVFitOptions::new().model(SVFitModel::Power),
        &independent,
        &values,
    );

    assert_coefficients(&result, &[2.0, 1.5]);
}

/// Asserts that every predicted sample lies on `intercept + slope * point`.
fn assert_predictions_follow_the_line(result: &SVFitResult, intercept: f64, slope: f64) {
    for (point, predicted) in result.grid().iter().copied().zip(result.predicted()) {
        let expected = intercept + slope * point;
        let tolerance = 1.0e-9 * expected.abs().max(1.0);
        assert!(
            (predicted - expected).abs() <= tolerance,
            "the prediction {predicted} at {point} differs from {expected}"
        );
    }
}

#[test]
fn a_linear_fit_survives_a_large_offset_of_a_million() {
    let independent: Vec<f64> = (0..11).map(|step| 1.0e6 + step as f64).collect();
    let values: Vec<f64> = independent.iter().map(|x| 1.0 + 2.0 * x).collect();

    let result = apply_fit(SVFitOptions::new(), &independent, &values);

    assert!((result.coefficients()[1] - 2.0).abs() < 1.0e-9);
    assert_predictions_follow_the_line(&result, 1.0, 2.0);
}

#[test]
fn a_linear_fit_survives_a_large_offset_of_a_billion() {
    let independent: Vec<f64> = (0..11).map(|step| 1.7e9 + step as f64).collect();
    let values: Vec<f64> = independent.iter().map(|x| 1.0 + 2.0 * x).collect();

    let result = apply_fit(SVFitOptions::new(), &independent, &values);

    // The generated value column is rounded at 3.4e9, so the recovered slope
    // carries that rounding rather than the solver's error.
    assert!((result.coefficients()[1] - 2.0).abs() < 1.0e-6);
    assert_predictions_follow_the_line(&result, 1.0, 2.0);
}

#[test]
fn a_quadratic_fit_survives_a_large_offset_around_a_million() {
    let independent: Vec<f64> = (-6..=6).map(|step| 1.0e6 + step as f64).collect();
    let values: Vec<f64> = independent.iter().map(|x| x * x - 3.0 * x + 7.0).collect();

    let result = apply_fit(
        SVFitOptions::new().model(SVFitModel::Polynomial).order(2),
        &independent,
        &values,
    );

    for (point, predicted) in result.grid().iter().copied().zip(result.predicted()) {
        let expected = point * point - 3.0 * point + 7.0;
        let tolerance = 1.0e-9 * expected.abs().max(1.0);
        assert!(
            (predicted - expected).abs() <= tolerance,
            "the prediction {predicted} at {point} differs from {expected}"
        );
    }
}

#[test]
fn a_confidence_band_survives_a_large_offset() {
    let independent: Vec<f64> = (0..11).map(|step| 1.7e9 + step as f64).collect();
    let values: Vec<f64> = independent
        .iter()
        .map(|x| 1.0 + 2.0 * x + 0.25 * ((x - 1.7e9) as i64 % 3) as f64)
        .collect();

    let result = apply_fit(SVFitOptions::new().confidence(0.95), &independent, &values);

    let lower = result.lower().expect("the band is configured");
    let upper = result.upper().expect("the band is configured");
    for ((low, predicted), high) in lower.iter().zip(result.predicted()).zip(upper) {
        assert!(low.is_finite() && high.is_finite());
        assert!(*low <= predicted + 1.0e-3 && *predicted <= high + 1.0e-3);
    }
}

#[test]
fn a_confidence_band_brackets_the_predicted_curve() {
    let result = apply_fit(
        SVFitOptions::new().confidence(0.95),
        &[0.0, 1.0, 2.0, 3.0, 4.0],
        &[1.0, 3.2, 4.8, 7.1, 9.2],
    );

    let lower = result.lower().expect("the band is configured");
    let upper = result.upper().expect("the band is configured");
    assert_eq!(lower.len(), result.predicted().len());
    for ((low, predicted), high) in lower.iter().zip(result.predicted()).zip(upper) {
        assert!(low <= predicted && predicted <= high);
    }
}

#[test]
fn a_wider_confidence_produces_a_wider_band() {
    let narrow = apply_fit(
        SVFitOptions::new().confidence(0.5),
        &[0.0, 1.0, 2.0, 3.0, 4.0],
        &[1.0, 3.2, 4.8, 7.1, 9.2],
    );
    let wide = apply_fit(
        SVFitOptions::new().confidence(0.99),
        &[0.0, 1.0, 2.0, 3.0, 4.0],
        &[1.0, 3.2, 4.8, 7.1, 9.2],
    );

    let narrow_width = narrow.upper().expect("band")[10] - narrow.lower().expect("band")[10];
    let wide_width = wide.upper().expect("band")[10] - wide.lower().expect("band")[10];
    assert!(wide_width > narrow_width);
}

#[test]
fn a_confidence_band_on_a_model_without_one_is_rejected() {
    let error = SVStat::fit(
        SVFitOptions::new()
            .model(SVFitModel::Exponential)
            .confidence(0.95),
    )
    .apply(
        SVStatInput::new(SVColumnView::Dense(&[2.0, 4.0, 8.0]))
            .with_secondary(SVColumnView::Dense(&[0.0, 1.0, 2.0])),
    )
    .expect_err("only linear models carry a computed band");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_sparse_column_is_rejected_with_an_alternative_path() {
    let independent = [0.0, 1.0];
    let sparse_values = [2.0];
    let sparse_indices = [0usize];
    let values =
        SVSparseView::new(&sparse_indices, &sparse_values, 2).expect("valid sparse storage");
    let input = SVStatInput::new(SVColumnView::Sparse(values))
        .with_secondary(SVColumnView::Dense(&independent));

    let error = SVStat::fit(SVFitOptions::new())
        .apply(input)
        .expect_err("a paired fit rejects sparse storage");

    assert!(matches!(
        error,
        SVError::UnsupportedRepresentation {
            operation: "fit",
            representation: crate::sv_data::SVRepresentation::Sparse,
            ..
        }
    ));
}

#[test]
fn a_fit_without_an_independent_column_is_rejected() {
    let error = SVStat::fit(SVFitOptions::new())
        .apply(SVStatInput::new(SVColumnView::Dense(&[1.0, 2.0])))
        .expect_err("a fit needs two columns");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn columns_of_different_lengths_are_rejected() {
    let error = SVStat::fit(SVFitOptions::new())
        .apply(
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
fn an_empty_column_is_rejected() {
    let error = SVStat::fit(SVFitOptions::new())
        .apply(SVStatInput::new(SVColumnView::Dense(&[])).with_secondary(SVColumnView::Dense(&[])))
        .expect_err("there is nothing to fit");

    assert_eq!(error, SVError::EmptyInput { operation: "fit" });
}

#[test]
fn a_non_finite_observation_is_rejected() {
    let error = SVStat::fit(SVFitOptions::new())
        .apply(
            SVStatInput::new(SVColumnView::Dense(&[1.0, 2.0]))
                .with_secondary(SVColumnView::Dense(&[0.0, f64::NAN])),
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

#[test]
fn a_streaming_fit_reports_that_it_is_windowed() {
    let independent = [0.0, 1.0, 2.0, 3.0];
    let values = [1.0, 3.0, 5.0, 7.0];
    let x_view = SVStreamingView::new(&independent, 12, 8);
    let y_view = SVStreamingView::new(&values, 12, 8);
    let input = SVStatInput::new(SVColumnView::Streaming(y_view))
        .with_secondary(SVColumnView::Streaming(x_view));

    let result = match SVStat::fit(SVFitOptions::new())
        .apply(input)
        .expect("the window is fittable")
    {
        SVStatResult::Fit(result) => result,
        other => panic!("the fit stat must produce a fit, received {other:?}"),
    };

    assert!(result.is_windowed());
}

#[test]
fn a_dense_fit_is_not_windowed() {
    let result = apply_fit(SVFitOptions::new(), &[0.0, 1.0, 2.0], &[1.0, 3.0, 5.0]);

    assert!(!result.is_windowed());
}

#[test]
fn a_zero_polynomial_order_is_rejected() {
    let error = SVStat::fit(SVFitOptions::new().model(SVFitModel::Polynomial).order(0))
        .apply(
            SVStatInput::new(SVColumnView::Dense(&[1.0, 2.0]))
                .with_secondary(SVColumnView::Dense(&[0.0, 1.0])),
        )
        .expect_err("a polynomial needs a positive degree");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn too_few_observations_for_the_polynomial_degree_are_rejected() {
    let error = SVStat::fit(SVFitOptions::new().model(SVFitModel::Polynomial).order(4))
        .apply(
            SVStatInput::new(SVColumnView::Dense(&[1.0, 2.0, 3.0]))
                .with_secondary(SVColumnView::Dense(&[0.0, 1.0, 2.0])),
        )
        .expect_err("a degree needs enough observations");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_logarithmic_fit_rejects_a_non_positive_independent_value() {
    let error = SVStat::fit(SVFitOptions::new().model(SVFitModel::Logarithmic))
        .apply(
            SVStatInput::new(SVColumnView::Dense(&[1.0, 2.0]))
                .with_secondary(SVColumnView::Dense(&[0.0, 1.0])),
        )
        .expect_err("a logarithm needs positive x");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_exponential_fit_rejects_a_non_positive_value() {
    let error = SVStat::fit(SVFitOptions::new().model(SVFitModel::Exponential))
        .apply(
            SVStatInput::new(SVColumnView::Dense(&[1.0, -2.0]))
                .with_secondary(SVColumnView::Dense(&[0.0, 1.0])),
        )
        .expect_err("a logarithm needs positive y");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_fit_is_independent_of_the_input_order() {
    let first = apply_fit(
        SVFitOptions::new(),
        &[0.0, 1.0, 2.0, 3.0],
        &[1.0, 3.0, 5.0, 7.0],
    );
    let second = apply_fit(
        SVFitOptions::new(),
        &[3.0, 2.0, 0.0, 1.0],
        &[7.0, 5.0, 1.0, 3.0],
    );

    assert_eq!(first.coefficients().len(), second.coefficients().len());
    for (left, right) in first.coefficients().iter().zip(second.coefficients()) {
        assert!((left - right).abs() < 1e-12);
    }
}
