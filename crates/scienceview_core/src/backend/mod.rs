//! Rendering backends: the closed backend kind, the `SVRenderBackend` contract
//! and the per-operation dispatch seam.

mod capability;
mod kind;

pub use capability::SVBackendCapability;
pub use kind::SVBackendKind;
