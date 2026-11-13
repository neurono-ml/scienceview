//! The composition pass: resolve columns, bin the lattice, paint the cells.

use super::super::configuration::{
    SV_HEXBIN_PLOT_HEIGHT, SV_HEXBIN_PLOT_LEFT, SV_HEXBIN_PLOT_TOP, SV_HEXBIN_PLOT_WIDTH,
    SVHexbinConfiguration,
};
use super::super::decorations::{sv_build_decorations, sv_viewport};
use super::super::{SVHexbinAggregation, SVHexbinCell};
use super::definition::SVHexbinComposition;
use super::geometry::sv_build_geometry;
use super::lattice::{SV_HEXBIN_SQRT_THREE, SVHexbinDomain, SVHexbinLattice};
use crate::sv_data::{SVColumnSnapshot, SVColumnView, SVDataSource};
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_scale::{
    SVContinuousColourScale, SVContinuousColourScaleBuilder, SVScale, SVScaleBuilder, SVScaleKind,
};
use crate::sv_stat::{SVAggregateMethod, SVAggregateOptions, SVStat, SVStatInput, SVStatResult};

pub(crate) fn sv_compose(
    configuration: &SVHexbinConfiguration<'_>,
) -> Result<SVHexbinComposition, SVError> {
    let mut horizontal_snapshot = configuration.data_source.column(&configuration.x)?;
    let mut vertical_snapshot = configuration.data_source.column(&configuration.y)?;
    let mut series_snapshot =
        sv_optional_snapshot(configuration.data_source, &configuration.series)?;
    let mut value_snapshot = sv_optional_snapshot(configuration.data_source, &configuration.value)?;

    let horizontal = horizontal_snapshot.column_view();
    let vertical = vertical_snapshot.column_view();
    let series = series_snapshot.as_mut().map(SVColumnSnapshot::column_view);
    let value = value_snapshot.as_mut().map(SVColumnSnapshot::column_view);
    let number_of_points = horizontal.length();
    sv_check_length("x", number_of_points, "y", vertical.length())?;
    if let Some(series) = series {
        sv_check_length("x", number_of_points, "series", series.length())?;
    }
    if let Some(value) = value {
        sv_check_length("x", number_of_points, "value", value.length())?;
    }

    let domain = sv_resolve_domain(configuration, horizontal, vertical)?;
    let mut lattice = SVHexbinLattice::new(domain);
    let mean = configuration.aggregation == SVHexbinAggregation::Mean;
    let mut codes: Vec<f64> = Vec::new();
    let mut aggregated_values: Vec<f64> = Vec::new();
    let mut number_of_observations = 0usize;

    for index in 0..number_of_points {
        let x = sv_read(horizontal, index, &configuration.x)?;
        let y = sv_read(vertical, index, &configuration.y)?;
        let series_value = match series {
            Some(series) => Some(sv_read(series, index, "series")?),
            None => None,
        };
        let value_at = match value {
            Some(value) => Some(sv_read(value, index, "value")?),
            None => None,
        };
        let absent = !x.is_finite()
            || !y.is_finite()
            || series_value.is_some_and(|series| !series.is_finite())
            || (mean && !value_at.is_some_and(f64::is_finite));
        if absent {
            configuration.missing_value_policy.report(index)?;
            continue;
        }
        let scalar = value_at.unwrap_or(1.0);
        let cell_index = lattice.deposit(x, y, series_value, scalar);
        if mean {
            codes.push(cell_index as f64);
            aggregated_values.push(scalar);
        }
        number_of_observations += 1;
    }

    if mean && number_of_observations > 0 {
        sv_apply_mean(configuration, &mut lattice, &aggregated_values, &codes)?;
    }

    let mut cells = sv_materialise_cells(configuration, &lattice, number_of_observations);
    let colour_scale = sv_colour_scale(configuration, cells.iter().map(|cell| cell.value))?;
    for cell in &mut cells {
        cell.colour = colour_scale.colour(cell.value)?;
    }

    let plot = SVResolvedRectangle::new(
        SV_HEXBIN_PLOT_LEFT,
        SV_HEXBIN_PLOT_TOP,
        SV_HEXBIN_PLOT_WIDTH,
        SV_HEXBIN_PLOT_HEIGHT,
    );
    let horizontal_scale = sv_position_scale(domain.x_lower, domain.x_upper, plot.x, plot.right())?;
    let vertical_scale = sv_position_scale(domain.y_lower, domain.y_upper, plot.bottom(), plot.y)?;
    let mark_geometries = sv_build_geometry(
        &cells,
        domain,
        &horizontal_scale,
        &vertical_scale,
        configuration.orientation,
    )?;
    let number_of_series = sv_number_of_series(&cells);
    let style = SVDecorationStyle::built_in();
    let (decorations_before_marks, decorations_after_marks) = sv_build_decorations(
        configuration,
        &horizontal_scale,
        &vertical_scale,
        &colour_scale,
        plot,
        &style,
    )?;

    Ok(SVHexbinComposition {
        backend_kind: configuration.backend_kind,
        representation: configuration.representation,
        viewport: sv_viewport(),
        plot_area: plot,
        background: style.background(),
        title: configuration.title.clone(),
        horizontal_scale,
        vertical_scale,
        colour_scale,
        number_of_drawn_marks: cells.len(),
        cells,
        number_of_observations,
        number_of_series,
        decorations_before_marks,
        decorations_after_marks,
        mark_geometries,
    })
}

fn sv_apply_mean(
    configuration: &SVHexbinConfiguration<'_>,
    lattice: &mut SVHexbinLattice,
    values: &[f64],
    codes: &[f64],
) -> Result<(), SVError> {
    let options = SVAggregateOptions::new().method(SVAggregateMethod::Mean);
    let input =
        SVStatInput::new(SVColumnView::Dense(values)).with_groups(SVColumnView::Dense(codes));
    let result = SVStat::aggregate(options).apply(input)?;
    let SVStatResult::Groups(groups) = result else {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "the `{}` aggregation needs the group result of the aggregate stat",
                configuration.aggregation.as_str()
            ),
        });
    };
    for group in groups.groups() {
        let Some(code) = group.group() else {
            continue;
        };
        let Ok(index) = usize::try_from(code.round() as i64) else {
            continue;
        };
        if let Some(entry) = lattice.entries_mut().get_mut(index) {
            entry.count = group.number_of_observations();
            entry.sum = group.value() * entry.count as f64;
        }
    }
    Ok(())
}

fn sv_materialise_cells(
    configuration: &SVHexbinConfiguration<'_>,
    lattice: &SVHexbinLattice,
    number_of_observations: usize,
) -> Vec<SVHexbinCell> {
    let area = sv_cell_area(lattice.domain().radius);
    let mut cells = Vec::new();
    for entry in lattice.entries() {
        if entry.count < configuration.min_count {
            continue;
        }
        let value = match configuration.aggregation {
            SVHexbinAggregation::Count => sv_normalise(
                entry.count as f64,
                number_of_observations,
                area,
                configuration.normalisation,
            ),
            SVHexbinAggregation::Mean => {
                if entry.count > 0 {
                    entry.sum / entry.count as f64
                } else {
                    0.0
                }
            }
        };
        cells.push(SVHexbinCell {
            column: entry.key.column,
            row: entry.key.row,
            series: entry.key.series_value(),
            centre_x: entry.centre_x,
            centre_y: entry.centre_y,
            value,
            observations: entry.count,
            colour: crate::sv_scale::SVColour::new(0, 0, 0, 255),
        });
    }
    cells
}

fn sv_colour_scale(
    configuration: &SVHexbinConfiguration<'_>,
    values: impl Iterator<Item = f64>,
) -> Result<SVContinuousColourScale, SVError> {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for value in values {
        if value.is_finite() {
            lower = lower.min(value);
            upper = upper.max(value);
        }
    }
    if !lower.is_finite() || !upper.is_finite() {
        lower = 0.0;
        upper = 1.0;
    }
    if upper <= lower {
        upper = lower + 1.0;
    }
    SVContinuousColourScaleBuilder::new()
        .domain(lower, upper)
        .transform(configuration.colour_transform)
        .colour_map(configuration.colour_map.clone())
        .nice(false)
        .build()
}

fn sv_resolve_domain(
    configuration: &SVHexbinConfiguration<'_>,
    horizontal: SVColumnView<'_>,
    vertical: SVColumnView<'_>,
) -> Result<SVHexbinDomain, SVError> {
    if let Some((x_lower, x_upper, y_lower, y_upper)) = configuration.extent {
        return SVHexbinDomain::new(x_lower, x_upper, y_lower, y_upper, configuration.grid_size);
    }
    let mut x_lower = f64::INFINITY;
    let mut x_upper = f64::NEG_INFINITY;
    let mut y_lower = f64::INFINITY;
    let mut y_upper = f64::NEG_INFINITY;
    for index in 0..horizontal.length() {
        let x = horizontal.value_at(index).unwrap_or(f64::NAN);
        let y = vertical.value_at(index).unwrap_or(f64::NAN);
        if x.is_finite() && y.is_finite() {
            x_lower = x_lower.min(x);
            x_upper = x_upper.max(x);
            y_lower = y_lower.min(y);
            y_upper = y_upper.max(y);
        }
    }
    if !x_lower.is_finite() {
        x_lower = 0.0;
        x_upper = 1.0;
        y_lower = 0.0;
        y_upper = 1.0;
    }
    SVHexbinDomain::new(x_lower, x_upper, y_lower, y_upper, configuration.grid_size)
}

fn sv_position_scale(
    lower: f64,
    upper: f64,
    start: f64,
    end: f64,
) -> Result<SVScale<'static>, SVError> {
    let (lower, upper) = if upper > lower {
        (lower, upper)
    } else {
        (lower - 0.5, lower + 0.5)
    };
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(lower, upper)
        .range(start, end)
        .build()
}

fn sv_number_of_series(cells: &[SVHexbinCell]) -> usize {
    let mut series: Vec<Option<u64>> = Vec::new();
    for cell in cells {
        let bits = cell.series.map(f64::to_bits);
        if !series.contains(&bits) {
            series.push(bits);
        }
    }
    series.len().max(1)
}

fn sv_cell_area(radius: f64) -> f64 {
    1.5 * SV_HEXBIN_SQRT_THREE * radius * radius
}

fn sv_normalise(
    count: f64,
    total: usize,
    area: f64,
    normalisation: crate::sv_stat::SVNormalisation,
) -> f64 {
    match normalisation {
        crate::sv_stat::SVNormalisation::Count => count,
        crate::sv_stat::SVNormalisation::Probability => {
            if total > 0 {
                count / total as f64
            } else {
                0.0
            }
        }
        crate::sv_stat::SVNormalisation::Density => {
            if total > 0 && area > 0.0 {
                count / (total as f64 * area)
            } else {
                0.0
            }
        }
    }
}

fn sv_check_length(
    _first_name: &str,
    first_length: usize,
    second_name: &str,
    second_length: usize,
) -> Result<(), SVError> {
    if first_length == second_length {
        Ok(())
    } else {
        Err(SVError::MismatchedLengths {
            name: second_name.to_string(),
            expected: first_length,
            actual: second_length,
        })
    }
}

fn sv_read(view: SVColumnView<'_>, index: usize, name: &str) -> Result<f64, SVError> {
    view.value_at(index)
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: format!("the `{name}` column ended before observation {index}"),
        })
}

fn sv_optional_snapshot<'source>(
    data_source: &'source dyn SVDataSource,
    name: &Option<String>,
) -> Result<Option<SVColumnSnapshot<'source>>, SVError> {
    match name {
        Some(name) => Ok(Some(data_source.column(name)?)),
        None => Ok(None),
    }
}
