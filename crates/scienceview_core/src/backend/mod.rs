//! Rendering backends: the closed backend kind, the capability matrix, the
//! `SVRenderBackend` contract and the per-operation dispatch seam.

mod capability;
mod dispatch;
mod kind;
mod matrix;
mod registry;
mod render_backend;

pub use capability::SVBackendCapability;
pub use dispatch::sv_resolve_render_backend;
pub use kind::SVBackendKind;
pub use matrix::SVBackendCapabilities;
pub use matrix::SVBackendRequirements;
pub use matrix::sv_backend_capabilities;
pub use matrix::sv_verify_backend_requirements;
pub use registry::SVBackendRegistry;
pub use registry::sv_backend_registry;
pub use render_backend::SVRenderBackend;
pub use render_backend::SVRenderBackendProvider;
pub use render_backend::SVRenderOutput;
pub use render_backend::SVRenderRequest;
pub use render_backend::SVSaveRequest;
pub use render_backend::SVShowRequest;
pub use render_backend::SVUpdateRequest;

#[cfg(test)]
mod backend_kind_tests;
#[cfg(test)]
mod dispatch_tests;
#[cfg(test)]
mod matrix_tests;
#[cfg(test)]
pub(crate) mod test_support;
