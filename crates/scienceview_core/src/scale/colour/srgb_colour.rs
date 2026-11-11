//! A backend-independent colour value used by every colour scale.
//!
//! The scale path carries its own colour type instead of importing the theme's,
//! so the scale model compiles without the `theme` feature; the theme bridge
//! converts a theme colour into this one when a token is resolved.

/// One straight (non-premultiplied) colour with four 8-bit channels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SVSrgbColour {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl SVSrgbColour {
    /// Builds a colour from its four channels.
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Returns the red channel.
    pub const fn red(&self) -> u8 {
        self.red
    }

    /// Returns the green channel.
    pub const fn green(&self) -> u8 {
        self.green
    }

    /// Returns the blue channel.
    pub const fn blue(&self) -> u8 {
        self.blue
    }

    /// Returns the alpha channel.
    pub const fn alpha(&self) -> u8 {
        self.alpha
    }

    /// Returns the four channels in draw order.
    pub const fn components(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }

    /// Returns the colour with a different alpha channel.
    pub const fn with_alpha(self, alpha: u8) -> Self {
        Self { alpha, ..self }
    }

    /// Interpolates between two colours in the authored (gamma-encoded) space.
    ///
    /// A non-finite fraction resolves to the start colour, so a missing value
    /// never poisons a render.
    pub fn interpolate(self, other: Self, fraction: f64) -> Self {
        let fraction = if fraction.is_finite() {
            fraction.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let blend = |start: u8, end: u8| -> u8 {
            let difference = f64::from(end) - f64::from(start);
            (f64::from(start) + difference * fraction).round() as u8
        };
        Self::new(
            blend(self.red, other.red),
            blend(self.green, other.green),
            blend(self.blue, other.blue),
            blend(self.alpha, other.alpha),
        )
    }

    /// Serialises as `#rrggbb`, appending the alpha pair only when it is not
    /// fully opaque.
    pub fn to_hex_string(&self) -> String {
        if self.alpha == u8::MAX {
            format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
        } else {
            format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                self.red, self.green, self.blue, self.alpha
            )
        }
    }
}
