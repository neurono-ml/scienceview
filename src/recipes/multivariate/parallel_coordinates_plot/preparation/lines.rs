//! The polyline extraction of one operation.

use super::super::configuration::{
    SV_PARALLEL_COORDINATES_PLOT_TOP, SVParallelCoordinatesConfiguration,
};
use super::super::model::{SVParallelCoordinatesAxis, SVParallelCoordinatesLine};
use super::super::options::SVParallelCoordinatesMissingPolicy;
use super::SVResolvedColumns;
use super::SVVisibleAxis;
use super::colour_groups::SVColourGroups;
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkPoint;
use crate::sv_scale::{SVColour, SVScaleValue, sv_default_scale_palette};

pub(super) fn sv_build_lines(
    configuration: &SVParallelCoordinatesConfiguration<'_>,
    columns: &mut SVResolvedColumns<'_>,
    visible: &[SVVisibleAxis<'_>],
    axes: &[SVParallelCoordinatesAxis],
    groups: &SVColourGroups,
) -> Result<Vec<SVParallelCoordinatesLine>, SVError> {
    let mut lines = Vec::new();
    for observation in 0..columns.length {
        let (group, colour) = match columns.colour.as_mut() {
            Some(snapshot) => {
                let value = {
                    let view = snapshot.column_view();
                    view.value_at(observation).unwrap_or(0.0)
                };
                groups.resolve(value)
            }
            None => (None, sv_default_scale_palette().colour_at_index(0)),
        };
        let points = sv_observation_points(columns, visible, axes, observation)?;
        sv_push_observation_lines(
            &mut lines,
            observation,
            group,
            colour,
            &points,
            configuration.missing,
        )?;
    }
    Ok(lines)
}

fn sv_observation_points(
    columns: &mut SVResolvedColumns<'_>,
    visible: &[SVVisibleAxis<'_>],
    axes: &[SVParallelCoordinatesAxis],
    observation: usize,
) -> Result<Vec<Option<SVMarkPoint>>, SVError> {
    visible
        .iter()
        .zip(axes.iter())
        .map(|(visible_axis, axis)| {
            let value = {
                let view = columns.variables[visible_axis.variable_index].column_view();
                view.value_at(observation).unwrap_or(0.0)
            };
            if !value.is_finite() {
                return Ok(None);
            }
            let y = axis.scale().position(SVScaleValue::Numeric(value))?;
            Ok(Some(SVMarkPoint::new(
                axis.position(),
                SV_PARALLEL_COORDINATES_PLOT_TOP + y,
            )))
        })
        .collect()
}

fn sv_push_observation_lines(
    lines: &mut Vec<SVParallelCoordinatesLine>,
    observation: usize,
    group: Option<String>,
    colour: SVColour,
    points: &[Option<SVMarkPoint>],
    missing: SVParallelCoordinatesMissingPolicy,
) -> Result<(), SVError> {
    match missing {
        SVParallelCoordinatesMissingPolicy::Bridge => {
            let vertices: Vec<SVMarkPoint> = points.iter().flatten().copied().collect();
            sv_push_run(lines, observation, &group, colour, vertices);
        }
        SVParallelCoordinatesMissingPolicy::DropObservation => {
            if points.iter().all(Option::is_some) {
                let vertices: Vec<SVMarkPoint> = points.iter().flatten().copied().collect();
                sv_push_run(lines, observation, &group, colour, vertices);
            }
        }
        SVParallelCoordinatesMissingPolicy::Error => {
            if points.iter().any(Option::is_none) {
                return Err(SVError::NonFiniteValue {
                    operation: "parallel coordinates plot",
                    position: observation,
                });
            }
            let vertices: Vec<SVMarkPoint> = points.iter().flatten().copied().collect();
            sv_push_run(lines, observation, &group, colour, vertices);
        }
        SVParallelCoordinatesMissingPolicy::Gap => {
            let mut run = Vec::new();
            for point in points {
                match point {
                    Some(point) => run.push(*point),
                    None => {
                        sv_push_run(lines, observation, &group, colour, std::mem::take(&mut run));
                    }
                }
            }
            sv_push_run(lines, observation, &group, colour, run);
        }
    }
    Ok(())
}

fn sv_push_run(
    lines: &mut Vec<SVParallelCoordinatesLine>,
    observation: usize,
    group: &Option<String>,
    colour: SVColour,
    vertices: Vec<SVMarkPoint>,
) {
    if vertices.is_empty() {
        return;
    }
    lines.push(SVParallelCoordinatesLine::new(
        observation,
        group.clone(),
        colour,
        vertices,
    ));
}
