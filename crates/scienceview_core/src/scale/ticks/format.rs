//! How a tick value becomes a label.

/// The label format of a tick.
///
/// The format is a layout decision like the tick count: a caller that knows the
/// unit (milliseconds, percentages, counts) pins the decimals, while the
/// automatic format keeps the label short.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVTickFormat {
    /// Shortest reasonable representation, trailing zeros trimmed.
    #[default]
    Automatic,
    /// A fixed number of decimals.
    Fixed(usize),
    /// A whole number, rounded.
    Integer,
}

impl SVTickFormat {
    /// Formats a tick value.
    pub fn format(&self, value: f64) -> String {
        if !value.is_finite() {
            return value.to_string();
        }

        match self {
            SVTickFormat::Automatic => sv_automatic_label(value),
            SVTickFormat::Fixed(decimals) => format!("{value:.decimals$}"),
            SVTickFormat::Integer => format!("{:.0}", value.round()),
        }
    }
}

/// Formats a value without trailing zeros and without scientific noise.
fn sv_automatic_label(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if value.abs() >= 1_000_000.0 || value.abs() < 0.000_1 {
        return format!("{value:.3e}");
    }

    let formatted = format!("{value:.6}");
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    trimmed.to_string()
}
