//! Titles, subtitles and captions.

use crate::sv_decoration::{
    SVDecorationPrimitive, SVDecorationStyle, SVPoint, SVTextAnchor, SVTextBox,
};
use crate::sv_layout::{SVLayoutNode, SVLength, SVResolvedRectangle};

/// What a title line is for.
///
/// The role decides the type size, the weight and the colour, so a caller
/// states the intent once and the theme styles it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVTitleRole {
    /// The figure title: the largest, boldest line.
    Title,
    /// The line under the title.
    Subtitle,
    /// The caption under the figure.
    Caption,
}

impl SVTitleRole {
    /// Returns the role name, as written in tests and errors.
    pub const fn as_str(&self) -> &'static str {
        match self {
            SVTitleRole::Title => "Title",
            SVTitleRole::Subtitle => "Subtitle",
            SVTitleRole::Caption => "Caption",
        }
    }
}

/// One title line: a title, a subtitle or a caption.
#[derive(Clone, Debug, PartialEq)]
pub struct SVTitle {
    role: SVTitleRole,
    text: String,
}

impl SVTitle {
    /// Builds a title line.
    pub fn new(role: SVTitleRole, text: impl Into<String>) -> Self {
        Self {
            role,
            text: text.into(),
        }
    }

    /// Returns the role.
    pub fn role(&self) -> SVTitleRole {
        self.role
    }

    /// Returns the text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Measures the line.
    pub fn measure(&self, style: &SVDecorationStyle) -> SVTitleMeasurement {
        let font_size = self.font_size(style);
        let weight = self.weight(style);
        let text_box = SVTextBox::new(self.text.clone(), font_size, style.line_height(), weight);
        let width = text_box.width();
        let height = text_box.height();
        SVTitleMeasurement {
            text_box,
            width,
            height,
        }
    }

    /// Places the line, centred horizontally, inside a solved rectangle.
    pub fn place(
        &self,
        measurement: &SVTitleMeasurement,
        rectangle: SVResolvedRectangle,
        style: &SVDecorationStyle,
    ) -> SVTitlePlacement {
        let position = SVPoint::new(
            rectangle.x + rectangle.width / 2.0,
            rectangle.y + measurement.height,
        );
        let primitive = SVDecorationPrimitive::text(
            position,
            self.text.clone(),
            measurement.text_box.font_size(),
            measurement.text_box.weight(),
            self.colour(style),
            SVTextAnchor::Middle,
        );

        SVTitlePlacement {
            rectangle,
            primitives: vec![primitive],
        }
    }

    /// Builds the layout node the solver measures for this line.
    pub fn layout_node(&self, measurement: &SVTitleMeasurement) -> SVLayoutNode {
        SVLayoutNode::new(format!("title-{}", self.role.as_str().to_lowercase()))
            .with_width(SVLength::fill())
            .with_height(SVLength::absolute(measurement.height))
            .with_intrinsic_size(measurement.width, measurement.height)
    }

    fn font_size(&self, style: &SVDecorationStyle) -> f64 {
        match self.role {
            SVTitleRole::Title => style.title_font_size(),
            SVTitleRole::Subtitle | SVTitleRole::Caption => style.body_font_size(),
        }
    }

    fn weight(&self, style: &SVDecorationStyle) -> u16 {
        match self.role {
            SVTitleRole::Title => style.bold_weight(),
            SVTitleRole::Subtitle | SVTitleRole::Caption => style.regular_weight(),
        }
    }

    fn colour(&self, style: &SVDecorationStyle) -> crate::sv_scale::SVColour {
        match self.role {
            SVTitleRole::Title => style.text(),
            SVTitleRole::Subtitle | SVTitleRole::Caption => style.text_muted(),
        }
    }
}

/// What one title line measured.
#[derive(Clone, Debug, PartialEq)]
pub struct SVTitleMeasurement {
    text_box: SVTextBox,
    width: f64,
    height: f64,
}

impl SVTitleMeasurement {
    /// Returns the measured text box.
    pub fn text_box(&self) -> &SVTextBox {
        &self.text_box
    }

    /// Returns the intrinsic width.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Returns the intrinsic height.
    pub fn height(&self) -> f64 {
        self.height
    }
}

/// The placed title line.
#[derive(Clone, Debug, PartialEq)]
pub struct SVTitlePlacement {
    rectangle: SVResolvedRectangle,
    primitives: Vec<SVDecorationPrimitive>,
}

impl SVTitlePlacement {
    /// Returns the rectangle the line occupies.
    pub fn rectangle(&self) -> SVResolvedRectangle {
        self.rectangle
    }

    /// Returns the backend-agnostic primitives that draw the line.
    pub fn primitives(&self) -> &[SVDecorationPrimitive] {
        &self.primitives
    }
}
