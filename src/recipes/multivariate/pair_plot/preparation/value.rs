//! One logical read from a column view and the missing-value policy.

use super::super::SVPairMissingValuePolicy;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;

pub(crate) fn sv_value_at(view: SVColumnView<'_>, index: usize) -> Result<f64, SVError> {
    view.value_at(index)
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: format!("a pair-grid column ended before observation {index}"),
        })
}

pub(crate) fn sv_report_missing(
    policy: SVPairMissingValuePolicy,
    _column: &str,
    index: usize,
) -> Result<(), SVError> {
    match policy {
        SVPairMissingValuePolicy::Omit => Ok(()),
        SVPairMissingValuePolicy::Error => Err(SVError::NonFiniteValue {
            operation: "pair_plot",
            position: index,
        }),
    }
}
