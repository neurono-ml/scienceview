//! Build-time validation and column probing for the density-heatmap builder.

use super::builder::SVDensityHeatmapPlotBuilder;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;

/// Rejects every impossible option before any column is read.
pub(super) fn sv_validate_options(
    builder: &SVDensityHeatmapPlotBuilder<'_>,
) -> Result<(), SVError> {
    if builder.bin_count_x == Some(0) {
        return Err(SVError::InvalidConfiguration {
            message: "a density heatmap needs at least one horizontal bin".to_string(),
        });
    }
    if builder.bin_count_y == Some(0) {
        return Err(SVError::InvalidConfiguration {
            message: "a density heatmap needs at least one vertical bin".to_string(),
        });
    }
    if !builder.opacity.is_finite() || !(0.0..=1.0).contains(&builder.opacity) {
        return Err(SVError::InvalidConfiguration {
            message: "the cell opacity must lie in 0.0..=1.0".to_string(),
        });
    }
    Ok(())
}

/// Probes the named columns once for existence and equal length.
pub(super) fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    names: &[&str],
) -> Result<(usize, SVRepresentation), SVError> {
    let mut expected: Option<(usize, SVRepresentation)> = None;
    for name in names {
        let mut snapshot = data_source.column(name)?;
        let view = snapshot.column_view();
        match expected {
            None => expected = Some((view.length(), view.representation())),
            Some((length, _)) if view.length() != length => {
                return Err(SVError::MismatchedLengths {
                    name: (*name).to_string(),
                    expected: length,
                    actual: view.length(),
                });
            }
            Some((_, seen)) => {
                expected = Some((view.length(), sv_dominant(seen, view.representation())));
            }
        }
    }
    Ok(expected.unwrap_or((0, SVRepresentation::Dense)))
}

fn sv_dominant(left: SVRepresentation, right: SVRepresentation) -> SVRepresentation {
    use SVRepresentation::{Dense, Sparse, Streaming};
    match (left, right) {
        (Streaming, _) | (_, Streaming) => Streaming,
        (Sparse, _) | (_, Sparse) => Sparse,
        (Dense, Dense) => Dense,
    }
}
