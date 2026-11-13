//! Build-time validation and column probing for the contour builder.

use super::builder::SVContourPlotBuilder;
use crate::sv_backend::{SVBackendRequirements, sv_verify_backend_requirements};
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;

/// Rejects every impossible option before any column is read.
pub(super) fn sv_validate_options(builder: &SVContourPlotBuilder<'_>) -> Result<(), SVError> {
    if builder.levels == 0 {
        return Err(SVError::InvalidConfiguration {
            message: "a contour plot needs at least one level".to_string(),
        });
    }
    if builder.grid_count < 2 {
        return Err(SVError::InvalidConfiguration {
            message: "a contour grid needs at least two cells per axis".to_string(),
        });
    }
    if let Some((rows, columns)) = builder.matrix_shape {
        if rows < 2 || columns < 2 {
            return Err(SVError::InvalidConfiguration {
                message: "a contour matrix needs at least two rows and two columns".to_string(),
            });
        }
        if builder.x.is_some() || builder.y.is_some() {
            return Err(SVError::InvalidConfiguration {
                message: "a matrix contour reads its axes from the matrix shape, not from `x`/`y`"
                    .to_string(),
            });
        }
    } else if builder.x_range.is_some() || builder.y_range.is_some() {
        return Err(SVError::InvalidConfiguration {
            message:
                "`x_range`/`y_range` describe a matrix layout; also call `matrix(rows, columns)`"
                    .to_string(),
        });
    }
    if let Some((lower, upper)) = builder.level_range {
        if !lower.is_finite() || !upper.is_finite() || lower >= upper {
            return Err(SVError::InvalidConfiguration {
                message: "the level range needs finite bounds with lower < upper".to_string(),
            });
        }
    }
    if let Some(level_size) = builder.level_size {
        if !level_size.is_finite() || level_size <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: "the level size must be a positive finite number".to_string(),
            });
        }
    }
    if !builder.line_width.is_finite() || builder.line_width <= 0.0 {
        return Err(SVError::InvalidConfiguration {
            message: "the contour line width must be a positive finite number".to_string(),
        });
    }
    sv_validate_range(builder.x_range, "x_range")?;
    sv_validate_range(builder.y_range, "y_range")?;
    Ok(())
}

fn sv_validate_range(range: Option<(f64, f64)>, name: &str) -> Result<(), SVError> {
    if let Some((lower, upper)) = range {
        if !lower.is_finite() || !upper.is_finite() || lower >= upper {
            return Err(SVError::InvalidConfiguration {
                message: format!("{name} needs finite bounds with lower < upper"),
            });
        }
    }
    Ok(())
}

/// Probes the named columns once for existence, equal length and the dominant
/// representation, and checks a matrix layout against its shape.
pub(super) fn sv_probe_lengths(
    data_source: &dyn SVDataSource,
    value: &str,
    x: Option<&str>,
    y: Option<&str>,
) -> Result<(usize, SVRepresentation), SVError> {
    let mut snapshot = data_source.column(value)?;
    let view = snapshot.column_view();
    let length = view.length();
    let mut representation = view.representation();
    for name in [x, y].into_iter().flatten() {
        let mut snapshot = data_source.column(name)?;
        let view = snapshot.column_view();
        if view.length() != length {
            return Err(SVError::MismatchedLengths {
                name: name.to_string(),
                expected: length,
                actual: view.length(),
            });
        }
        representation = sv_dominant(representation, view.representation());
    }
    Ok((length, representation))
}

/// Verifies that the backend can host a streaming window, when the bound
/// columns present one.
pub(super) fn sv_verify_streaming(
    backend_kind: crate::sv_backend::SVBackendKind,
    representation: SVRepresentation,
) -> Result<(), SVError> {
    if representation == SVRepresentation::Streaming {
        sv_verify_backend_requirements(
            backend_kind,
            SVBackendRequirements::default().requiring_streaming(),
        )?;
    }
    Ok(())
}

fn sv_dominant(left: SVRepresentation, right: SVRepresentation) -> SVRepresentation {
    use SVRepresentation::{Dense, Sparse, Streaming};
    match (left, right) {
        (Streaming, _) | (_, Streaming) => Streaming,
        (Sparse, _) | (_, Sparse) => Sparse,
        (Dense, Dense) => Dense,
    }
}
