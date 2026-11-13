//! Where an event came from.

use core::fmt;

/// Where an event came from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SVControlOrigin {
    /// A built-in control, by its name.
    Control(String),
    /// A third-party widget, by its name.
    ExternalWidget(String),
    /// A script, by its registered name.
    Script(String),
    /// A programmatic dispatch with no widget attached.
    Programmatic,
}

impl fmt::Display for SVControlOrigin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Control(name) => write!(formatter, "control={name}"),
            Self::ExternalWidget(name) => write!(formatter, "widget={name}"),
            Self::Script(name) => write!(formatter, "script={name}"),
            Self::Programmatic => formatter.write_str("origin=programmatic"),
        }
    }
}
