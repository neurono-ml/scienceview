//! Rendering backends: the closed backend kind, the capability matrix, the
//! `SVRenderBackend` contract and the per-operation dispatch seam.

#[cfg(test)]
mod backend_kind_tests;
mod capability;
mod dispatch;
#[cfg(test)]
mod dispatch_tests;
mod kind;
mod matrix;
mod registry;
mod render_backend;

pub use capability::SVBackendCapability;
pub use dispatch::sv_resolve_render_backend;
pub use kind::SVBackendKind;
pub use matrix::SVBackendCapabilities;
pub use matrix::sv_backend_capabilities;
pub use registry::SVBackendRegistry;
pub use registry::sv_backend_registry;
pub use render_backend::SVRenderBackend;
pub use render_backend::SVRenderBackendProvider;
pub use render_backend::SVRenderOutput;
pub use render_backend::SVRenderRequest;
pub use render_backend::SVSaveRequest;
pub use render_backend::SVShowRequest;
pub use render_backend::SVUpdateRequest;
