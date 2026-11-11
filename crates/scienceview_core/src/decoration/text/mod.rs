//! Text measured as a box, without font rasterization.

mod text_box;

pub use text_box::{SVTextBox, sv_estimate_text_width};

#[cfg(test)]
#[path = "text_tests.rs"]
mod text_tests;
