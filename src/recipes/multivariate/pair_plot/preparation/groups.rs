//! The hue categories and the observations they select.

use super::super::configuration::SVPairConfiguration;
use super::value::{sv_report_missing, sv_value_at};
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, sv_default_scale_palette};

/// One hue category the grid paints.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVPairHueGroup {
    pub(crate) value: f64,
    pub(crate) label: String,
    pub(crate) colour: SVColour,
}

/// The hue groups and the group index of every observation.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVPairGroups {
    pub(crate) groups: Vec<SVPairHueGroup>,
    pub(crate) indices: Vec<usize>,
}

pub(crate) fn sv_hue_groups(
    configuration: &SVPairConfiguration<'_>,
) -> Result<SVPairGroups, SVError> {
    let Some(hue) = configuration.hue.as_deref() else {
        return Ok(SVPairGroups {
            groups: vec![SVPairHueGroup {
                value: 0.0,
                label: String::new(),
                colour: crate::sv_mark::SVMarkTheme::default().mark_colour(),
            }],
            indices: vec![0; configuration.number_of_points],
        });
    };

    let mut snapshot = configuration.data_source.column(hue)?;
    let view = snapshot.column_view();
    let palette = sv_default_scale_palette();
    let mut groups: Vec<SVPairHueGroup> = Vec::new();
    let mut indices = Vec::with_capacity(view.length());
    for index in 0..view.length() {
        let value = sv_value_at(view, index)?;
        if !value.is_finite() {
            sv_report_missing(configuration.missing_value_policy, hue, index)?;
            // The index vector is positional: keep one slot per logical
            // observation so a missing hue never shifts the group of the rows
            // that follow it. The readers skip this row because its hue is not
            // finite.
            indices.push(usize::MAX);
            continue;
        }
        let group_index = match groups.iter().position(|group| group.value == value) {
            Some(group_index) => group_index,
            None => {
                groups.push(SVPairHueGroup {
                    value,
                    label: sv_label(value),
                    colour: palette.colour_at_index(groups.len()),
                });
                groups.len() - 1
            }
        };
        indices.push(group_index);
    }
    Ok(SVPairGroups { groups, indices })
}

pub(crate) fn sv_group_values(
    configuration: &SVPairConfiguration<'_>,
    variable: &str,
    group_index: usize,
    groups: &SVPairGroups,
) -> Result<Vec<f64>, SVError> {
    let mut values_snapshot = configuration.data_source.column(variable)?;
    let values = values_snapshot.column_view();
    let mut hue_snapshot = match configuration.hue.as_deref() {
        Some(hue) => Some(configuration.data_source.column(hue)?),
        None => None,
    };
    let hue = hue_snapshot.as_mut().map(|snapshot| snapshot.column_view());

    let mut selected = Vec::new();
    for index in 0..values.length() {
        let value = sv_value_at(values, index)?;
        if !value.is_finite() {
            sv_report_missing(configuration.missing_value_policy, variable, index)?;
            continue;
        }
        if groups.indices.get(index).copied().unwrap_or(0) != group_index {
            continue;
        }
        if let Some(hue) = hue {
            if !sv_value_at(hue, index)?.is_finite() {
                continue;
            }
        }
        selected.push(value);
    }
    Ok(selected)
}

fn sv_label(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}
