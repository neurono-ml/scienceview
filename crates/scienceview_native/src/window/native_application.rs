//! The event loop that presents frames and re-solves the viewport.
//!
//! It is separate from [`SVNativeWindow`](super::SVNativeWindow) so the public
//! description of a window stays independent from the winit callback plumbing.

use super::present_surface::SVPresentSurface;
use super::present_surface::sv_window_failure;
use crate::backend::SVFrameClock;
use crate::backend::SVFrameLoop;
use crate::backend::SVFrameOutcome;
use crate::canvas::SVNativeFrame;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::window::Window;
use winit::window::WindowId;

pub(crate) struct SVNativeApplication<C, F>
where
    C: SVFrameClock,
    F: FnMut(f64) -> Result<SVNativeFrame, SVError>,
{
    pub(crate) viewport: SVViewport,
    pub(crate) frame_loop: SVFrameLoop<C>,
    pub(crate) painter: F,
    pub(crate) position: f64,
    pub(crate) surface: Option<SVPresentSurface>,
    pub(crate) failure: Option<SVError>,
}

impl<C, F> SVNativeApplication<C, F>
where
    C: SVFrameClock,
    F: FnMut(f64) -> Result<SVNativeFrame, SVError>,
{
    fn request_redraw(&self) {
        if let Some(surface) = &self.surface {
            surface.window().request_redraw();
        }
    }

    fn update_viewport(&mut self, physical_width: u32, physical_height: u32, scale: f64) {
        let scale = if scale.is_finite() && scale > 0.0 {
            scale
        } else {
            1.0
        };
        self.viewport = SVViewport::new(
            f64::from(physical_width) / scale,
            f64::from(physical_height) / scale,
        )
        .with_device_pixel_ratio(scale);
    }

    fn paint(&mut self) -> Result<(), SVError> {
        let Some(surface) = self.surface.as_mut() else {
            return Ok(());
        };
        let frame = (self.painter)(self.position)?;
        surface.blit(&frame)
    }
}

impl<C, F> ApplicationHandler for SVNativeApplication<C, F>
where
    C: SVFrameClock,
    F: FnMut(f64) -> Result<SVNativeFrame, SVError>,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.surface.is_some() {
            return;
        }
        let attributes = Window::default_attributes()
            .with_title("scienceview")
            .with_inner_size(LogicalSize::new(
                self.viewport.width(),
                self.viewport.height(),
            ));
        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                self.failure = Some(sv_window_failure(error));
                event_loop.exit();
                return;
            }
        };
        let scale = window.scale_factor();
        let size = window.inner_size();
        self.update_viewport(size.width, size.height, scale);
        match SVPresentSurface::new(window) {
            Ok(surface) => self.surface = Some(surface),
            Err(error) => {
                self.failure = Some(error);
                event_loop.exit();
                return;
            }
        }
        self.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                let scale = self
                    .surface
                    .as_ref()
                    .map(|surface| surface.window().scale_factor())
                    .unwrap_or(1.0);
                self.update_viewport(size.width, size.height, scale);
                self.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let size = self
                    .surface
                    .as_ref()
                    .map(|surface| surface.window().inner_size())
                    .unwrap_or_default();
                self.update_viewport(size.width, size.height, scale_factor);
                self.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if let Err(error) = self.paint() {
                    self.failure = Some(error);
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let mut next_position = None;
        let outcome = self.frame_loop.run_frame(|position| {
            next_position = Some(position);
            true
        });
        match outcome {
            SVFrameOutcome::Presented => {
                if let Some(position) = next_position {
                    self.position = position;
                }
                event_loop.set_control_flow(ControlFlow::Poll);
                self.request_redraw();
            }
            SVFrameOutcome::Idle => event_loop.set_control_flow(ControlFlow::Wait),
        }
    }
}
