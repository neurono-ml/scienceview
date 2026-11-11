//! The validated sequence: the only public way to compose moves.

use crate::sv_error::SVError;
use crate::sv_move::dodge::sv_apply_dodge;
use crate::sv_move::jitter::sv_apply_jitter;
use crate::sv_move::move_span::sv_run_move_sequence;
use crate::sv_move::normalize::sv_apply_normalize;
use crate::sv_move::shift::sv_apply_shift;
use crate::sv_move::stack::sv_apply_stack;
use crate::sv_move::swarm::sv_apply_swarm;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVMove, SVMoveOutput, SVMoveRequest, SVMoveStage};

/// Several moves that apply in the fixed composition order.
///
/// The order is enforced here, once, at construction (design D2): the
/// displacement stage — stack, dodge, jitter or swarm, at most one — then
/// shift, then normalize. Every stage is optional, so an empty sequence is the
/// identity and a single move is the common case.
///
/// ```
/// use scienceview_core::sv_move::{SVMove, SVMoveSequence};
///
/// let sequence = SVMoveSequence::new(vec![
///     SVMove::dodge().into(),
///     SVMove::normalize().into(),
/// ])
/// .expect("dodge then normalize is the documented order");
///
/// assert_eq!(sequence.number_of_moves(), 2);
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVMoveSequence {
    moves: Vec<SVMove>,
}

impl SVMoveSequence {
    /// Builds a sequence, rejecting an invalid composition order.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MoveOrderViolation`] when a move occupies an earlier
    /// stage than its predecessor, [`SVError::ConflictingMoveStage`] when two
    /// moves claim the same stage, and [`SVError::InvalidConfiguration`] when a
    /// move's options are outside their range.
    pub fn new(moves: Vec<SVMove>) -> Result<Self, SVError> {
        let mut preceding_stage: Option<SVMoveStage> = None;
        for (position, single_move) in moves.iter().enumerate() {
            single_move.validate()?;
            let stage = single_move.stage();
            if let Some(preceding) = preceding_stage {
                if stage < preceding {
                    return Err(SVError::MoveOrderViolation {
                        position,
                        preceding_stage: preceding,
                        found_stage: stage,
                    });
                }
                if stage == preceding {
                    return Err(SVError::ConflictingMoveStage { stage });
                }
            }
            preceding_stage = Some(stage);
        }
        Ok(Self { moves })
    }

    /// Builds a sequence of one move.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::new`].
    pub fn single(single_move: SVMove) -> Result<Self, SVError> {
        Self::new(vec![single_move])
    }

    /// Builds the identity sequence.
    pub fn empty() -> Self {
        Self { moves: Vec::new() }
    }

    /// Returns the moves in application order.
    pub fn moves(&self) -> &[SVMove] {
        &self.moves
    }

    /// Returns the number of moves.
    pub fn number_of_moves(&self) -> usize {
        self.moves.len()
    }

    /// Applies every move to the request, in the validated order.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MismatchedLengths`] when a key column does not match
    /// the values, and [`SVError::InvalidConfiguration`] when a move cannot
    /// express its options on this data.
    pub fn apply(&self, request: &SVMoveRequest<'_>) -> Result<SVMoveOutput, SVError> {
        request.validate()?;
        let representation = request.values().representation();
        sv_run_move_sequence(request, self.moves.len(), || {
            let mut state = SVMoveState::from_request(request)?;
            for single_move in &self.moves {
                match single_move {
                    SVMove::Stack(options) => sv_apply_stack(&mut state, request, options)?,
                    SVMove::Dodge(options) => sv_apply_dodge(&mut state, request, options)?,
                    SVMove::Jitter(options) => sv_apply_jitter(&mut state, request, options)?,
                    SVMove::Shift(options) => sv_apply_shift(&mut state, options)?,
                    SVMove::Swarm(options) => sv_apply_swarm(&mut state, request, options)?,
                    SVMove::Normalize(options) => sv_apply_normalize(&mut state, request, options)?,
                }
            }
            Ok(state.into_output(representation))
        })
    }
}
