//! The finite observations of one off-diagonal cell.

use super::super::configuration::SVPairConfiguration;
use super::groups::SVPairGroups;
use super::value::{sv_report_missing, sv_value_at};
use crate::sv_error::SVError;

/// The finite observations one cell draws, with their hue group.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SVPairCellPoints {
    pub(crate) horizontal: Vec<f64>,
    pub(crate) vertical: Vec<f64>,
    pub(crate) groups: Vec<usize>,
}

pub(crate) fn sv_cell_points(
    configuration: &SVPairConfiguration<'_>,
    horizontal_name: &str,
    vertical_name: &str,
    groups: &SVPairGroups,
) -> Result<SVPairCellPoints, SVError> {
    let mut horizontal_snapshot = configuration.data_source.column(horizontal_name)?;
    let mut vertical_snapshot = configuration.data_source.column(vertical_name)?;
    let horizontal = horizontal_snapshot.column_view();
    let vertical = vertical_snapshot.column_view();
    let mut hue_snapshot = match configuration.hue.as_deref() {
        Some(hue) => Some(configuration.data_source.column(hue)?),
        None => None,
    };
    let hue = hue_snapshot.as_mut().map(|snapshot| snapshot.column_view());
    let length = horizontal
        .length()
        .min(vertical.length())
        .min(hue.map_or(usize::MAX, |view| view.length()))
        .min(groups.indices.len());

    let mut points = SVPairCellPoints {
        horizontal: Vec::with_capacity(length),
        vertical: Vec::with_capacity(length),
        groups: Vec::with_capacity(length),
    };
    for index in 0..length {
        let horizontal_value = sv_value_at(horizontal, index)?;
        let vertical_value = sv_value_at(vertical, index)?;
        let hue_value = match hue {
            Some(hue) => sv_value_at(hue, index)?,
            None => 0.0,
        };
        if !horizontal_value.is_finite() {
            sv_report_missing(configuration.missing_value_policy, horizontal_name, index)?;
            continue;
        }
        if !vertical_value.is_finite() {
            sv_report_missing(configuration.missing_value_policy, vertical_name, index)?;
            continue;
        }
        if !hue_value.is_finite() && configuration.hue.is_some() {
            sv_report_missing(
                configuration.missing_value_policy,
                configuration.hue.as_deref().unwrap_or_default(),
                index,
            )?;
            continue;
        }
        points.horizontal.push(horizontal_value);
        points.vertical.push(vertical_value);
        points
            .groups
            .push(groups.indices.get(index).copied().unwrap_or(0));
    }
    Ok(points)
}
