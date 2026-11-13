//! Fitting one group through the stat kernel.

use super::composition::SVRegressionSeriesFit;
use super::configuration::{SVExtrapolationPolicy, SVRegressionConfiguration};
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_scale::SVColour;
use scienceview_core::sv_stat::{SVFitOptions, SVFitResult, SVStat, SVStatInput, SVStatResult};

/// Builds the fit options the configuration asks for.
pub(crate) fn sv_fit_configuration(configuration: &SVRegressionConfiguration) -> SVFitOptions {
    let options = SVFitOptions::new()
        .model(configuration.model)
        .order(configuration.order);
    match configuration.confidence {
        Some(confidence) => options.confidence(confidence),
        None => options,
    }
}

/// Applies the stat kernel to one group.
pub(crate) fn sv_apply_fit(
    options: SVFitOptions,
    x: &[f64],
    y: &[f64],
) -> Result<SVFitResult, SVError> {
    let input = SVStatInput::new(SVColumnView::Dense(y)).with_secondary(SVColumnView::Dense(x));
    match SVStat::fit(options).apply(input)? {
        SVStatResult::Fit(result) => Ok(result),
        other => Err(SVError::InvalidConfiguration {
            message: format!("the fit stat produced an unexpected result: {other:?}"),
        }),
    }
}

/// Turns a kernel result into the owned series frame.
pub(crate) fn sv_series_from_fit(
    label: String,
    colour: SVColour,
    result: SVFitResult,
    points_x: Vec<f64>,
    points_y: Vec<f64>,
    configuration: &SVRegressionConfiguration,
    windowed: bool,
) -> SVRegressionSeriesFit {
    let mut grid = result.grid().to_vec();
    let mut predicted = result.predicted().to_vec();
    let mut lower = result.lower().map(<[f64]>::to_vec);
    let mut upper = result.upper().map(<[f64]>::to_vec);

    if configuration.extrapolation == SVExtrapolationPolicy::ClipToVisibleDomain {
        if let Some((low, high)) = configuration.visible_x_domain {
            let keep: Vec<usize> = grid
                .iter()
                .enumerate()
                .filter(|(_, point)| (low..=high).contains(*point))
                .map(|(index, _)| index)
                .collect();
            grid = keep.iter().map(|index| grid[*index]).collect();
            predicted = keep.iter().map(|index| predicted[*index]).collect();
            lower = lower.map(|values| keep.iter().map(|index| values[*index]).collect());
            upper = upper.map(|values| keep.iter().map(|index| values[*index]).collect());
        }
    }

    SVRegressionSeriesFit {
        label,
        colour,
        coefficients: result.coefficients().to_vec(),
        grid,
        predicted,
        lower,
        upper,
        points_x,
        points_y,
        windowed: windowed || result.is_windowed(),
    }
}
