//! Build-time validation and column probing for the density-contour builder.

use super::builder::SVDensityContourPlotBuilder;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;

/// Rejects every impossible option before any column is read.
pub(super) fn sv_validate_options(
    builder: &SVDensityContourPlotBuilder<'_>,
) -> Result<(), SVError> {
    if builder.levels == 0 {
        return Err(SVError::InvalidConfiguration {
            message: "a density contour needs at least one level".to_string(),
        });
    }
    if builder.grid_count < 2 {
        return Err(SVError::InvalidConfiguration {
            message: "a density grid needs at least two cells per axis".to_string(),
        });
    }
    if let Some(bandwidth) = builder.bandwidth {
        if !bandwidth.is_finite() || bandwidth <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: "the density bandwidth must be a positive finite number".to_string(),
            });
        }
    }
    if !builder.bw_adjust.is_finite() || builder.bw_adjust <= 0.0 {
        return Err(SVError::InvalidConfiguration {
            message: "the bandwidth adjustment must be a positive finite number".to_string(),
        });
    }
    if let Some((lower, upper)) = builder.level_range {
        if !lower.is_finite() || !upper.is_finite() || lower >= upper {
            return Err(SVError::InvalidConfiguration {
                message: "the level range needs finite bounds with lower < upper".to_string(),
            });
        }
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
