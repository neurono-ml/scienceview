//! The `SVImagePlot` builder and its validating state machine.

use super::configuration::SVImageConfiguration;
use super::core_implementation::SVImagePlot;
use super::{
    SVImageAspectRatio, SVImageExtent, SVImageInterpolation, SVImageMissingPolicy, SVImageOrigin,
    SVImageSource,
};
use crate::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::{SVMark, SVMarkKind};
use crate::sv_scale::SVColourMap;
use std::marker::PhantomData;

/// Builds a [`SVImagePlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVImagePlotBuilder::build), so an image plot that exists can
/// render.
#[derive(Clone)]
pub struct SVImagePlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) source: Option<SVImageSource<'data>>,
    pub(super) colour_map: Option<SVColourMap>,
    pub(super) interpolation: SVImageInterpolation,
    pub(super) minimum_intensity: Option<f64>,
    pub(super) maximum_intensity: Option<f64>,
    pub(super) opacity: f32,
    pub(super) origin: SVImageOrigin,
    pub(super) aspect_ratio: SVImageAspectRatio,
    pub(super) missing_policy: SVImageMissingPolicy,
    pub(super) colour_bar: bool,
    pub(super) extent: Option<SVImageExtent>,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) states: PhantomData<&'data ()>,
}

impl<'data> SVImagePlotBuilder<'data> {
    /// Starts an image plot with no source and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            source: None,
            colour_map: None,
            interpolation: SVImageInterpolation::Nearest,
            minimum_intensity: None,
            maximum_intensity: None,
            opacity: 1.0,
            origin: SVImageOrigin::TopLeft,
            aspect_ratio: SVImageAspectRatio::Equal,
            missing_policy: SVImageMissingPolicy::Transparent,
            colour_bar: false,
            extent: None,
            title: None,
            x_label: None,
            y_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
            states: PhantomData,
        }
    }

    /// Validates the configuration and builds the image plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`], [`SVError::MissingBackendSelection`]
    /// or a configuration error naming the missing raster, the impossible
    /// option or the column whose length does not fill the raster,
    /// [`SVError::ColumnNotFound`] for a column the source does not expose and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw a raster.
    pub fn build(self) -> Result<SVImagePlot<'data>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let source = self.source.ok_or_else(|| SVError::InvalidConfiguration {
            message: "SVImagePlot requires a raster; pass one with `raster(...)`".to_string(),
        })?;
        let dimensions = source.dimensions();

        sv_verify_mark_supported(SVMarkKind::Image, backend_kind)?;
        let mark: SVMark = SVMark::image()
            .interpolation(self.interpolation)
            .opacity(self.opacity)
            .into();
        mark.validate()?;
        sv_validate_intensity_bounds(self.minimum_intensity, self.maximum_intensity)?;

        if self.colour_bar && !source.is_intensity() {
            return Err(SVError::InvalidConfiguration {
                message: "a colour bar describes a continuous colour map, so it needs an intensity raster, not a colour raster"
                    .to_string(),
            });
        }

        let (number_of_pixels, representation) = sv_probe_source(data_source, &source, dimensions)?;

        Ok(SVImagePlot {
            configuration: SVImageConfiguration {
                data_source,
                source,
                colour_map: self.colour_map,
                interpolation: self.interpolation,
                minimum_intensity: self.minimum_intensity,
                maximum_intensity: self.maximum_intensity,
                opacity: self.opacity,
                origin: self.origin,
                aspect_ratio: self.aspect_ratio,
                missing_policy: self.missing_policy,
                colour_bar: self.colour_bar,
                extent: self.extent,
                title: self.title,
                x_label: self.x_label,
                y_label: self.y_label,
                backend_kind,
                execution_mode: self.execution_mode,
                representation,
                number_of_pixels,
            },
        })
    }
}

impl Default for SVImagePlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_validate_intensity_bounds(minimum: Option<f64>, maximum: Option<f64>) -> Result<(), SVError> {
    for (name, bound) in [
        ("minimum_intensity", minimum),
        ("maximum_intensity", maximum),
    ] {
        if let Some(bound) = bound {
            if !bound.is_finite() {
                return Err(SVError::InvalidConfiguration {
                    message: format!("{name} needs a finite value, received {bound}"),
                });
            }
        }
    }
    if let (Some(minimum), Some(maximum)) = (minimum, maximum) {
        if minimum > maximum {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "minimum_intensity must not exceed maximum_intensity, received {minimum} and {maximum}"
                ),
            });
        }
    }
    Ok(())
}

fn sv_probe_source(
    data_source: &dyn SVDataSource,
    source: &SVImageSource<'_>,
    dimensions: super::SVRasterDimensions,
) -> Result<(usize, SVRepresentation), SVError> {
    let expected = dimensions.number_of_pixels();
    match source {
        SVImageSource::Colour { pixels, .. } => {
            if pixels.len() != dimensions.number_of_colour_bytes() {
                return Err(SVError::InvalidConfiguration {
                    message: format!(
                        "a {} by {} colour raster needs {} bytes, received {}",
                        dimensions.rows(),
                        dimensions.columns(),
                        dimensions.number_of_colour_bytes(),
                        pixels.len()
                    ),
                });
            }
            Ok((expected, SVRepresentation::Dense))
        }
        SVImageSource::Intensity { column, .. } => {
            let mut snapshot = data_source.column(column)?;
            let representation = snapshot.representation();
            let length = snapshot.column_view().length();
            if representation == SVRepresentation::Streaming {
                return Ok((expected, representation));
            }
            if length != expected {
                return Err(SVError::MismatchedLengths {
                    name: format!(
                        "{} (a {} by {} raster)",
                        column,
                        dimensions.rows(),
                        dimensions.columns()
                    ),
                    expected,
                    actual: length,
                });
            }
            Ok((expected, representation))
        }
    }
}
