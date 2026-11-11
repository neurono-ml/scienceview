//! The resolved decoration style: colours, typography and spacing.
//!
//! Decorations never read a token name and never parse a stylesheet. They read
//! this value, which is either the documented built-in default
//! ([`SVDecorationStyle::built_in`]) or the compiled theme tokens
//! ([`SVDecorationStyle::from_theme_tokens`]). The built-in values mirror the
//! default theme exactly, so a build without the `theme` feature produces the
//! same decorations as a themed build.

use crate::sv_scale::SVColour;

/// Every colour, type size and spacing a decoration needs.
///
/// The fields group the way the theme groups them: semantic colour roles,
/// typography, and the spacing that separates an axis band from its plot area.
/// A backend reads this value and emits its own notation; it never invents a
/// literal.
#[derive(Clone, Debug, PartialEq)]
pub struct SVDecorationStyle {
    pub(super) background: SVColour,
    pub(super) surface: SVColour,
    pub(super) text: SVColour,
    pub(super) text_muted: SVColour,
    pub(super) grid: SVColour,
    pub(super) axis: SVColour,
    pub(super) body_font_size: f64,
    pub(super) title_font_size: f64,
    pub(super) label_font_size: f64,
    pub(super) line_height: f64,
    pub(super) regular_weight: u16,
    pub(super) bold_weight: u16,
    pub(super) gap: f64,
    pub(super) padding: f64,
    pub(super) axis_line_width: f64,
    pub(super) grid_line_width: f64,
    pub(super) tick_length: f64,
}

impl SVDecorationStyle {
    /// Returns the figure background colour.
    pub const fn background(&self) -> SVColour {
        self.background
    }

    /// Returns the surface colour of an overlaid decoration.
    pub const fn surface(&self) -> SVColour {
        self.surface
    }

    /// Returns the primary text colour.
    pub const fn text(&self) -> SVColour {
        self.text
    }

    /// Returns the muted text colour used by tick labels.
    pub const fn text_muted(&self) -> SVColour {
        self.text_muted
    }

    /// Returns the grid-line colour.
    pub const fn grid(&self) -> SVColour {
        self.grid
    }

    /// Returns the axis-line colour.
    pub const fn axis(&self) -> SVColour {
        self.axis
    }

    /// Returns the body font size.
    pub const fn body_font_size(&self) -> f64 {
        self.body_font_size
    }

    /// Returns the title font size.
    pub const fn title_font_size(&self) -> f64 {
        self.title_font_size
    }

    /// Returns the axis-label font size.
    pub const fn label_font_size(&self) -> f64 {
        self.label_font_size
    }

    /// Returns the line height as a multiple of the font size.
    pub const fn line_height(&self) -> f64 {
        self.line_height
    }

    /// Returns the regular font weight.
    pub const fn regular_weight(&self) -> u16 {
        self.regular_weight
    }

    /// Returns the bold font weight.
    pub const fn bold_weight(&self) -> u16 {
        self.bold_weight
    }

    /// Returns the gap between two decoration elements.
    pub const fn gap(&self) -> f64 {
        self.gap
    }

    /// Returns the padding inside a decoration band.
    pub const fn padding(&self) -> f64 {
        self.padding
    }

    /// Returns the stroke width of an axis line.
    pub const fn axis_line_width(&self) -> f64 {
        self.axis_line_width
    }

    /// Returns the stroke width of a grid line.
    pub const fn grid_line_width(&self) -> f64 {
        self.grid_line_width
    }

    /// Returns the length of a tick mark.
    pub const fn tick_length(&self) -> f64 {
        self.tick_length
    }
}

impl Default for SVDecorationStyle {
    fn default() -> Self {
        Self::built_in()
    }
}
