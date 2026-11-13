//! The engine contract: kinds, placements, prepared programs and outcomes.

mod contract;
mod kind;
mod outcome;
mod placement;
mod program;
mod span;

pub use contract::{
    SVHostExecutionRequest, SVHostExecutorEngine, SVHostScriptExecutor, SVScriptEngine,
    SVScriptExecution, sv_host_executor_engine,
};
pub use kind::SVScriptEngineKind;
pub use outcome::{
    SVScriptActionInvocation, SVScriptCapabilityRequest, SVScriptOutcome, SVScriptStateChange,
};
pub use placement::SVScriptExecutionPlacement;
pub use program::SVScriptProgram;
pub use span::SVScriptSpan;

pub(crate) use program::sv_source_digest;
