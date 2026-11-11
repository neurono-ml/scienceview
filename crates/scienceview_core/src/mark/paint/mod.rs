//! Paint: how a mark shape is filled and stroked, and the tokens that default it.

mod mark_paint;
mod mark_theme;
mod resolution;

pub use mark_paint::SVMarkPaint;
pub use mark_theme::SVMarkTheme;
pub(crate) use resolution::{sv_resolve_colour, sv_resolve_length, sv_resolve_opacity};
