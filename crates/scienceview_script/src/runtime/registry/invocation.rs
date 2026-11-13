//! One contained invocation, inside its observability span.

use scienceview_core::sv_control::SVControlEvent;
use scienceview_core::sv_error::SVError;
use tracing::Span;

use super::super::SVScriptRuntime;
use crate::engine::{SVScriptExecution, SVScriptOutcome};

impl SVScriptRuntime {
    /// Runs one invocation of a registered script.
    ///
    /// A failure is returned to the caller with the script identity and a
    /// source location; the script stays registered and the next invocation
    /// runs normally.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ScriptNotFound`] for an unknown name and propagates
    /// every engine failure.
    pub fn invoke(
        &self,
        name: &str,
        event: Option<&SVControlEvent>,
    ) -> Result<SVScriptOutcome, SVError> {
        let span = tracing::info_span!(
            target: "scienceview::script",
            "scienceview.script",
            script = name,
            engine = %self.engine.kind(),
            placement = %self.placement,
            statements = tracing::field::Empty,
            actions = tracing::field::Empty,
            outcome = tracing::field::Empty,
        );
        let _entered = span.enter();

        let mut scripts = self.sv_scripts()?;
        let registration = scripts
            .get_mut(name)
            .ok_or_else(|| SVError::ScriptNotFound {
                name: name.to_string(),
            })?;

        let result = {
            let mut execution = SVScriptExecution::new(
                name,
                event,
                registration.script.sandbox(),
                &mut registration.state,
                self.statement_budget,
            );
            self.engine.execute(&registration.program, &mut execution)
        };

        match result {
            Ok(outcome) => {
                registration.invocations += 1;
                sv_record_span(&span, &outcome);
                Ok(outcome)
            }
            Err(error) => {
                span.record("outcome", tracing::field::display(&error));
                Err(error)
            }
        }
    }
}

fn sv_record_span(span: &Span, outcome: &SVScriptOutcome) {
    span.record("statements", outcome.statements_executed());
    span.record("actions", outcome.actions().len());
    span.record("outcome", "success");
}
