//! Placing one panel's SVG fragment into its solved cell rectangle.

use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;

/// Places a panel fragment at the cell rectangle.
///
/// The fragment must be one `<svg>` element with a `viewBox`. The dashboard
/// sets its `x`, `y`, `width` and `height` and preserves the viewBox, so the
/// content scales into the cell without re-deriving any geometry. The transform
/// is pure string surgery on the opening tag; the element list, attribute order
/// and number format stay deterministic.
pub(super) fn sv_place_panel_fragment(
    fragment: &str,
    rectangle: &SVResolvedRectangle,
    panel_identifier: &str,
) -> Result<String, SVError> {
    let start = fragment
        .find("<svg")
        .ok_or_else(|| sv_fragment_error(panel_identifier, "it has no `<svg>` element"))?;
    let end = fragment[start..]
        .find('>')
        .map(|offset| start + offset)
        .ok_or_else(|| sv_fragment_error(panel_identifier, "its `<svg>` element is not closed"))?;
    let opening = &fragment[start..end];
    if !opening.contains("viewBox=\"") {
        return Err(sv_fragment_error(
            panel_identifier,
            "its `<svg>` element declares no `viewBox`, so it cannot scale into the cell",
        ));
    }

    let opening = sv_remove_attribute(opening, "width");
    let opening = sv_remove_attribute(&opening, "height");
    let mut placed = opening.trim_end().to_string();
    placed.push_str(&format!(
        " x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\">",
        scienceview_svg::sv_format_number(rectangle.x),
        scienceview_svg::sv_format_number(rectangle.y),
        scienceview_svg::sv_format_number(rectangle.width),
        scienceview_svg::sv_format_number(rectangle.height),
    ));

    Ok(format!(
        "{}{}{}",
        &fragment[..start],
        placed,
        &fragment[end + 1..]
    ))
}

fn sv_remove_attribute(opening: &str, name: &str) -> String {
    let needle = format!(" {name}=\"");
    let Some(start) = opening.find(&needle) else {
        return opening.to_string();
    };
    let value_start = start + needle.len();
    let Some(value_end) = opening[value_start..].find('"') else {
        return opening.to_string();
    };
    let end = value_start + value_end + 1;
    format!("{}{}", &opening[..start], &opening[end..])
}

fn sv_fragment_error(panel_identifier: &str, message: &str) -> SVError {
    SVError::InvalidDashboardPanelFragment {
        panel: panel_identifier.to_owned(),
        message: message.to_string(),
    }
}
