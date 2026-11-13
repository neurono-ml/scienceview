//! Series grouping shared by the central panel and the legend.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, sv_default_scale_palette};

/// One series the central panel groups its marks into.
pub(super) struct SVJointSeriesGroup {
    pub(super) label: String,
    pub(super) colour: SVColour,
}

/// Resolves the series groups and the group index of every observation.
///
/// A sparse view is walked through `value_at`; the group index vector is an
/// index per logical observation, never an expansion of the column.
pub(super) fn sv_series_groups(
    series: Option<SVColumnView<'_>>,
    number_of_points: usize,
    fallback_colour: SVColour,
) -> Result<(Vec<SVJointSeriesGroup>, Vec<usize>), SVError> {
    let Some(series) = series else {
        return Ok((
            vec![SVJointSeriesGroup {
                label: String::new(),
                colour: fallback_colour,
            }],
            vec![0; number_of_points],
        ));
    };

    let palette = sv_default_scale_palette();
    let mut values: Vec<f64> = Vec::new();
    let mut groups: Vec<SVJointSeriesGroup> = Vec::new();
    let mut indices = Vec::with_capacity(number_of_points);
    for index in 0..number_of_points {
        let value = series
            .value_at(index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the series column ended before observation {index}"),
            })?;
        if !value.is_finite() {
            return Err(SVError::NonFiniteValue {
                operation: "joint_series",
                position: index,
            });
        }
        let group_index = match values.iter().position(|candidate| *candidate == value) {
            Some(group_index) => group_index,
            None => {
                values.push(value);
                groups.push(SVJointSeriesGroup {
                    label: sv_series_label(value),
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
