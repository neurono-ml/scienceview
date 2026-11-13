//! The engine trait.

use scienceview_core::sv_error::SVError;

use crate::engine::{SVScriptEngineKind, SVScriptExecution, SVScriptOutcome, SVScriptProgram};

/// One script language implementation.
///
/// Preparing a source is separate from executing it: preparation is where a
/// parse failure belongs, and it happens once, so a replacement that does not
/// parse can be rejected without disturbing the running program.
pub trait SVScriptEngine: Send + Sync {
    /// Returns the engine kind this instance implements.
    fn kind(&self) -> SVScriptEngineKind;

    /// Prepares a source, or reports where it could not be prepared.
    ///
    /// # Errors
    ///
    /// Any [`SVError`] variant the engine itself uses, carrying the script name
    /// and the source location of the failure.
    fn prepare(&self, script_name: &str, source: &str) -> Result<SVScriptProgram, SVError>;

    /// Executes one prepared program.
    ///
    /// # Errors
    ///
    /// Returns a typed [`SVError`]; the runtime never aborts the host.
    fn execute(
        &self,
        program: &SVScriptProgram,
        execution: &mut SVScriptExecution<'_>,
    ) -> Result<SVScriptOutcome, SVError>;
}
