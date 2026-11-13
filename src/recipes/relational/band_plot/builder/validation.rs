//! Build-time validation of the mutually exclusive modes.

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVWithBackend;
use scienceview_core::sv_scale::SVScaleKind;

use super::super::configuration::sv_invalid;
use super::builder_definition::SVBandPlotBuilder;

/// Rejects an `x` scale that cannot order a band.
///
/// A band is a sequence along a continuous or temporal axis; a nominal or
/// boolean axis has no between-ness to fill. The check runs at build time so
/// the caller learns the configuration is impossible before any data is read
/// (patterns P7).
pub(crate) fn sv_validate_x_scale_kind(x_scale_kind: SVScaleKind) -> Result<(), SVError> {
    match x_scale_kind {
        SVScaleKind::Continuous | SVScaleKind::Temporal => Ok(()),
        SVScaleKind::Nominal | SVScaleKind::Boolean => Err(sv_invalid(&format!(
            "the `x_scale_kind` `{x_scale_kind}` cannot order a band; select `Continuous` or `Temporal`"
        ))),
    }
}

pub(crate) fn sv_validate_mode(
    builder: &SVBandPlotBuilder<'_, SVWithBackend>,
) -> Result<(), SVError> {
    if builder.derived {
        if builder.values.is_none() {
            return Err(sv_invalid("the derived mode needs the `values` column"));
        }
        if builder.lower.is_some() || builder.upper.is_some() {
            return Err(sv_invalid(
                "the derived mode cannot also name explicit `lower` and `upper` columns",
            ));
        }
        Ok(())
    } else {
        if builder.lower.is_none() || builder.upper.is_none() {
            return Err(sv_invalid(
                "the explicit mode needs both the `lower` and `upper` columns",
            ));
        }
        if builder.values.is_some() {
            return Err(sv_invalid(
                "the explicit mode cannot also name the `values` column",
            ));
        }
        Ok(())
    }
}
