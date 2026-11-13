//! `scienceview_script`: behaviour authored at runtime, executed host-side by
//! default, reloadable and sandboxed.
//!
//! A script is a named resource. The runtime owns it, an engine prepares it,
//! and every invocation turns the script's statements into semantic actions of
//! the [`sv_control`](scienceview_core::sv_control) vocabulary — the same
//! actions a built-in control or a third-party widget dispatches. Scripting is
//! therefore not a second mutation path: it is one more producer of the single
//! contract.
//!
//! ## Engines are a trait
//!
//! [`SVScriptEngine`] is the pluggable contract: prepare a source once, execute
//! it many times. [`SVLuaScriptEngine`] and [`SVTypeScriptScriptEngine`] are
//! the two shipped engines. A third-party language plugs in by implementing the
//! trait, or by implementing the narrower [`SVHostScriptExecutor`] and wrapping
//! it with [`sv_host_executor_engine`].
//!
//! ## Dependency-light by design, and honest about it
//!
//! The shipped engines do **not** embed a Lua virtual machine or a JavaScript
//! engine: either would add a heavy dependency to the default build. They are
//! reference front-ends over a minimal, deterministic interpreter that accepts
//! a documented subset of statement shapes — `dispatch`, `increment`, `log` and
//! the five capability calls — in the surface syntax of each language. The
//! subset is enough to exercise the whole contract (parsing, deterministic
//! dispatch, sandboxing, reload, bounded execution) with no external runtime.
//! To run full-language scripts, implement [`SVScriptEngine`] over a real
//! interpreter, or [`SVHostScriptExecutor`] over one and use the adapter; the
//! runtime, sandbox policy and reload machinery are unchanged.
//!
//! ```
//! use scienceview_core::sv_control::{SVControlAction, SVControlAxis};
//! use scienceview_script::{
//!     SVScript, SVScriptEngineKind, SVScriptRuntimeBuilder, SVScriptStatePolicy,
//! };
//!
//! let mut runtime = SVScriptRuntimeBuilder::new()
//!     .engine(SVScriptEngineKind::Lua)
//!     .build()
//!     .expect("the engine is available");
//!
//! runtime
//!     .register(
//!         SVScript::builder("zoom-on-event")
//!             .source("dispatch(\"zoom\", { axis = \"x\", factor = 0.8, anchor = 0.5 })")
//!             .build()
//!             .expect("the script is complete"),
//!     )
//!     .expect("the script loads");
//!
//! let outcome = runtime.invoke("zoom-on-event", None).expect("the script runs");
//! assert_eq!(
//!     outcome.actions()[0].action(),
//!     &SVControlAction::Zoom {
//!         axis: SVControlAxis::Horizontal,
//!         factor: 0.8,
//!         anchor: 0.5,
//!     }
//! );
//!
//! // The source is a runtime resource: replacing it keeps the script's state
//! // when the caller asks for it.
//! runtime
//!     .replace(
//!         "zoom-on-event",
//!         "dispatch(\"reset\")",
//!         SVScriptStatePolicy::Preserve,
//!     )
//!     .expect("the replacement loads");
//! ```
//!
//! ## Sandboxing
//!
//! A script starts with the `graphics` capability only. `read_file`,
//! `write_file`, `http_get`, `run_process` and `read_environment` are refused
//! with [`SVError::ScriptSandboxViolation`](scienceview_core::sv_error::SVError::ScriptSandboxViolation)
//! unless the script explicitly widens its policy. The built-in interpreter
//! performs no system access at all: an allowed capability call is *reported*
//! in the outcome for a host adapter to service, so the sandbox is the
//! authority even when an external interpreter is plugged in.
//!
//! ## Failures are contained
//!
//! Every failure is a typed [`SVError`] carrying the script identity and an
//! [`SVScriptLocation`] (line, column and byte span). The runtime never panics
//! and never aborts the host: an invocation returns the error, the registry
//! keeps its scripts, and the next invocation works normally.
//!
//! ## Execution placement
//!
//! [`SVScriptExecutionPlacement::Host`] is the default. `Embedded` is opt-in
//! behind the `embedded-execution` feature and only marks the script for the
//! WebAssembly artifact; the artifact-side bridge belongs to the WebAssembly
//! backend.

#![warn(missing_docs)]

mod directive;
mod engine;
mod engines;
mod runtime;
mod sandbox;

pub use engine::{
    SVHostExecutionRequest, SVHostExecutorEngine, SVHostScriptExecutor, SVScriptActionInvocation,
    SVScriptCapabilityRequest, SVScriptEngine, SVScriptEngineKind, SVScriptExecution,
    SVScriptExecutionPlacement, SVScriptOutcome, SVScriptProgram, SVScriptSpan,
    SVScriptStateChange, sv_host_executor_engine,
};
pub use engines::{SVLuaScriptEngine, SVTypeScriptScriptEngine, sv_default_script_engine};
pub use runtime::{
    SV_DEFAULT_STATEMENT_BUDGET, SVScript, SVScriptBuilder, SVScriptRuntime,
    SVScriptRuntimeBuilder, SVScriptStatePolicy, sv_dispatch_outcome,
};
pub use sandbox::{SVSandboxCapability, SVSandboxPolicy};

pub use scienceview_core::sv_error::{SVError, SVScriptLocation};

/// The tracing target every script span is emitted under.
pub const SV_SCRIPT_SPAN_NAME: &str = "scienceview.script";
