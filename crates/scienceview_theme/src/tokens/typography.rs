//! Typography tokens: one family, three sizes and two weights, all backend
//! independent. A terminal backend approximates them; a web backend emits them
//! as CSS custom properties.

/// The typography half of a compiled theme.
#[derive(Debug, Clone, PartialEq)]
pub struct SVTypographyTokens {
    pub(crate) family: String,
    pub(crate) size: SVFontSizes,
    pub(crate) weight: SVFontWeights,
    pub(crate) line_height: f64,
}

impl SVTypographyTokens {
    /// The authored font family name, without any backend-specific fallback.
    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn size(&self) -> &SVFontSizes {
        &self.size
    }

    pub fn weight(&self) -> &SVFontWeights {
        &self.weight
    }

    /// Line height as a multiple of the font size.
    pub fn line_height(&self) -> f64 {
        self.line_height
    }
}

/// The three typographic sizes of a figure: body copy, title and axis label.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SVFontSizes {
    pub(crate) body: f64,
    pub(crate) title: f64,
    pub(crate) label: f64,
}

impl SVFontSizes {
    pub fn body(&self) -> f64 {
        self.body
    }

    pub fn title(&self) -> f64 {
        self.title
    }

    pub fn label(&self) -> f64 {
        self.label
    }
}

/// The two font weights a figure needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SVFontWeights {
    pub(crate) regular: u16,
    pub(crate) bold: u16,
}

impl SVFontWeights {
    pub fn regular(&self) -> u16 {
        self.regular
    }

    pub fn bold(&self) -> u16 {
        self.bold
    }
}
