//! Build-time validation of the columns and the backend mark matrix.

use super::configuration::SVRegressionConfiguration;
use scienceview_core::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::SVMarkKind;

/// Checks that every bound column is declared by the source.
pub(crate) fn sv_validate_columns(
    source: &dyn SVDataSource,
    configuration: &SVRegressionConfiguration,
) -> Result<(), SVError> {
    let schema = source.schema();
    let bound = [
        configuration.x.as_deref(),
        configuration.y.as_deref(),
        configuration.series.as_deref(),
    ];
    for name in bound.into_iter().flatten() {
        if schema.column(name).is_none() {
            return Err(SVError::ColumnNotFound {
                name: name.to_string(),
            });
        }
    }
    Ok(())
}

/// Checks the selected backend can draw every composed mark.
pub(crate) fn sv_validate_marks(
    backend_kind: SVBackendKind,
    configuration: &SVRegressionConfiguration,
) -> Result<(), SVError> {
    if configuration.show_points {
        sv_verify_mark_supported(SVMarkKind::Dot, backend_kind)?;
    }
    if configuration.show_fit {
        sv_verify_mark_supported(SVMarkKind::Line, backend_kind)?;
    }
    if configuration.confidence.is_some() {
        sv_verify_mark_supported(SVMarkKind::Ribbon, backend_kind)?;
    }
    Ok(())
}
