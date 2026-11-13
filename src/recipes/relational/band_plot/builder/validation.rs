//! Build-time validation of the mutually exclusive modes.

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVWithBackend;

use super::super::configuration::sv_invalid;
use super::builder_definition::SVBandPlotBuilder;

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
