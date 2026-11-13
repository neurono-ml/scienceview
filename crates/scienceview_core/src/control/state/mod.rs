//! The reducer: controls apply actions to model state, never to a figure.

mod definition;
mod dispatch;
mod domain_dispatch;
mod geometry;
mod model_dispatch;

pub use definition::SVControlState;
pub use dispatch::SVControlDispatch;
