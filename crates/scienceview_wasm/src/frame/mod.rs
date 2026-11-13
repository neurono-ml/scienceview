//! The model-clock frame loop of the WebAssembly target.

mod frame_loop;
mod model_clock;

pub use frame_loop::SVWasmFrameLoop;
pub use frame_loop::SVWasmFrameOutcome;
pub use model_clock::SVPausedWasmClock;
pub use model_clock::SVWasmModelClock;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
mod browser_driver;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use browser_driver::SVBrowserFrameDriver;

#[cfg(test)]
#[path = "frame_tests.rs"]
mod frame_tests;
