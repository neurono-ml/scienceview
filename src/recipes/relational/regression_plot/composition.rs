//! The composition: one fit per series, the domains and the colours.
//!
//! This is the recipe's whole mathematics, and it contains none of it: every
//! model, every coefficient and every confidence bound comes from
//! [`SVStat::Fit`](scienceview_core::sv_stat::SVStat::Fit). The module groups
//! the paired observations, asks the kernel, and derives the two domains the
//! scales need.

use super::configuration::SVRegressionConfiguration;
use super::domains::sv_domains;
use super::fitting::{sv_apply_fit, sv_fit_configuration, sv_series_from_fit};
use super::grouping::sv_group_positions;
use super::pairing::SVResolvedInputs;
use scienceview_core::sv_data::SVRepresentation;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_scale::{SVColour, sv_default_scale_palette};

/// One fitted series with its observations.
#[derive(Clone, Debug, PartialEq)]
pub struct SVRegressionSeriesFit {
    pub(crate) label: String,
    pub(crate) colour: SVColour,
    pub(crate) coefficients: Vec<f64>,
    pub(crate) grid: Vec<f64>,
    pub(crate) predicted: Vec<f64>,
    pub(crate) lower: Option<Vec<f64>>,
    pub(crate) upper: Option<Vec<f64>>,
    pub(crate) points_x: Vec<f64>,
    pub(crate) points_y: Vec<f64>,
    pub(crate) windowed: bool,
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
