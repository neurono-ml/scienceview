//! Cross-reference and placement validation of a dashboard composition.

use super::linked_axis::SVLinkedAxisRegistration;
use crate::sv_error::SVError;
use crate::sv_panel::SVDashboardPanel;
use crate::sv_source::SVSharedSource;

pub(super) fn sv_validate_unique_sources(sources: &[SVSharedSource<'_>]) -> Result<(), SVError> {
    for (position, source) in sources.iter().enumerate() {
        if sources[..position]
            .iter()
            .any(|earlier| earlier.identifier() == source.identifier())
        {
            return Err(SVError::DuplicateDashboardSource {
                source_identifier: source.identifier().to_owned(),
            });
        }
    }
    Ok(())
}

pub(super) fn sv_validate_unique_panels(panels: &[SVDashboardPanel<'_>]) -> Result<(), SVError> {
    for (position, panel) in panels.iter().enumerate() {
        if panels[..position]
            .iter()
            .any(|earlier| earlier.identifier() == panel.identifier())
        {
            return Err(SVError::DuplicateDashboardPanel {
                panel: panel.identifier().to_owned(),
            });
        }
    }
    Ok(())
}

pub(super) fn sv_validate_linked_axes(axes: &[SVLinkedAxisRegistration]) -> Result<(), SVError> {
    for (position, axis) in axes.iter().enumerate() {
        if axes[..position]
            .iter()
            .any(|earlier| earlier.identifier == axis.identifier)
        {
            return Err(SVError::DuplicateDashboardAxis {
                axis: axis.identifier.clone(),
            });
        }
    }
    Ok(())
}

pub(super) fn sv_validate_panel_references(
    panel: &SVDashboardPanel<'_>,
    sources: &[SVSharedSource<'_>],
    axes: &[SVLinkedAxisRegistration],
) -> Result<(), SVError> {
    if !sources
        .iter()
        .any(|source| source.identifier() == panel.source_identifier())
    {
        return Err(SVError::UnknownDashboardSource {
            source_identifier: panel.source_identifier().to_owned(),
        });
    }
    for binding in panel.axes() {
        let axis = axes
            .iter()
            .find(|registered| registered.identifier == binding.identifier())
            .ok_or_else(|| SVError::UnknownDashboardAxis {
                axis: binding.identifier().to_owned(),
            })?;
        if axis.axis != binding.axis() {
            return Err(SVError::DashboardAxisMismatch {
                axis: binding.identifier().to_owned(),
                expected: axis.axis,
                found: binding.axis(),
            });
        }
    }
    Ok(())
}

pub(super) fn sv_validate_no_overlap(panels: &[SVDashboardPanel<'_>]) -> Result<(), SVError> {
    for (position, panel) in panels.iter().enumerate() {
        for earlier in &panels[..position] {
            if earlier.placement().overlaps(&panel.placement()) {
                return Err(SVError::OverlappingDashboardPanels {
                    first: earlier.identifier().to_owned(),
                    second: panel.identifier().to_owned(),
                });
            }
        }
    }
    Ok(())
}
