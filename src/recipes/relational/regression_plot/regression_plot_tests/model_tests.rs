//! Every documented model, its coefficients and the large-data path.

use super::super::SVRegressionPlotBuilder;
use super::{plot, table};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_stat::SVFitModel;

fn coefficients_for(model: SVFitModel, order: usize, y: &[f64]) -> Vec<f64> {
    let x: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let data = table(&x, y);
    let chart = SVRegressionPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .y("y")
        .model(model)
        .order(order)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is valid");
    let composition = chart.compose().expect("the fit runs");
    composition.series()[0].coefficients().to_vec()
}

fn assert_coefficients(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert!(
            (actual - expected).abs() < 1.0e-9,
            "coefficient {actual} differs from {expected}"
        );
    }
}

#[test]
fn every_documented_model_recovers_its_generating_coefficients() {
    let x: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let linear: Vec<f64> = x.iter().map(|value| 1.0 + 2.0 * value).collect();
    let polynomial: Vec<f64> = x.iter().map(|value| value * value).collect();
    let logarithmic: Vec<f64> = x.iter().map(|value| 1.0 + 2.0 * value.ln()).collect();
    let exponential: Vec<f64> = x.iter().map(|value| 2.0 * (0.5 * value).exp()).collect();
    let power: Vec<f64> = x.iter().map(|value| 2.0 * value.powf(1.5)).collect();

    assert_coefficients(
        &coefficients_for(SVFitModel::Linear, 1, &linear),
        &[1.0, 2.0],
    );
    assert_coefficients(
        &coefficients_for(SVFitModel::Polynomial, 2, &polynomial),
        &[0.0, 0.0, 1.0],
    );
    assert_coefficients(
        &coefficients_for(SVFitModel::Logarithmic, 1, &logarithmic),
        &[1.0, 2.0],
    );
    assert_coefficients(
        &coefficients_for(SVFitModel::Exponential, 1, &exponential),
        &[2.0, 0.5],
    );
    assert_coefficients(&coefficients_for(SVFitModel::Power, 1, &power), &[2.0, 1.5]);
}

#[test]
fn a_large_dataset_composes_with_the_kernel() {
    let x: Vec<f64> = (0..10_000).map(|index| index as f64 / 100.0).collect();
    let y: Vec<f64> = x.iter().map(|value| 1.0 + 2.0 * value).collect();
    let data = table(&x, &y);

    let composition = plot(&data).compose().expect("the large fit runs");
    assert_eq!(composition.point_count(), 10_000);
    assert!((composition.series()[0].coefficients()[1] - 2.0).abs() < 1.0e-9);
}
