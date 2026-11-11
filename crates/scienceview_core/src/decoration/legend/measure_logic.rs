//! Measuring a legend: bounded rows and the overflow summary.

use super::entry::SVLegendEntry;
use super::measurement::{SVLegendCoordinates, SVLegendMeasurement};
use crate::sv_decoration::{SVDecorationStyle, SVTextBox};

/// Measures the legend entries into a bounded grid.
pub(crate) fn sv_measure_legend(
    entries: &[SVLegendEntry],
    maximum_number_of_entries: usize,
    maximum_number_of_rows: usize,
    style: &SVDecorationStyle,
) -> SVLegendMeasurement {
    let maximum_entries = maximum_number_of_entries.max(1);
    let total = entries.len();
    let (drawn_entries, overflow) = if total > maximum_entries {
        let drawn = maximum_entries.saturating_sub(1);
        (drawn, total - drawn)
    } else {
        (total, 0)
    };

    let mut entry_boxes: Vec<SVTextBox> = entries
        .iter()
        .take(drawn_entries)
        .map(|entry| sv_entry_box(entry.label(), style))
        .collect();
    if overflow > 0 {
        entry_boxes.push(sv_entry_box(&format!("+{overflow} more"), style));
    }

    let row_height = style.body_font_size() * style.line_height();
    let swatch = style.body_font_size();
    let column_width = entry_boxes
        .iter()
        .map(|label_box| swatch + style.gap() / 2.0 + label_box.width())
        .fold(0.0_f64, f64::max);
    let rows = entry_boxes
        .len()
        .min(maximum_number_of_rows.max(1))
        .max(usize::from(!entry_boxes.is_empty()));
    let columns = if rows == 0 {
        0
    } else {
        entry_boxes.len().div_ceil(rows)
    };
    let gap = style.gap();
    let width = if columns == 0 {
        0.0
    } else {
        columns as f64 * column_width + (columns - 1) as f64 * gap
    };
    let height = if rows == 0 {
        0.0
    } else {
        rows as f64 * row_height + (rows - 1) as f64 * gap
    };

    SVLegendMeasurement::new(
        entry_boxes,
        drawn_entries,
        overflow,
        SVLegendCoordinates {
            columns,
            rows,
            column_width,
            row_height,
            width,
            height,
        },
    )
}

/// Builds the text box of one entry label.
pub(crate) fn sv_entry_box(label: &str, style: &SVDecorationStyle) -> SVTextBox {
    SVTextBox::new(
        label.to_string(),
        style.body_font_size(),
        style.line_height(),
        style.regular_weight(),
    )
}
