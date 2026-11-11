//! Mapping data columns onto display positions through the scale kernel.

use super::accessor::SVColumnAccessor;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_mark::{SVMarkRequest, SVPoint};
use crate::sv_scale::{SVScale, SVScaleValue};

/// Places one numeric value on a scale.
///
/// A mark places numeric values only: a nominal axis is drawn by placing the
/// category's slot code through a numeric slot scale, so a mark handed a
/// nominal scale reports the scale's own configuration error instead of
/// inventing a category.
pub(crate) fn sv_scale_position(scale: &SVScale<'_>, value: f64) -> Result<f64, SVError> {
    if !value.is_finite() {
        return Err(SVError::InvalidConfiguration {
            message: format!("a mark cannot place the non-finite value {value}"),
        });
    }

    scale.position(SVScaleValue::Numeric(value))
}

/// Rejects columns of different lengths.
pub(crate) fn sv_require_same_length(
    expected: usize,
    actual: usize,
    column_name: &str,
) -> Result<(), SVError> {
    if expected == actual {
        return Ok(());
    }

    Err(SVError::MismatchedLengths {
        name: column_name.to_string(),
        expected,
        actual,
    })
}

/// Maps both channels onto display space, dispatching on each representation
/// exactly once.
pub(crate) fn sv_plot_positions(request: &SVMarkRequest<'_, '_>) -> Result<Vec<SVPoint>, SVError> {
    let horizontal = SVColumnAccessor::resolve(request.primary());
    let vertical = SVColumnAccessor::resolve(request.secondary());
    sv_require_same_length(
        horizontal.length(),
        vertical.length(),
        "mark vertical values",
    )?;

    (0..horizontal.length())
        .map(|index| {
            let x = sv_scale_position(request.horizontal_scale(), horizontal.value_at(index))?;
            let y = sv_scale_position(request.vertical_scale(), vertical.value_at(index))?;
            Ok(SVPoint::new(x, y))
        })
        .collect()
}

/// Maps one boundary channel onto display values through the vertical scale.
pub(crate) fn sv_boundary_positions<'data>(
    request: &SVMarkRequest<'data, '_>,
    view: SVColumnView<'data>,
    column_name: &str,
) -> Result<Vec<f64>, SVError> {
    let accessor = SVColumnAccessor::resolve(view);
    sv_require_same_length(accessor.length(), request.primary().length(), column_name)?;

    (0..accessor.length())
        .map(|index| sv_scale_position(request.vertical_scale(), accessor.value_at(index)))
        .collect()
}

/// Maps one boundary channel onto display values through the horizontal scale.
pub(crate) fn sv_horizontal_boundary_positions<'data>(
    request: &SVMarkRequest<'data, '_>,
    view: SVColumnView<'data>,
    column_name: &str,
) -> Result<Vec<f64>, SVError> {
    let accessor = SVColumnAccessor::resolve(view);
    sv_require_same_length(accessor.length(), request.primary().length(), column_name)?;

    (0..accessor.length())
        .map(|index| sv_scale_position(request.horizontal_scale(), accessor.value_at(index)))
        .collect()
}
