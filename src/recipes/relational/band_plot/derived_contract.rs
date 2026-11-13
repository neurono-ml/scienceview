//! The derived mode: `SVStat::Aggregate` produces the estimate and interval.

use scienceview_core::sv_data::{SVColumnView, SVRepresentation};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::{
    SVAggregateMethod, SVAggregateOptions, SVStat, SVStatInput, SVStatResult,
};

use super::column_access::{sv_group_rows, sv_positions, sv_series_name, sv_widest};
use super::configuration::{SVBandPlotConfiguration, sv_invalid};
use super::data_contract::{SVBand, SVBandColumn, SVBandData};
pub(crate) fn sv_resolve_derived<'values>(
    configuration: &SVBandPlotConfiguration<'values>,
) -> Result<SVBandData<'values>, SVError> {
    let source = configuration.data_source;
    let mut x_snapshot = source.column(configuration.x)?;
    let value_name = configuration.values.unwrap_or_default();
    let mut value_snapshot = source.column(value_name)?;
    let mut series_snapshot = match configuration.series {
        Some(series_name) => Some(source.column(series_name)?),
        None => None,
    };
    let representations = match &mut series_snapshot {
        Some(series) => sv_widest([
            x_snapshot.representation(),
            value_snapshot.representation(),
            series.representation(),
        ]),
        None => sv_widest([x_snapshot.representation(), value_snapshot.representation()]),
    };
    let representation = representations;
    let x_view = x_snapshot.column_view();
    let value_view = value_snapshot.column_view();
    let series_view = series_snapshot.as_mut().map(|series| series.column_view());
    let positions = sv_positions(x_view);

    let x_values: Vec<f64> = positions
        .iter()
        .map(|row| x_view.value_at(*row).unwrap_or(f64::NAN))
        .collect();
    let values: Vec<f64> = positions
        .iter()
        .map(|row| value_view.value_at(*row).unwrap_or(f64::NAN))
        .collect();
    let series_values: Option<Vec<f64>> = series_view.map(|view| {
        positions
            .iter()
            .map(|row| view.value_at(*row).unwrap_or(f64::NAN))
            .collect()
    });

    let stat = SVStat::aggregate(
        SVAggregateOptions::new()
            .method(SVAggregateMethod::Mean)
            .error(configuration.error),
    );

    let mut bands = Vec::new();
    let mut number_of_points = 0;
    let groups: Vec<(f64, Vec<usize>)> = match &series_values {
        Some(series) => sv_group_rows(series),
        None => vec![(f64::NAN, (0..x_values.len()).collect())],
    };
    for (series_code, rows) in groups {
        let mut group_x: Vec<f64> = Vec::with_capacity(rows.len());
        let mut group_y: Vec<f64> = Vec::with_capacity(rows.len());
        for row in rows {
            let x_value = x_values[row];
            let y_value = values[row];
            if !x_value.is_finite() || !y_value.is_finite() {
                configuration
                    .missing_value_policy
                    .report(value_name, positions[row])?;
                continue;
            }
            group_x.push(x_value);
            group_y.push(y_value);
        }
        if group_x.is_empty() {
            continue;
        }
        let input = SVStatInput::new(SVColumnView::Dense(&group_y))
            .with_groups(SVColumnView::Dense(&group_x));
        let SVStatResult::Groups(result) = stat.apply(input)? else {
            return Err(sv_invalid("the band stat must produce aggregate groups"));
        };

        let mut x = Vec::with_capacity(result.groups().len());
        let mut lower = Vec::with_capacity(result.groups().len());
        let mut upper = Vec::with_capacity(result.groups().len());
        for group in result.groups() {
            let (Some(x_value), Some(interval)) = (group.group(), group.error()) else {
                configuration
                    .missing_value_policy
                    .report(value_name, x.len())?;
                continue;
            };
            x.push(x_value);
            lower.push(interval.lower());
            upper.push(interval.upper());
        }
        number_of_points += x
            .iter()
            .zip(&lower)
            .zip(&upper)
            .filter(|((x, lower), upper)| x.is_finite() && lower.is_finite() && upper.is_finite())
            .count();
        if x.is_empty() {
            continue;
        }
        bands.push(SVBand {
            name: if series_values.is_some() {
                Some(sv_series_name(series_code))
            } else {
                None
            },
            x: SVBandColumn::Owned(x),
            lower: SVBandColumn::Owned(lower),
            upper: SVBandColumn::Owned(upper),
        });
    }

    Ok(SVBandData {
        bands,
        representation,
        number_of_points,
        is_approximate: representation == SVRepresentation::Streaming,
    })
}
