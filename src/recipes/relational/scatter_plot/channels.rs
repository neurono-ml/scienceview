//! Per-point channel resolution: series, colour, size and jitter.
//!
//! Every helper reads a column through [`SVColumnView::value_at`], which answers
//! from a dense slice, a sparse index search or a streaming window without ever
//! densifying the column.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_scale::{
    SVColour, SVContinuousColourScale, SVContinuousColourScaleBuilder, SVScale, SVScaleBuilder,
    SVScaleKind, SVScaleValue, sv_default_colour_map, sv_default_scale_palette,
};

/// One series the scatter groups its marks into.
pub(super) struct SVScatterSeriesGroup {
    pub(super) value: f64,
    pub(super) label: String,
    pub(super) colour: SVColour,
}

/// Resolves the series groups and the group index of every point.
pub(super) fn sv_series_groups(
    series: Option<SVColumnView<'_>>,
    labels: &[String],
    number_of_points: usize,
    fallback_colour: SVColour,
) -> Result<(Vec<SVScatterSeriesGroup>, Vec<usize>), SVError> {
    let Some(series) = series else {
        return Ok((
            vec![SVScatterSeriesGroup {
                value: 0.0,
                label: String::new(),
                colour: fallback_colour,
            }],
            vec![0; number_of_points],
        ));
    };

    let palette = sv_default_scale_palette();
    let mut groups: Vec<SVScatterSeriesGroup> = Vec::new();
    let mut indices = Vec::with_capacity(number_of_points);
    for index in 0..number_of_points {
        let value = series
            .value_at(index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the series column ended before point {index}"),
            })?;
        if !value.is_finite() {
            return Err(SVError::NonFiniteValue {
                operation: "scatter_series",
                position: index,
            });
        }
        let group_index = match groups.iter().position(|group| group.value == value) {
            Some(group_index) => group_index,
            None => {
                let label = labels
                    .get(groups.len())
                    .cloned()
                    .unwrap_or_else(|| sv_series_label(value));
                groups.push(SVScatterSeriesGroup {
                    value,
                    label,
                    colour: palette.colour_at_index(groups.len()),
                });
                groups.len() - 1
            }
        };
        indices.push(group_index);
    }

    Ok((groups, indices))
}

fn sv_series_label(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}

/// Builds the continuous colour override from the `colour` column.
pub(super) fn sv_colour_scale(
    colour: SVColumnView<'_>,
) -> Result<SVContinuousColourScale, SVError> {
    SVContinuousColourScaleBuilder::new()
        .data_view(colour)
        .colour_map(sv_default_colour_map())
        .build()
}

/// Resolves the colour of every point, applying the column override last.
pub(super) fn sv_point_colours(
    groups: &[SVScatterSeriesGroup],
    group_indices: &[usize],
    colour_values: Option<SVColumnView<'_>>,
    colour_scale: Option<&SVContinuousColourScale>,
) -> Result<Vec<SVColour>, SVError> {
    let Some(colour_values) = colour_values else {
        return Ok(group_indices
            .iter()
            .map(|group_index| {
                groups
                    .get(*group_index)
                    .map(|group| group.colour)
                    .unwrap_or_else(|| SVColour::new(0, 0, 0, 255))
            })
            .collect());
    };
    let colour_scale = colour_scale.ok_or_else(|| SVError::InvalidConfiguration {
        message: "a colour column requires a continuous colour scale".to_string(),
    })?;
    let mut colours = Vec::with_capacity(colour_values.length());
    for index in 0..colour_values.length() {
        let value = colour_values
            .value_at(index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the colour column ended before point {index}"),
            })?;
        colours.push(colour_scale.colour(value)?);
    }
    Ok(colours)
}

/// Builds the continuous size scale from the `size` column.
pub(super) fn sv_size_scale(
    size: SVColumnView<'_>,
    minimum: f64,
    maximum: f64,
) -> Result<SVScale<'_>, SVError> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .data_view(size)
        .range(minimum, maximum)
        .build()
}

/// Resolves the diameter of every point.
pub(super) fn sv_point_sizes(
    size_values: Option<SVColumnView<'_>>,
    size_scale: Option<&SVScale<'_>>,
    base_size: f64,
) -> Result<Vec<f64>, SVError> {
    let (Some(size_values), Some(size_scale)) = (size_values, size_scale) else {
        return Ok(Vec::new());
    };
    let mut sizes = Vec::with_capacity(size_values.length());
    for index in 0..size_values.length() {
        let value = size_values
            .value_at(index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the size column ended before point {index}"),
            })?;
        let diameter = size_scale.position(SVScaleValue::Numeric(value))?;
        sizes.push(diameter.max(0.1));
    }
    if sizes.is_empty() {
        sizes.push(base_size);
    }
    Ok(sizes)
}
