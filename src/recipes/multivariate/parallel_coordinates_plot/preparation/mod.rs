//! Resolving the configured columns into axes, colour groups and polylines.

pub(super) mod axes;
mod colour_groups;
mod lines;

use self::axes::sv_build_axes;
use self::colour_groups::{SVColourGroups, sv_colour_groups};
use self::lines::sv_build_lines;
use super::configuration::{
    SV_PARALLEL_COORDINATES_PLOT_HEIGHT, SV_PARALLEL_COORDINATES_PLOT_LEFT,
    SV_PARALLEL_COORDINATES_PLOT_TOP, SV_PARALLEL_COORDINATES_PLOT_WIDTH,
    SV_PARALLEL_COORDINATES_VIEWPORT_HEIGHT, SV_PARALLEL_COORDINATES_VIEWPORT_WIDTH,
    SVParallelCoordinatesConfiguration,
};
use super::model::{SVParallelCoordinatesAxis, SVParallelCoordinatesLine};
use super::options::SVDimension;
use crate::sv_data::SVColumnSnapshot;
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};

/// One visible axis before it is scaled.
#[derive(Clone, Copy)]
pub(super) struct SVVisibleAxis<'values> {
    pub(super) variable: &'values str,
    pub(super) variable_index: usize,
    pub(super) range_override: Option<(f64, f64)>,
    pub(super) coloured: bool,
}

/// The values one operation resolved from the data source.
pub(super) struct SVResolvedColumns<'values> {
    pub(super) variables: Vec<SVColumnSnapshot<'values>>,
    pub(super) colour: Option<SVColumnSnapshot<'values>>,
    pub(super) length: usize,
}

/// Everything the composition needs, in data space.
pub(super) struct SVParallelCoordinatesPrepared {
    pub(super) viewport: SVViewport,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) axes: Vec<SVParallelCoordinatesAxis>,
    pub(super) lines: Vec<SVParallelCoordinatesLine>,
    pub(super) colour_groups: Vec<(String, crate::sv_scale::SVColour)>,
    pub(super) number_of_vertices: usize,
}

pub(super) fn sv_prepare(
    configuration: &SVParallelCoordinatesConfiguration<'_>,
) -> Result<SVParallelCoordinatesPrepared, SVError> {
    let mut columns = sv_resolve_columns(configuration)?;
    let visible = sv_visible_axes(configuration);
    let axes = sv_build_axes(configuration, &mut columns, &visible)?;
    let colour_groups: SVColourGroups = sv_colour_groups(&mut columns)?;
    let lines = sv_build_lines(configuration, &mut columns, &visible, &axes, &colour_groups)?;
    let number_of_vertices = lines.iter().map(|line| line.vertices().len()).sum();

    Ok(SVParallelCoordinatesPrepared {
        viewport: SVViewport::new(
            SV_PARALLEL_COORDINATES_VIEWPORT_WIDTH,
            SV_PARALLEL_COORDINATES_VIEWPORT_HEIGHT,
        ),
        plot_area: SVResolvedRectangle::new(
            SV_PARALLEL_COORDINATES_PLOT_LEFT,
            SV_PARALLEL_COORDINATES_PLOT_TOP,
            SV_PARALLEL_COORDINATES_PLOT_WIDTH,
            SV_PARALLEL_COORDINATES_PLOT_HEIGHT,
        ),
        axes,
        lines,
        colour_groups: colour_groups.entries,
        number_of_vertices,
    })
}

fn sv_resolve_columns<'values>(
    configuration: &SVParallelCoordinatesConfiguration<'values>,
) -> Result<SVResolvedColumns<'values>, SVError> {
    let variables = configuration
        .variables
        .iter()
        .map(|name| configuration.data_source.column(name))
        .collect::<Result<Vec<_>, _>>()?;
    let colour = configuration
        .colour
        .map(|name| configuration.data_source.column(name))
        .transpose()?;
    let length = configuration.number_of_observations;
    Ok(SVResolvedColumns {
        variables,
        colour,
        length,
    })
}

fn sv_visible_axes<'values>(
    configuration: &SVParallelCoordinatesConfiguration<'values>,
) -> Vec<SVVisibleAxis<'values>> {
    let dimensions: &[SVDimension<'values>] = &configuration.dimensions;
    let mut visible = Vec::new();
    for dimension in dimensions.iter().filter(|dimension| dimension.is_visible()) {
        let positional = configuration
            .range
            .as_ref()
            .and_then(|range| range.get(visible.len()))
            .copied();
        let variable_index = configuration
            .variables
            .iter()
            .position(|name| *name == dimension.variable())
            .unwrap_or(0);
        visible.push(SVVisibleAxis {
            variable: dimension.variable(),
            variable_index,
            range_override: dimension.range_override().or(positional),
            coloured: dimension.is_coloured(),
        });
    }
    visible
}
