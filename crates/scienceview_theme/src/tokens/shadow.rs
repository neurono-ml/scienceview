//! Shadow tokens: an offset, a blur radius and a colour.

use crate::tokens::SVColour;

/// One resolved shadow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SVShadow {
    offset_x: f64,
    offset_y: f64,
    blur_radius: f64,
    colour: SVColour,
}

impl SVShadow {
    pub const fn new(offset_x: f64, offset_y: f64, blur_radius: f64, colour: SVColour) -> Self {
        Self {
            offset_x,
            offset_y,
            blur_radius,
            colour,
        }
    }

    pub const fn offset_x(&self) -> f64 {
        self.offset_x
    }

    pub const fn offset_y(&self) -> f64 {
        self.offset_y
    }

    pub const fn blur_radius(&self) -> f64 {
        self.blur_radius
    }

    pub const fn colour(&self) -> SVColour {
        self.colour
    }
}

/// The shadow half of a compiled theme.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SVShadowTokens {
    pub(crate) none: SVShadow,
    pub(crate) overlay: SVShadow,
}

impl SVShadowTokens {
    /// The absence of a shadow, tokenised so a backend never writes a literal.
    pub fn none(&self) -> SVShadow {
        self.none
    }

    /// The shadow of floating elements such as tooltips and popovers.
    pub fn overlay(&self) -> SVShadow {
        self.overlay
    }
}
