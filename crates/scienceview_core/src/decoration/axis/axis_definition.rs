//! An axis: the ticks that fit one side of the plot area.

use super::builder::SVAxisBuilder;
use super::labels::{sv_measure_axis_ticks, sv_widest_label};
use super::measurement::SVAxisMeasurement;
use super::placement::SVAxisPlacement;
use super::side::SVAxisSide;
use super::space::SVAxisSpace;
use crate::sv_decoration::{SVDecorationStyle, SVTextBox};
use crate::sv_error::SVError;
use crate::sv_layout::{SVLayoutNode, SVLength, SVResolvedRectangle};
use crate::sv_scale::{SVScale, SVTickDerivation, SVTickFormat, SVTickSpace};

/// The multiplier that turns a label font size into a minimum tick spacing.
///
/// A horizontal tick needs room for its label and the gap beside it; four
/// em-widths keep adjacent labels apart before the exact collision pass runs.
/// A vertical tick only needs its line height plus a little air.
const HORIZONTAL_SPACING_IN_EM: f64 = 4.0;
const VERTICAL_SPACING_IN_LINE_HEIGHTS: f64 = 1.5;

/// One axis of a figure.
///
/// The axis owns its presentation, never the mathematics: it asks the scale for
/// the ticks that fit the granted space and formats their labels. The direct
/// constructor is private; [`SVAxis::builder`] is the only public way to build
/// one.
#[derive(Clone, Debug, PartialEq)]
pub struct SVAxis {
    side: SVAxisSide,
    title: Option<String>,
    format: SVTickFormat,
    minimum_tick_spacing: Option<f64>,
}

impl SVAxis {
    /// Starts building an axis on one side.
    pub fn builder(side: SVAxisSide) -> SVAxisBuilder {
        SVAxisBuilder {
            side,
            title: None,
            format: SVTickFormat::Automatic,
            minimum_tick_spacing: None,
        }
    }

    pub(crate) fn from_builder(builder: SVAxisBuilder) -> Self {
        Self {
            side: builder.side,
            title: builder.title,
            format: builder.format,
            minimum_tick_spacing: builder.minimum_tick_spacing,
        }
    }

    /// Returns the side the axis occupies.
    pub fn side(&self) -> SVAxisSide {
        self.side
    }

    /// Returns the axis title, when it has one.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the label format.
    pub fn tick_format(&self) -> SVTickFormat {
        self.format
    }

    /// Returns the configured minimum tick spacing, when the caller set one.
    pub fn minimum_tick_spacing(&self) -> Option<f64> {
        self.minimum_tick_spacing
    }

    /// Measures the axis against the plot extent the layout engine granted.
    ///
    /// The count comes from [`SVTickSpace`], which the scale's
    /// [`SVTickDerivation`] turns into ticks; the collision pass then drops the
    /// labels that would touch their predecessor. A narrow plot therefore draws
    /// fewer, readable ticks.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the granted extent or the
    /// resolved spacing is not a positive finite number.
    pub fn measure(
        &self,
        scale: &SVScale<'_>,
        space: SVAxisSpace,
        style: &SVDecorationStyle,
    ) -> Result<SVAxisMeasurement, SVError> {
        let tick_space = SVTickSpace::new(
            space.plot_extent(),
            self.resolved_minimum_tick_spacing(style),
        )?;
        let ticks = self.derive_ticks(scale, tick_space);
        let horizontal = self.side.is_horizontal();
        let measured = sv_measure_axis_ticks(ticks, horizontal, style, style.gap() / 2.0);
        let title_box = self.measure_title(style);
        let band_thickness = self.band_thickness(&measured, title_box.as_ref(), style, horizontal);

        Ok(SVAxisMeasurement::new(
            self.side,
            measured,
            band_thickness,
            title_box,
        ))
    }

    /// Places the axis beside a solved plot rectangle.
    ///
    /// The tick positions are the scale's display coordinates, measured from
    /// the plot origin along the axis; a reversed scale range therefore reverses
    /// the axis without changing this code.
    pub fn place(
        &self,
        measurement: &SVAxisMeasurement,
        plot: SVResolvedRectangle,
        style: &SVDecorationStyle,
    ) -> SVAxisPlacement {
        SVAxisPlacement::sv_place(self.side, measurement, plot, style)
    }

    /// Builds the layout node the solver measures for this axis.
    ///
    /// A vertical axis declares its band thickness as width and fills the
    /// height; a horizontal axis declares its band thickness as height and
    /// fills the width. The node carries the measurement as its intrinsic size,
    /// so the solver never re-measures the decoration.
    pub fn layout_node(&self, measurement: &SVAxisMeasurement) -> SVLayoutNode {
        let node = SVLayoutNode::new(format!("axis-{}", self.side.as_str().to_lowercase()));
        if self.side.is_horizontal() {
            node.with_width(SVLength::fill())
                .with_height(SVLength::absolute(measurement.band_thickness()))
                .with_intrinsic_size(0.0, measurement.band_thickness())
        } else {
            node.with_width(SVLength::absolute(measurement.band_thickness()))
                .with_height(SVLength::fill())
                .with_intrinsic_size(measurement.band_thickness(), 0.0)
        }
    }

    fn derive_ticks(
        &self,
        scale: &SVScale<'_>,
        tick_space: SVTickSpace,
    ) -> Vec<crate::sv_scale::SVTick> {
        match self.format {
            SVTickFormat::Automatic => SVTickDerivation::ticks(scale, tick_space),
            format => scale.ticks_with_format(tick_space, format),
        }
    }

    fn resolved_minimum_tick_spacing(&self, style: &SVDecorationStyle) -> f64 {
        let configured = self.minimum_tick_spacing.unwrap_or(f64::NAN);
        if configured.is_finite() && configured > 0.0 {
            return configured;
        }

        if self.side.is_horizontal() {
            style.label_font_size() * HORIZONTAL_SPACING_IN_EM
        } else {
            style.label_font_size() * style.line_height() * VERTICAL_SPACING_IN_LINE_HEIGHTS
        }
    }

    fn measure_title(&self, style: &SVDecorationStyle) -> Option<SVTextBox> {
        self.title.as_ref().map(|title| {
            SVTextBox::new(
                title.clone(),
                style.label_font_size(),
                style.line_height(),
                style.regular_weight(),
            )
        })
    }

    fn band_thickness(
        &self,
        measured: &[super::measurement::SVAxisTickMeasurement],
        title_box: Option<&SVTextBox>,
        style: &SVDecorationStyle,
        horizontal: bool,
    ) -> f64 {
        let label_extent = if horizontal {
            style.label_font_size() * style.line_height()
        } else {
            sv_widest_label(measured)
        };
        let mut thickness = style.tick_length() + style.gap() + label_extent;
        if let Some(title_box) = title_box {
            thickness += style.gap() + title_box.height();
        }

        thickness.max(style.tick_length())
    }
}
