//! The schema check and the small helpers shared by the data contract.

use scienceview_core::sv_data::{SVColumnRole, SVColumnView, SVSourceSchema};
use scienceview_core::sv_error::SVError;

use super::super::configuration::{SVViolinPlotConfiguration, sv_invalid};

/// Validates that the configured columns exist with the documented roles.
pub(crate) fn sv_validate_columns(
    configuration: &SVViolinPlotConfiguration<'_>,
) -> Result<(), SVError> {
    let schema = configuration.data_source.schema();
    sv_require_role(
        &schema,
        configuration.value,
        &[SVColumnRole::Y, SVColumnRole::X],
        "value",
    )?;
    sv_require_role(
        &schema,
        configuration.category,
        &[SVColumnRole::Grouping, SVColumnRole::X, SVColumnRole::Label],
        "category",
    )?;
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
            "the `{role_name}` column `{name}` has the wrong role for a violin plot"
        )))
    }
}

pub(crate) fn sv_positions(view: SVColumnView<'_>) -> Vec<usize> {
    view.dispatch(
        |dense| (0..dense.len()).collect(),
        |sparse| sparse.indices().to_vec(),
        |streaming| (0..streaming.length()).collect(),
    )
}

/// Formats a category code as the label the axis shows.
pub(crate) fn sv_category_label(code: f64) -> String {
    if code.fract() == 0.0 {
        format!("category {}", code as i64)
    } else {
        format!("category {code}")
    }
}

/// Formats a series code as the label the legend shows.
pub(crate) fn sv_series_label(code: f64) -> String {
    if code.fract() == 0.0 {
        format!("series {}", code as i64)
    } else {
        format!("series {code}")
    }
}
