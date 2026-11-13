//! Converting a rendered cell frame into a ratatui buffer.

use crate::cell::SVCellFrame;
use ratatui::buffer::Buffer;
use ratatui::style::Color;

/// Copies every cell of `frame` into `buffer`.
///
/// A cell outside the buffer is skipped, so a frame larger than the terminal
/// is clipped instead of panicking. Transparent colours keep the buffer's
/// default, which lets the terminal theme show through.
pub fn sv_fill_buffer(frame: &SVCellFrame, buffer: &mut Buffer) {
    let area = buffer.area;
    let rows = frame.height().min(area.height as usize);
    let columns = frame.width().min(area.width as usize);

    for row in 0..rows {
        for column in 0..columns {
            let Some(source) = frame.cell(column, row) else {
                continue;
            };
            if source.is_continuation() {
                continue;
            }
            let Some(target) = buffer.cell_mut((column as u16, row as u16)) else {
                continue;
            };

            target.set_char(source.symbol());
            if let Some(colour) = source.foreground() {
                target.fg = Color::Rgb(colour.red(), colour.green(), colour.blue());
            }
            if let Some(colour) = source.background() {
                target.bg = Color::Rgb(colour.red(), colour.green(), colour.blue());
            }
        }
    }
}
