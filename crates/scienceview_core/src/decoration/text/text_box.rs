//! A text box: the size a label needs, independent of any font rasterizer.

/// One text label measured as a box.
///
/// The decoration layer lays text out as boxes so that a backend owns glyphs
/// and a decoration owns geometry. The width is an advance estimate — the
/// character count times [`SVTextBox::ADVANCE_FACTOR`] times the font size —
/// because no font is loaded in the core. A backend with real metrics
/// rasterizes the same box more precisely without changing the layout contract.
#[derive(Clone, Debug, PartialEq)]
pub struct SVTextBox {
    text: String,
    font_size: f64,
    line_height: f64,
    weight: u16,
}

impl SVTextBox {
    /// The average advance of one character, as a fraction of the font size.
    ///
    /// The value is a documented estimate, not a measurement: 0.6 em is close
    /// to the average advance of the default sans-serif family for mixed text.
    pub const ADVANCE_FACTOR: f64 = 0.6;

    /// Builds a text box.
    ///
    /// A non-finite or non-positive font size and line height become the
    /// documented defaults of one pixel and `1.0`, so a malformed style never
    /// produces a non-finite layout.
    pub fn new(text: impl Into<String>, font_size: f64, line_height: f64, weight: u16) -> Self {
        Self {
            text: text.into(),
            font_size: sv_sanitize_positive(font_size),
            line_height: sv_sanitize_positive(line_height),
            weight,
        }
    }

    /// Returns the text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the font size.
    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    /// Returns the line height as a multiple of the font size.
    pub fn line_height(&self) -> f64 {
        self.line_height
    }

    /// Returns the font weight.
    pub fn weight(&self) -> u16 {
        self.weight
    }

    /// Returns the estimated width of the box.
    pub fn width(&self) -> f64 {
        sv_estimate_text_width(&self.text, self.font_size)
    }

    /// Returns the height of the box.
    pub fn height(&self) -> f64 {
        self.font_size * self.line_height
    }

    /// Returns the number of characters the text contains.
    pub fn character_count(&self) -> usize {
        self.text.chars().count()
    }
}

/// Estimates the width of a string at a font size.
///
/// This is the single place the core approximates text metrics. A non-finite
/// or negative font size contributes nothing, so the estimate is always finite.
pub fn sv_estimate_text_width(text: &str, font_size: f64) -> f64 {
    let font_size = if font_size.is_finite() && font_size > 0.0 {
        font_size
    } else {
        return 0.0;
    };
    let characters = text.chars().count() as f64;

    characters * SVTextBox::ADVANCE_FACTOR * font_size
}

fn sv_sanitize_positive(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        1.0
    }
}
