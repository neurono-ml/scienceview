//! Swarm: a collision-free deterministic layout.

use crate::sv_error::SVError;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVMoveRequest, SVSwarmOptions};

/// Applies a swarm layout to the working state.
///
/// This is the identity for now; the task's tests drive the implementation.
pub(crate) fn sv_apply_swarm(
    _state: &mut SVMoveState,
    _request: &SVMoveRequest<'_>,
    _options: &SVSwarmOptions,
) -> Result<(), SVError> {
    Ok(())
}
