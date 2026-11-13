/// Where a control is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SVControlPlacement {
    /// Inside the figure's own frame.
    #[default]
    InFrame,
    /// In an external host arrangement supplied by the application.
    Host,
}

impl SVControlPlacement {
    /// Returns the placement name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InFrame => "in-frame",
            Self::Host => "host",
        }
    }
}
