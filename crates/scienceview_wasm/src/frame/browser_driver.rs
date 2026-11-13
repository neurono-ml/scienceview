//! The `requestAnimationFrame` driver, compiled only for the browser target.
//!
//! The driver is the only code that touches animation frames: every frame hands
//! its timestamp to the model clock, and while the clock answers `None` the
//! loop keeps idling instead of advancing. It never stores an animation state of
//! its own.

use super::frame_loop::SVWasmFrameLoop;
use super::model_clock::SVWasmModelClock;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_error::SVError;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// The running callback and the identifier of the frame it scheduled.
type SVAnimationRegistration = Rc<RefCell<Option<(Closure<dyn FnMut(f64)>, i32)>>>;

/// Drives animation frames until it is stopped or dropped.
pub struct SVBrowserFrameDriver {
    registration: SVAnimationRegistration,
}

impl SVBrowserFrameDriver {
    /// Starts the loop.
    ///
    /// `clock` maps each frame timestamp to a playback position, and `draw`
    /// returns `true` when it presented a frame.
    pub fn start<C, D>(clock: C, mut draw: D) -> Result<Self, SVError>
    where
        C: SVWasmModelClock + 'static,
        D: FnMut(f64) -> bool + 'static,
    {
        let window = web_sys::window().ok_or_else(sv_missing_window)?;
        let frame_loop = Rc::new(RefCell::new(SVWasmFrameLoop::new(clock)));
        let registration: SVAnimationRegistration = Rc::new(RefCell::new(None));

        let scheduler = Rc::clone(&registration);
        let loop_handle = Rc::clone(&frame_loop);
        let callback = Closure::<dyn FnMut(f64)>::new(move |timestamp_milliseconds: f64| {
            let frame_time = timestamp_milliseconds / 1000.0;
            {
                let mut frame_loop = loop_handle.borrow_mut();
                frame_loop.run_frame(frame_time, &mut draw);
            }

            if let Some(window) = web_sys::window() {
                let next = scheduler
                    .borrow()
                    .as_ref()
                    .map(|(callback, _)| callback.as_ref().clone());
                if let Some(next) = next {
                    let next: js_sys::Function = next.unchecked_into();
                    if let Ok(identifier) = window.request_animation_frame(&next) {
                        if let Some(entry) = scheduler.borrow_mut().as_mut() {
                            entry.1 = identifier;
                        }
                    }
                }
            }
        });

        let first_callback: js_sys::Function = callback
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .clone();
        let identifier = window
            .request_animation_frame(&first_callback)
            .map_err(|_| sv_frame_error())?;

        *registration.borrow_mut() = Some((callback, identifier));

        Ok(Self { registration })
    }

    /// Stops the loop and releases the callback.
    pub fn stop(&self) {
        let registration = self.registration.borrow_mut().take();
        if let Some((_, identifier)) = registration {
            if let Some(window) = web_sys::window() {
                let _ = window.cancel_animation_frame(identifier);
            }
        }
    }
}

impl Drop for SVBrowserFrameDriver {
    fn drop(&mut self) {
        self.stop();
    }
}

fn sv_missing_window() -> SVError {
    sv_frame_error()
}

fn sv_frame_error() -> SVError {
    SVError::BackendFailure {
        backend_kind: SVBackendKind::Wasm,
        operation: "show",
        message: "the browser animation frame could not be scheduled".to_string(),
    }
}
