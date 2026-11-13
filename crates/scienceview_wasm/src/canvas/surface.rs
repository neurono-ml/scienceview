//! The seam between the renderer and whatever receives its pixels.

use crate::SVNativeFrame;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;

/// A surface that accepts a finished frame.
///
/// The renderer knows nothing about a DOM: it asks the surface for the viewport
/// it should solve the layout for and hands back the frame. The host target
/// captures it in memory; the browser path blits it into a canvas.
pub trait SVCanvasSurface: Send + Sync {
    /// Returns the viewport the surface is sized for.
    fn viewport(&self) -> SVViewport;

    /// Presents one frame.
    ///
    /// A frame whose device size differs from the surface is rejected rather
    /// than silently cropped.
    fn blit(&self, frame: &SVNativeFrame) -> Result<(), SVError>;
}
