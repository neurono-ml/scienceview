//! Scripts as named, reloadable resources.

mod builder;
mod dispatch;
mod inspection;
mod registry;
mod script;

pub use builder::{SV_DEFAULT_STATEMENT_BUDGET, SVScriptRuntimeBuilder};
pub use dispatch::sv_dispatch_outcome;
pub use registry::SVScriptRuntime;
pub use script::{SVScript, SVScriptBuilder, SVScriptStatePolicy};

#[cfg(test)]
#[path = "registry_tests.rs"]
mod registry_tests;
#[cfg(test)]
#[path = "reload_tests.rs"]
mod reload_tests;
