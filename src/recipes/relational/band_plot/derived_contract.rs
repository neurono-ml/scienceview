//! The derived mode: `SVStat::Aggregate` produces the estimate and interval.

use scienceview_core::sv_data::{SVColumnView, SVRepresentation};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::{
    SVAggregateMethod, SVAggregateOptions, SVStat, SVStatInput, SVStatResult,
};

use super::column_access::{sv_group_rows, sv_read_column, sv_series_name, sv_widest};
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
    let x_values = sv_read_column(&mut x_snapshot);
    let values = sv_read_column(&mut value_snapshot);
    let series_values = series_snapshot.as_mut().map(sv_read_column);

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
        let group_x: Vec<f64> = rows.iter().map(|row| x_values[*row]).collect();
        let group_y: Vec<f64> = rows.iter().map(|row| values[*row]).collect();
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
        number_of_points += x.len();
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
