//! Placing the legend entries into a solved rectangle.

use super::legend_model::SVLegend;
use super::measurement::SVLegendMeasurement;
use crate::sv_decoration::{
    SVDecorationPrimitive, SVDecorationStyle, SVPoint, SVRectanglePrimitive, SVStroke, SVTextAnchor,
};
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_scale::SVColour;

/// One placed legend entry.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLegendEntryPlacement {
    series_key: String,
    label: String,
    colour: SVColour,
    visible: bool,
    rectangle: SVResolvedRectangle,
}

impl SVLegendEntryPlacement {
    /// Returns the series key, empty for the overflow row.
    pub fn series_key(&self) -> &str {
        &self.series_key
    }

    /// Returns the label drawn.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the swatch colour.
    pub fn colour(&self) -> SVColour {
        self.colour
    }

    /// Returns `true` when the series is visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Returns the row rectangle, useful for hit-testing a toggle.
    pub fn rectangle(&self) -> SVResolvedRectangle {
        self.rectangle
    }
}

/// The placed legend.
#[derive(Clone, Debug, PartialEq)]
pub struct SVLegendPlacement {
    rectangle: SVResolvedRectangle,
    entries: Vec<SVLegendEntryPlacement>,
    overflow: usize,
    primitives: Vec<SVDecorationPrimitive>,
}

impl SVLegendPlacement {
    pub(crate) fn sv_place(
        legend: &SVLegend,
        measurement: &SVLegendMeasurement,
        rectangle: SVResolvedRectangle,
        style: &SVDecorationStyle,
    ) -> Self {
        let gap = style.gap();
        let rows = measurement.rows();
        let column_width = measurement.column_width();
        let row_height = measurement.row_height();
        let swatch = style.body_font_size();
        let mut entries = Vec::with_capacity(measurement.entry_boxes().len());
        let mut primitives = Vec::new();

        for index in 0..measurement.entry_boxes().len() {
            let column = if rows == 0 { 0 } else { index / rows };
            let row = if rows == 0 { 0 } else { index % rows };
            let x = rectangle.x + column as f64 * (column_width + gap);
            let y = rectangle.y + row as f64 * (row_height + gap);
            let row_rectangle = SVResolvedRectangle::new(x, y, column_width, row_height);
            let is_overflow = index >= measurement.drawn_entries();
            let (series_key, label, colour, visible) = if is_overflow {
                (
                    String::new(),
                    format!("+{} more", measurement.overflow()),
                    style.text_muted(),
                    true,
                )
            } else if let Some(entry) = legend.entries().get(index) {
                (
                    entry.series_key().to_string(),
                    entry.label().to_string(),
                    entry.colour(),
                    entry.is_visible(),
                )
            } else {
                continue;
            };

            if !is_overflow {
                let swatch_rectangle =
                    SVResolvedRectangle::new(x, y + (row_height - swatch) / 2.0, swatch, swatch);
                primitives.push(SVDecorationPrimitive::rectangle(
                    SVRectanglePrimitive::filled(
                        swatch_rectangle,
                        sv_swatch_colour(colour, visible),
                    ),
                ));
                primitives.push(SVDecorationPrimitive::rectangle(
                    SVRectanglePrimitive::outlined(
                        swatch_rectangle,
                        SVStroke::new(style.axis(), style.axis_line_width()),
                    ),
                ));
            }

            let label_x = if is_overflow {
                x
            } else {
                x + swatch + gap / 2.0
            };
            primitives.push(SVDecorationPrimitive::text(
                SVPoint::new(label_x, y + row_height),
                label.clone(),
                style.body_font_size(),
                style.regular_weight(),
                if visible {
                    style.text()
                } else {
                    style.text_muted()
                },
                SVTextAnchor::Start,
            ));

            entries.push(SVLegendEntryPlacement {
                series_key,
                label,
                colour,
                visible,
                rectangle: row_rectangle,
            });
        }

        Self {
            rectangle,
            entries,
            overflow: measurement.overflow(),
            primitives,
        }
    }

    /// Returns the rectangle the legend occupies.
    pub fn rectangle(&self) -> SVResolvedRectangle {
        self.rectangle
    }

    /// Returns the placed rows.
    pub fn entries(&self) -> &[SVLegendEntryPlacement] {
        &self.entries
    }

    /// Returns how many series the overflow row summarizes.
    pub fn overflow(&self) -> usize {
        self.overflow
    }

    /// Returns the backend-agnostic primitives that draw the legend.
    pub fn primitives(&self) -> &[SVDecorationPrimitive] {
        &self.primitives
    }
}

fn sv_swatch_colour(colour: SVColour, visible: bool) -> SVColour {
    if visible {
        colour
    } else {
        colour.with_alpha(colour.alpha() / 3)
    }
}
