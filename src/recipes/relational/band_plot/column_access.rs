//! Reading, grouping and validating the columns the recipe consumes.

use scienceview_core::sv_data::{SVColumnRole, SVColumnSnapshot, SVRepresentation, SVSourceSchema};
use scienceview_core::sv_error::SVError;

use super::configuration::{SVBandPlotConfiguration, sv_invalid};
use super::data_contract::SVBandColumn;
pub(crate) fn sv_validate_columns(
    configuration: &SVBandPlotConfiguration<'_>,
) -> Result<(), SVError> {
    let schema = configuration.data_source.schema();
    sv_require_role(
        &schema,
        configuration.x,
        &[SVColumnRole::X, SVColumnRole::Y],
        "x",
    )?;
    if !configuration.derived {
        sv_require_role(
            &schema,
            configuration.lower.unwrap_or_default(),
            &[SVColumnRole::Y],
            "lower",
        )?;
        sv_require_role(
            &schema,
            configuration.upper.unwrap_or_default(),
            &[SVColumnRole::Y],
            "upper",
        )?;
    }
    if let Some(series) = configuration.series {
        sv_require_role(
            &schema,
            series,
            &[SVColumnRole::Grouping, SVColumnRole::Y],
            "series",
        )?;
    }
    Ok(())
}

fn sv_require_role(
    schema: &SVSourceSchema<'_>,
    name: &str,
    allowed: &[SVColumnRole],
    role_name: &str,
) -> Result<(), SVError> {
    let Some(descriptor) = schema.column(name) else {
        return Err(SVError::ColumnNotFound {
            name: name.to_string(),
        });
    };
    if allowed.contains(&descriptor.role()) {
        Ok(())
    } else {
        Err(sv_invalid(&format!(
            "the `{role_name}` column `{name}` has the wrong role; expected a numeric or grouping column"
        )))
    }
}

pub(crate) fn sv_read_column(snapshot: &mut SVColumnSnapshot<'_>) -> Vec<f64> {
    snapshot.column_view().dispatch(
        |dense| dense.to_vec(),
        |sparse| {
            (0..sparse.length())
                .map(|index| sparse.value_at(index).unwrap_or(0.0))
                .collect()
        },
        |streaming| streaming.values().to_vec(),
    )
}

pub(crate) fn sv_group_rows(series: &[f64]) -> Vec<(f64, Vec<usize>)> {
    let mut groups: Vec<(f64, Vec<usize>)> = Vec::new();
    for (row, &code) in series.iter().enumerate() {
        match groups
            .iter_mut()
            .find(|(key, _)| key.to_bits() == code.to_bits())
        {
            Some((_, rows)) => rows.push(row),
            None => groups.push((code, vec![row])),
        }
    }
    groups
}

pub(crate) fn sv_owned_length(column: &SVBandColumn<'_>) -> usize {
    match column {
        SVBandColumn::Snapshot(_) => 0,
        SVBandColumn::Owned(values) => values.len(),
    }
}

pub(crate) fn sv_snapshot_length(snapshot: &mut SVColumnSnapshot<'_>) -> usize {
    snapshot.column_view().length()
}

pub(crate) fn sv_widest<const COUNT: usize>(
    representations: [SVRepresentation; COUNT],
) -> SVRepresentation {
    representations
        .into_iter()
        .fold(SVRepresentation::Dense, |widest, candidate| {
            if sv_width(candidate) > sv_width(widest) {
                candidate
            } else {
                widest
            }
        })
}

fn sv_width(representation: SVRepresentation) -> u8 {
    match representation {
        SVRepresentation::Dense => 0,
        SVRepresentation::Sparse => 1,
        SVRepresentation::Streaming => 2,
    }
}

pub(crate) fn sv_series_name(series_code: f64) -> String {
    if series_code.fract() == 0.0 {
        format!("series {}", series_code as i64)
    } else {
        format!("series {series_code}")
    }
}
