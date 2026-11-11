//! A backend-independent colour value: four 8-bit channels.
//!
//! Every backend converts the same components to its own notation — hex for
//! SVG and HTML, a terminal palette index approximation for TUI, pixels for
//! the native renderer — so the identity never depends on the output format.

use crate::error::SVThemeError;

/// One resolved colour, with straight (non-premultiplied) alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SVColour {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl SVColour {
    /// Builds a colour from its four channels.
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Parses `#rgb`, `#rrggbb` or `#rrggbbaa`.
    pub fn from_hex(value: &str) -> Result<Self, SVThemeError> {
        parse_hex_components(value)
            .map(|(red, green, blue, alpha)| Self::new(red, green, blue, alpha))
    }

    pub const fn red(&self) -> u8 {
        self.red
    }

    pub const fn green(&self) -> u8 {
        self.green
    }

    pub const fn blue(&self) -> u8 {
        self.blue
    }

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

    /// Component-wise interpolation in the authored (gamma-encoded) space.
    ///
    /// Interpolating the encoded channels keeps the midpoint visually close to
    /// the authored stops and avoids the darker midpoint of linear-light
    /// blending. The fraction is clamped to `0.0..=1.0`; a non-finite fraction
    /// resolves to the start colour, so a missing value never panics a render.
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
}

fn parse_hex_components(value: &str) -> Result<(u8, u8, u8, u8), SVThemeError> {
    let Some(hex_digits) = value.strip_prefix('#') else {
        return Err(SVThemeError::InvalidValue {
            token_name: "colour".to_string(),
            value: value.to_string(),
            expected: "a colour written as #rgb, #rrggbb or #rrggbbaa".to_string(),
        });
    };
    let digits: Vec<u8> = hex_digits
        .chars()
        .map(|character| {
            character
                .to_digit(16)
                .map(|digit| digit as u8)
                .ok_or_else(|| SVThemeError::InvalidValue {
                    token_name: "colour".to_string(),
                    value: value.to_string(),
                    expected: "hexadecimal digits".to_string(),
                })
        })
        .collect::<Result<_, _>>()?;
    match digits.len() {
        3 => Ok((digits[0] * 17, digits[1] * 17, digits[2] * 17, u8::MAX)),
        6 => Ok((
            digits[0] * 16 + digits[1],
            digits[2] * 16 + digits[3],
            digits[4] * 16 + digits[5],
            u8::MAX,
        )),
        8 => Ok((
            digits[0] * 16 + digits[1],
            digits[2] * 16 + digits[3],
            digits[4] * 16 + digits[5],
            digits[6] * 16 + digits[7],
        )),
        _ => Err(SVThemeError::InvalidValue {
            token_name: "colour".to_string(),
            value: value.to_string(),
            expected: "3, 6 or 8 hexadecimal digits".to_string(),
        }),
    }
}
