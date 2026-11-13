//! The shared component contract, data embedding and source emitter.

mod contract;
mod data_serialization;
mod emitter;
mod escaping;
mod templates;

pub use contract::{SV_REACT_COMPONENT_CONTRACT, SVReactComponentContract};
pub(crate) use emitter::sv_render_component_source;
pub use emitter::sv_render_react_component;
