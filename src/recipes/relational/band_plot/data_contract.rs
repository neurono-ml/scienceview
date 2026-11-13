//! Resolving the documented columns into the bands the composition draws.
//!
//! The explicit mode borrows the three columns through their snapshots; the
//! derived mode reads them once, aggregates the value column with
//! [`SVStat::Aggregate`] and keeps the resulting estimate and interval. Sparse
//! storage is read through its stored entries and a streaming column through
//! the bounded window it already carries, so neither path can block.

use scienceview_core::sv_data::{SVColumnSnapshot, SVColumnView, SVRepresentation};
use scienceview_core::sv_error::SVError;

use super::column_access::{
    sv_count_finite, sv_group_rows, sv_positions, sv_series_name, sv_validate_columns, sv_widest,
};
use super::configuration::SVBandPlotConfiguration;
use super::derived_contract::sv_resolve_derived;

/// One band: its optional series name and its three data columns.
pub(crate) struct SVBand<'values> {
    pub(crate) name: Option<String>,
    pub(crate) x: SVBandColumn<'values>,
    pub(crate) lower: SVBandColumn<'values>,
    pub(crate) upper: SVBandColumn<'values>,
}

/// A column of a band, borrowed from the source or owned by a derived band.
pub(crate) enum SVBandColumn<'values> {
    /// A static source column, kept in its own representation.
    Snapshot(SVColumnSnapshot<'values>),
    /// An owned column produced by grouping or aggregation.
    Owned(Vec<f64>),
}

impl SVBandColumn<'_> {
    /// Borrows the column as one of the three representations.
    pub(crate) fn view(&mut self) -> SVColumnView<'_> {
        match self {
            Self::Snapshot(snapshot) => snapshot.column_view(),
            Self::Owned(values) => SVColumnView::Dense(values),
        }
    }
}

/// Every band of one operation, plus the facts the span records.
pub(crate) struct SVBandData<'values> {
    pub(crate) bands: Vec<SVBand<'values>>,
    pub(crate) representation: SVRepresentation,
    pub(crate) number_of_points: usize,
    pub(crate) is_approximate: bool,
}

/// Resolves the configuration into bands.
pub(crate) fn sv_resolve_band_data<'values>(
    configuration: &SVBandPlotConfiguration<'values>,
) -> Result<SVBandData<'values>, SVError> {
    sv_validate_columns(configuration)?;
    if configuration.derived {
        sv_resolve_derived(configuration)
    } else {
        sv_resolve_explicit(configuration)
    }
}

fn sv_resolve_explicit<'values>(
    configuration: &SVBandPlotConfiguration<'values>,
) -> Result<SVBandData<'values>, SVError> {
    let source = configuration.data_source;
    let lower_name = configuration.lower.unwrap_or_default();
    let upper_name = configuration.upper.unwrap_or_default();

    let Some(series_name) = configuration.series else {
        let mut x = source.column(configuration.x)?;
        let mut lower = source.column(lower_name)?;
        let mut upper = source.column(upper_name)?;
        let representation = sv_widest([
            x.representation(),
            lower.representation(),
            upper.representation(),
        ]);
        let number_of_points = {
            let x_view = x.column_view();
            let lower_view = lower.column_view();
            let upper_view = upper.column_view();
            let positions = sv_positions(x_view);
            sv_count_finite(&positions, x_view, lower_view, upper_view)
        };
        return Ok(SVBandData {
            bands: vec![SVBand {
                name: None,
                x: SVBandColumn::Snapshot(x),
                lower: SVBandColumn::Snapshot(lower),
                upper: SVBandColumn::Snapshot(upper),
            }],
            representation,
            number_of_points,
            is_approximate: representation == SVRepresentation::Streaming,
        });
    };

    let mut x_snapshot = source.column(configuration.x)?;
    let mut lower_snapshot = source.column(lower_name)?;
    let mut upper_snapshot = source.column(upper_name)?;
    let mut series_snapshot = source.column(series_name)?;
    let representation = sv_widest([
        x_snapshot.representation(),
        lower_snapshot.representation(),
        upper_snapshot.representation(),
        series_snapshot.representation(),
    ]);
    let x_view = x_snapshot.column_view();
    let lower_view = lower_snapshot.column_view();
    let upper_view = upper_snapshot.column_view();
    let series_view = series_snapshot.column_view();
    let positions = sv_positions(x_view);
    let series_values: Vec<f64> = positions
        .iter()
        .map(|row| series_view.value_at(*row).unwrap_or(f64::NAN))
        .collect();

    let mut bands: Vec<SVBand<'values>> = Vec::new();
    let mut number_of_points = 0;
    for (series_code, rows) in sv_group_rows(&series_values) {
        let row_positions: Vec<usize> = rows.iter().map(|row| positions[*row]).collect();
        let mut x = Vec::with_capacity(rows.len());
        let mut lower = Vec::with_capacity(rows.len());
        let mut upper = Vec::with_capacity(rows.len());
        for row in &row_positions {
            x.push(x_view.value_at(*row).unwrap_or(f64::NAN));
            lower.push(lower_view.value_at(*row).unwrap_or(f64::NAN));
            upper.push(upper_view.value_at(*row).unwrap_or(f64::NAN));
        }
        number_of_points += sv_count_finite(
            &(0..row_positions.len()).collect::<Vec<_>>(),
            SVColumnView::Dense(&x),
            SVColumnView::Dense(&lower),
            SVColumnView::Dense(&upper),
        );
        bands.push(SVBand {
            name: Some(sv_series_name(series_code)),
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
