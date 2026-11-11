//! Rendering backends: the closed backend kind, the `SVRenderBackend` contract
//! and the per-operation dispatch seam.

#[cfg(test)]
mod backend_kind_tests;
mod capability;
#[cfg(test)]
mod dispatch_tests;
mod kind;

pub use capability::SVBackendCapability;
pub use kind::SVBackendKind;
