//! Presenting the newest published frame through an abstract presenter.

use super::frame_channel::SVFrameChannel;
use crate::canvas::SVNativeFrame;
use scienceview_core::sv_error::SVError;

/// A surface that shows one frame.
///
/// The window's softbuffer surface implements it; a test implements it with a
/// recorder, so the update path is exercised without a display.
pub(crate) trait SVPresenter {
    /// Shows one frame.
    fn present(&mut self, frame: &SVNativeFrame) -> Result<(), SVError>;
}

/// Presents published frames in place.
#[derive(Default)]
pub(crate) struct SVPresentLoop {
    channel: SVFrameChannel,
}

impl SVPresentLoop {
    /// Builds a loop around a channel.
    pub(crate) fn new(channel: SVFrameChannel) -> Self {
        Self { channel }
    }

    /// Presents the pending frame, if any.
    ///
    /// Returns `true` when a frame was presented.
    pub(crate) fn present_pending(
        &mut self,
        presenter: &mut impl SVPresenter,
    ) -> Result<bool, SVError> {
        let Some(frame) = self.channel.take() else {
            return Ok(false);
        };
        presenter.present(&frame)?;
        Ok(true)
    }
}
