//! Small helpers shared by the composition pass.

use crate::sv_data::{SVColumnSnapshot, SVColumnView, SVDataSource};
use crate::sv_error::SVError;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

pub(super) fn sv_optional_snapshot<'source>(
    data_source: &'source dyn SVDataSource,
    name: &Option<String>,
) -> Result<Option<SVColumnSnapshot<'source>>, SVError> {
    match name {
        Some(name) => Ok(Some(data_source.column(name)?)),
        None => Ok(None),
    }
}

pub(super) fn sv_check_length(
    reference_name: &str,
    expected: usize,
    name: &str,
    view: SVColumnView<'_>,
) -> Result<(), SVError> {
    if view.length() == expected {
        return Ok(());
    }
    Err(SVError::MismatchedLengths {
        name: format!("{name} (against {reference_name})"),
        expected,
        actual: view.length(),
    })
}

pub(super) fn sv_position_scale(
    view: SVColumnView<'_>,
    start: f64,
    end: f64,
) -> Result<SVScale<'_>, SVError> {
    if view.length() == 0 {
        return SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(0.0, 1.0)
            .range(start, end)
            .build();
    }
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .data_view(view)
        .range(start, end)
        .build()
}

pub(super) fn sv_resolved_edge_width(edge_width: SVLength, plot: SVResolvedRectangle) -> f64 {
    let width = edge_width.resolve_width(plot.width, plot.width, plot.height);
    if width.is_finite() && width > 0.0 {
        width
    } else {
        0.0
    }
}
