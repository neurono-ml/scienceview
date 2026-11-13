//! The shared component contract, data embedding and source emitter.

mod contract;
mod data_serialization;
mod emitter;
mod escaping;
mod templates;

pub use contract::{SV_ANGULAR_COMPONENT_CONTRACT, SVAngularComponentContract};
pub use emitter::sv_render_angular_component;
pub(crate) use emitter::sv_render_component_source;
