//! Building one continuous scale per visible variable.

use super::super::configuration::{
    SV_PARALLEL_COORDINATES_PLOT_HEIGHT, SV_PARALLEL_COORDINATES_PLOT_LEFT,
    SV_PARALLEL_COORDINATES_PLOT_WIDTH, SVParallelCoordinatesConfiguration,
};
use super::super::model::SVParallelCoordinatesAxis;
use super::super::options::SVParallelCoordinatesNormalization;
use super::SVResolvedColumns;
use super::SVVisibleAxis;
use crate::sv_data::SVColumnSnapshot;
use crate::sv_error::SVError;
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind, SVScaleValue};

pub(super) fn sv_build_axes(
    configuration: &SVParallelCoordinatesConfiguration<'_>,
    columns: &mut SVResolvedColumns<'_>,
    visible: &[SVVisibleAxis<'_>],
) -> Result<Vec<SVParallelCoordinatesAxis>, SVError> {
    let labels: Vec<&str> = visible.iter().map(|axis| axis.variable).collect();
    let horizontal = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&labels)
        .range(0.0, SV_PARALLEL_COORDINATES_PLOT_WIDTH)
        .build()?;

    let derived = visible
        .iter()
        .map(|axis| sv_derive_domain(&mut columns.variables[axis.variable_index], columns.length))
        .collect::<Vec<_>>();
    let shared = sv_shared_domain(&derived);

    let mut axes = Vec::with_capacity(visible.len());
    for (index, axis) in visible.iter().enumerate() {
        let position = SV_PARALLEL_COORDINATES_PLOT_LEFT
            + horizontal.position(SVScaleValue::Category(axis.variable))?;
        let domain = sv_axis_domain(configuration, axis, derived[index], shared);
        let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(domain.0, domain.1)
            .range(SV_PARALLEL_COORDINATES_PLOT_HEIGHT, 0.0)
            .build()?;
        axes.push(SVParallelCoordinatesAxis::new(
            axis.variable.to_string(),
            position,
            scale,
            axis.coloured,
        ));
    }
    Ok(axes)
}

fn sv_axis_domain(
    configuration: &SVParallelCoordinatesConfiguration<'_>,
    axis: &SVVisibleAxis<'_>,
    derived: Option<(f64, f64)>,
    shared: Option<(f64, f64)>,
) -> (f64, f64) {
    let domain = axis.range_override.or(match configuration.normalization {
        SVParallelCoordinatesNormalization::PerAxis => derived,
        SVParallelCoordinatesNormalization::Shared => shared,
    });
    let (lower, upper) = domain.unwrap_or((0.0, 1.0));
    sv_padded_domain(lower, upper)
}

fn sv_shared_domain(derived: &[Option<(f64, f64)>]) -> Option<(f64, f64)> {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for (axis_lower, axis_upper) in derived.iter().flatten() {
        lower = lower.min(*axis_lower);
        upper = upper.max(*axis_upper);
    }
    (lower.is_finite() && upper.is_finite()).then_some((lower, upper))
}

fn sv_derive_domain(snapshot: &mut SVColumnSnapshot<'_>, length: usize) -> Option<(f64, f64)> {
    let view = snapshot.column_view();
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for index in 0..length {
        if let Some(value) = view.value_at(index).filter(|value| value.is_finite()) {
            lower = lower.min(value);
            upper = upper.max(value);
        }
    }
    (lower.is_finite() && upper.is_finite()).then_some((lower, upper))
}

fn sv_padded_domain(lower: f64, upper: f64) -> (f64, f64) {
    if (upper - lower).abs() > f64::EPSILON {
        return (lower, upper);
    }
    let padding = if lower.abs() <= f64::EPSILON {
        1.0
    } else {
        lower.abs() * 0.1
    };
    (lower - padding, upper + padding)
}

/// Builds the identity scale a mark uses to place already-scaled pixels.
pub(crate) fn sv_identity_scale(start: f64, end: f64) -> Result<SVScale<'static>, SVError> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(start, end)
        .range(start, end)
        .build()
}
