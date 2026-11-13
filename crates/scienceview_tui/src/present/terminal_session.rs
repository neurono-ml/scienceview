//! The interactive terminal session: draw, resize and quit.
//!
//! Ratatui owns the buffer and the diffing; crossterm owns raw mode, the
//! alternate screen and the event source. The session never owns a timer: a
//! frame source calls [`SVTerminalSession::draw`] when its own clock says a
//! frame is due.

use crate::backend::SVTuiBackend;
use crate::cell::SVCellFrame;
use crate::present::buffer_fill::sv_fill_buffer;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use ratatui::DefaultTerminal;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_error::SVError;
use std::time::Duration;

/// One thing that happened while the session waited for input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVTerminalEvent {
    /// The terminal changed size, in columns and rows.
    Resize(usize, usize),
    /// The user asked to quit (`q`, Escape or Control-C).
    Quit,
    /// Nothing relevant happened before the timeout.
    None,
}

/// An open terminal in raw mode on the alternate screen.
pub struct SVTerminalSession {
    terminal: DefaultTerminal,
    active: bool,
}

impl SVTerminalSession {
    /// Opens the terminal, entering raw mode and the alternate screen.
    pub fn open() -> Result<Self, SVError> {
        let terminal = ratatui::try_init().map_err(|error| sv_terminal_failure("open", error))?;
        Ok(Self {
            terminal,
            active: true,
        })
    }

    /// Returns the current terminal size in columns and rows.
    pub fn size(&self) -> Result<(usize, usize), SVError> {
        self.terminal
            .size()
            .map(|size| (usize::from(size.width), usize::from(size.height)))
            .map_err(|error| sv_terminal_failure("size", error))
    }

    /// Draws a frame, clipping it to the terminal.
    pub fn draw(&mut self, frame: &SVCellFrame) -> Result<(), SVError> {
        self.terminal
            .draw(|target| sv_fill_buffer(frame, target.buffer_mut()))
            .map(|_| ())
            .map_err(|error| sv_terminal_failure("draw", error))
    }

    /// Waits up to `timeout` for a resize or a quit key.
    pub fn next_event(&mut self, timeout: Duration) -> Result<SVTerminalEvent, SVError> {
        if !crossterm::event::poll(timeout).map_err(|error| sv_terminal_failure("poll", error))? {
            return Ok(SVTerminalEvent::None);
        }

        match crossterm::event::read().map_err(|error| sv_terminal_failure("read", error))? {
            Event::Resize(columns, rows) => Ok(SVTerminalEvent::Resize(
                usize::from(columns),
                usize::from(rows),
            )),
            Event::Key(key)
                if key.kind != KeyEventKind::Release
                    && (matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                        || (key.code == KeyCode::Char('c')
                            && key.modifiers.contains(KeyModifiers::CONTROL))) =>
            {
                Ok(SVTerminalEvent::Quit)
            }
            _ => Ok(SVTerminalEvent::None),
        }
    }

    /// Leaves raw mode and the alternate screen.
    pub fn close(mut self) {
        if self.active {
            ratatui::restore();
            self.active = false;
        }
    }
}

impl Drop for SVTerminalSession {
    fn drop(&mut self) {
        if self.active {
            ratatui::restore();
        }
    }
}

/// Presents a request, redrawing whenever the terminal resizes.
pub(crate) fn sv_present_terminal(request: &SVRenderRequest<'_>) -> Result<(), SVError> {
    let backend = SVTuiBackend::new();
    let mut session = SVTerminalSession::open()?;
    let mut size = session.size()?;

    loop {
        let frame = backend.render_frame_for_terminal(request, size.0, size.1)?;
        session.draw(&frame)?;

        match session.next_event(Duration::from_millis(250))? {
            SVTerminalEvent::Quit => break,
            SVTerminalEvent::Resize(columns, rows) => size = (columns, rows),
            SVTerminalEvent::None => {}
        }
    }

    session.close();
    Ok(())
}

fn sv_terminal_failure(operation: &'static str, error: std::io::Error) -> SVError {
    SVError::BackendFailure {
        backend_kind: SVBackendKind::Tui,
        operation,
        message: format!("terminal {operation} failed: {error}"),
    }
}
