//! Deterministic text and number formatting shared by every serializer.

mod escape;
mod number;

pub use escape::sv_escape_text;
pub use number::sv_format_number;
