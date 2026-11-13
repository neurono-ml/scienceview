//! Reading, grouping and validating the columns the recipe consumes.

use scienceview_core::sv_data::{SVColumnRole, SVColumnView, SVRepresentation, SVSourceSchema};
use scienceview_core::sv_error::SVError;

use super::configuration::{SVBandPlotConfiguration, sv_invalid};
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

/// Returns the logical positions a band reads from `view`.
///
/// The decision is representation-aware: a dense column contributes every
/// position, a sparse column contributes only its stored indices (the implicit
/// zeros are never expanded into rows), and a streaming column contributes its
/// current window. A sparse column of ten million logical values with three
/// stored entries therefore yields three positions, not ten million.
pub(crate) fn sv_positions(view: SVColumnView<'_>) -> Vec<usize> {
    view.dispatch(
        |dense| (0..dense.len()).collect(),
        |sparse| sparse.indices().to_vec(),
        |streaming| (0..streaming.length()).collect(),
    )
}

/// Counts the positions whose three band samples are all finite.
///
/// This is the number of vertices the composition actually draws: a missing
/// sample splits the band and is not a drawn point, so the observation span
/// never overstates the rendered size.
pub(crate) fn sv_count_finite(
    positions: &[usize],
    x: SVColumnView<'_>,
    lower: SVColumnView<'_>,
    upper: SVColumnView<'_>,
) -> usize {
    positions
        .iter()
        .filter(|index| {
            let index = **index;
            x.value_at(index).is_some_and(f64::is_finite)
                && lower.value_at(index).is_some_and(f64::is_finite)
                && upper.value_at(index).is_some_and(f64::is_finite)
        })
        .count()
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
