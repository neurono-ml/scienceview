//! Applying a script's dispatched actions to the control session.

use scienceview_core::sv_control::{SVControlOrigin, SVControlSession};
use scienceview_core::sv_error::SVError;

use crate::engine::SVScriptOutcome;

/// Applies every action a script produced to one control session.
///
/// The actions are dispatched with [`SVControlOrigin::Script`], so every
/// subscriber sees the script as the origin and the session applies exactly the
/// reducer rules it applies to a built-in control. Returns how many actions were
/// applied; the first failing action stops the sequence and is returned, leaving
/// the earlier ones applied (each dispatch is atomic on its own).
///
/// # Errors
///
/// Propagates the reducer and timeline errors of
/// [`SVControlSession::dispatch_from`](scienceview_core::sv_control::SVControlSession::dispatch_from).
pub fn sv_dispatch_outcome(
    session: &mut SVControlSession,
    script_name: &str,
    outcome: &SVScriptOutcome,
) -> Result<usize, SVError> {
    let origin = SVControlOrigin::Script(script_name.to_string());
    let mut applied = 0;
    for invocation in outcome.actions() {
        session.dispatch_from(origin.clone(), invocation.action())?;
        applied += 1;
    }
    Ok(applied)
}
