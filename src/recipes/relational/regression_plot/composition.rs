//! The composition: one fit per series, the domains and the colours.
//!
//! This is the recipe's whole mathematics, and it contains none of it: every
//! model, every coefficient and every confidence bound comes from
//! [`SVStat::Fit`](scienceview_core::sv_stat::SVStat::Fit). The module groups
//! the paired observations, asks the kernel, and derives the two domains the
//! scales need.

use super::configuration::{SVExtrapolationPolicy, SVRegressionConfiguration};
use super::pairing::SVResolvedInputs;
use scienceview_core::sv_data::{SVColumnView, SVRepresentation};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_scale::{SVColour, sv_default_scale_palette};
use scienceview_core::sv_stat::{SVFitOptions, SVFitResult, SVStat, SVStatInput, SVStatResult};

/// One fitted series with its observations.
#[derive(Clone, Debug, PartialEq)]
pub struct SVRegressionSeriesFit {
    label: String,
    colour: SVColour,
    coefficients: Vec<f64>,
    grid: Vec<f64>,
    predicted: Vec<f64>,
    lower: Option<Vec<f64>>,
    upper: Option<Vec<f64>>,
    points_x: Vec<f64>,
    points_y: Vec<f64>,
    windowed: bool,
}

impl SVRegressionSeriesFit {
    /// Returns the series label; `"all"` when no grouping column was bound.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the colour shared by the points and the fit of this series.
    pub fn colour(&self) -> SVColour {
        self.colour
    }

    /// Returns the model coefficients, constant term first.
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    /// Returns the predictor values the curve was evaluated at.
    pub fn grid(&self) -> &[f64] {
        &self.grid
    }

    /// Returns the fitted value at each grid sample.
    pub fn predicted(&self) -> &[f64] {
        &self.predicted
    }

    /// Returns the lower confidence bound, when a band was requested.
    pub fn lower(&self) -> Option<&[f64]> {
        self.lower.as_deref()
    }

    /// Returns the upper confidence bound, when a band was requested.
    pub fn upper(&self) -> Option<&[f64]> {
        self.upper.as_deref()
    }

    /// Returns the predictor values of the observations in this series.
    pub fn points_x(&self) -> &[f64] {
        &self.points_x
    }

    /// Returns the response values of the observations in this series.
    pub fn points_y(&self) -> &[f64] {
        &self.points_y
    }

    /// Returns `true` when the fit describes only the current stream window.
    pub fn is_windowed(&self) -> bool {
        self.windowed
    }
}

/// Everything one render needs from the data, with no borrow left behind.
#[derive(Clone, Debug, PartialEq)]
pub struct SVRegressionComposition {
    series: Vec<SVRegressionSeriesFit>,
    x_domain: (f64, f64),
    y_domain: (f64, f64),
    representation: SVRepresentation,
    point_count: usize,
    dropped_pairs: usize,
}

impl SVRegressionComposition {
    /// Returns one fit per series.
    pub fn series(&self) -> &[SVRegressionSeriesFit] {
        &self.series
    }

    /// Returns the horizontal domain shared by every series.
    pub fn x_domain(&self) -> (f64, f64) {
        self.x_domain
    }

    /// Returns the vertical domain that contains the points and the band.
    pub fn y_domain(&self) -> (f64, f64) {
        self.y_domain
    }

    /// Returns the representation the input arrived in.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// Returns the number of observations that remain after pairing.
    pub fn point_count(&self) -> usize {
        self.point_count
    }

    /// Returns the number of pairs dropped for a non-finite value.
    pub fn dropped_pairs(&self) -> usize {
        self.dropped_pairs
    }

    /// Returns `true` when no series was fitted.
    pub fn is_empty(&self) -> bool {
        self.series.is_empty()
    }
}

/// Groups the paired observations and fits one model per group.
pub(crate) fn sv_compose(
    inputs: SVResolvedInputs,
    configuration: &SVRegressionConfiguration,
) -> Result<SVRegressionComposition, SVError> {
    if inputs.x.is_empty() {
        return Err(SVError::EmptyInput {
            operation: "regression plot",
        });
    }

    let groups = sv_group_positions(inputs.series.as_deref(), inputs.x.len());
    let palette = sv_default_scale_palette();
    let mut series = Vec::with_capacity(groups.len());
    for (index, (label, positions)) in groups.into_iter().enumerate() {
        let x: Vec<f64> = positions
            .iter()
            .map(|position| inputs.x[*position])
            .collect();
        let y: Vec<f64> = positions
            .iter()
            .map(|position| inputs.y[*position])
            .collect();
        let fit = sv_fit_configuration(configuration);
        let result = sv_apply_fit(fit, &x, &y)?;
        let windowed = inputs.representation == SVRepresentation::Streaming;
        series.push(sv_series_from_fit(
            label,
            palette.colour_at_index(index),
            result,
            x,
            y,
            configuration,
            windowed,
        ));
    }

    let (x_domain, y_domain) = sv_domains(&series)?;
    Ok(SVRegressionComposition {
        series,
        x_domain,
        y_domain,
        representation: inputs.representation,
        point_count: inputs.point_count(),
        dropped_pairs: inputs.dropped_pairs,
    })
}

/// Builds the fit options the configuration asks for.
fn sv_fit_configuration(configuration: &SVRegressionConfiguration) -> SVFitOptions {
    let options = SVFitOptions::new()
        .model(configuration.model)
        .order(configuration.order);
    match configuration.confidence {
        Some(confidence) => options.confidence(confidence),
        None => options,
    }
}

/// Applies the stat kernel to one group.
fn sv_apply_fit(options: SVFitOptions, x: &[f64], y: &[f64]) -> Result<SVFitResult, SVError> {
    let input = SVStatInput::new(SVColumnView::Dense(y)).with_secondary(SVColumnView::Dense(x));
    match SVStat::fit(options).apply(input)? {
        SVStatResult::Fit(result) => Ok(result),
        other => Err(SVError::InvalidConfiguration {
            message: format!("the fit stat produced an unexpected result: {other:?}"),
        }),
    }
}

/// Turns a kernel result into the owned series frame.
fn sv_series_from_fit(
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

/// Groups logical positions by group code, preserving first-appearance order.
fn sv_group_positions(series: Option<&[f64]>, length: usize) -> Vec<(String, Vec<usize>)> {
    let Some(codes) = series else {
        return vec![("all".to_string(), (0..length).collect())];
    };

    let mut groups: Vec<(f64, Vec<usize>)> = Vec::new();
    for (position, code) in codes.iter().copied().enumerate() {
        match groups
            .iter_mut()
            .find(|(existing, _)| existing.total_cmp(&code).is_eq())
        {
            Some((_, positions)) => positions.push(position),
            None => groups.push((code, vec![position])),
        }
    }
    groups
        .into_iter()
        .map(|(code, positions)| (sv_group_label(code), positions))
        .collect()
}

/// Formats a numeric group code without an avoidable decimal tail.
fn sv_group_label(code: f64) -> String {
    if code.fract() == 0.0 {
        format!("{code:.0}")
    } else {
        format!("{code}")
    }
}

/// The horizontal and vertical domains of one composition.
type SVDomains = ((f64, f64), (f64, f64));

/// Derives the two domains from the points, the curves and the bands.
fn sv_domains(series: &[SVRegressionSeriesFit]) -> Result<SVDomains, SVError> {
    let mut x_low = f64::INFINITY;
    let mut x_high = f64::NEG_INFINITY;
    let mut y_low = f64::INFINITY;
    let mut y_high = f64::NEG_INFINITY;

    let mut observe_x = |value: f64| {
        x_low = x_low.min(value);
        x_high = x_high.max(value);
    };
    let mut observe_y = |value: f64| {
        y_low = y_low.min(value);
        y_high = y_high.max(value);
    };

    for group in series {
        for point in group.points_x.iter().copied() {
            observe_x(point);
        }
        for point in group.points_y.iter().copied() {
            observe_y(point);
        }
        for point in group.grid.iter().copied() {
            observe_x(point);
        }
        for point in group.predicted.iter().copied() {
            observe_y(point);
        }
        if let Some(values) = &group.lower {
            for point in values.iter().copied() {
                observe_y(point);
            }
        }
        if let Some(values) = &group.upper {
            for point in values.iter().copied() {
                observe_y(point);
            }
        }
    }

    if !x_low.is_finite() || !x_high.is_finite() || !y_low.is_finite() || !y_high.is_finite() {
        return Err(SVError::InvalidConfiguration {
            message: "the regression plot has no finite domain to draw".to_string(),
        });
    }

    Ok((sv_padded(x_low, x_high), sv_padded(y_low, y_high)))
}

/// Pads a degenerate or zero-width domain so a scale accepts it.
fn sv_padded(low: f64, high: f64) -> (f64, f64) {
    if high > low {
        let padding = (high - low) * 0.05;
        (low - padding, high + padding)
    } else {
        let padding = if low.abs() <= f64::EPSILON {
            1.0
        } else {
            low.abs() * 0.1
        };
        (low - padding, high + padding)
    }
}
