//! The engine contract, its execution context and the host-executor adapter.

mod engine;
mod execution;
mod host;

pub use engine::SVScriptEngine;
pub use execution::SVScriptExecution;
pub use host::{
    SVHostExecutionRequest, SVHostExecutorEngine, SVHostScriptExecutor, sv_host_executor_engine,
};
