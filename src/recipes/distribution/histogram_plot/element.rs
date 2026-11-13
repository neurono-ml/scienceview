//! How the bins of a histogram are drawn.
//!
//! The bar element draws one rectangle per bin. The step element draws the
//! bin tops as a stepped outline, and the polygon element fills the region
//! under the bin tops. All three consume the same binned values; the element
//! only selects the mark that expresses them.

/// The mark that expresses the bins.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVBarElement {
    /// One rectangle per bin, with the configured gap (default).
    #[default]
    Bar,
    /// A stepped outline through the bin tops.
    Step,
    /// A filled polygon under the bin tops.
    Poly,
}

impl SVBarElement {
    /// Returns the element name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVBarElement::Bar => "Bar",
            SVBarElement::Step => "Step",
            SVBarElement::Poly => "Poly",
        }
    }
}
