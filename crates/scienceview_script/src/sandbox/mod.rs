//! Sandboxing: restricted by default, widened only on explicit request.

mod capability;
mod policy;

pub use capability::SVSandboxCapability;
pub use policy::SVSandboxPolicy;

#[cfg(test)]
#[path = "policy_tests.rs"]
mod policy_tests;
