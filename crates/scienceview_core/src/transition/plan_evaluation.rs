//! Frame evaluation: a pure function of the plan and a caller progress.
//!
//! The kernel owns no clock. The caller supplies the progress — or a duration,
//! which [`SVTransitionPlan::progress_at`] maps onto it — and the plan resolves
//! the same frame every time. A backend without in-place update has no clock, so
//! it evaluates the final state; reduced motion does the same at every progress.

use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;

use super::enter_exit::{SVExitRule, sv_apply_enter_rule, sv_apply_exit_rule};
use super::frame::{SVResolvedMark, SVTransitionFrame, SVTransitionPhase};
use super::interpolation::sv_interpolate_properties;
use super::plan::SVTransitionPlan;
use super::support::{SVTransitionSupport, sv_transition_support};
use super::transition_span::sv_run_transition_evaluation;

impl SVTransitionPlan {
    /// Evaluates the frame a caller drives at `progress`.
    ///
    /// The transition owns no clock: the caller supplies the progress. A finite
    /// progress outside `0..=1` is clamped; a non-finite one is rejected. Under
    /// reduced motion every frame is the final state.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] for a non-finite progress.
    pub fn evaluate(&self, progress: f64) -> Result<SVTransitionFrame, SVError> {
        self.evaluate_with(None, SVTransitionSupport::Animated, progress)
    }

    /// Evaluates the frame a specific backend draws at `progress`.
    ///
    /// A backend without in-place update has no clock, so the frame is the final
    /// state. The configured backend kind is recorded in the observability span.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] for a non-finite progress.
    pub fn evaluate_for_backend(
        &self,
        backend_kind: SVBackendKind,
        progress: f64,
    ) -> Result<SVTransitionFrame, SVError> {
        self.evaluate_with(
            Some(backend_kind),
            sv_transition_support(backend_kind),
            progress,
        )
    }

    fn evaluate_with(
        &self,
        backend_kind: Option<SVBackendKind>,
        support: SVTransitionSupport,
        progress: f64,
    ) -> Result<SVTransitionFrame, SVError> {
        let number_of_marks = self.updates.len() + self.enters.len() + self.exits.len();

        sv_run_transition_evaluation(backend_kind, number_of_marks, || {
            if !progress.is_finite() {
                return Err(SVError::InvalidConfiguration {
                    message: format!("a transition progress must be finite, received {progress}"),
                });
            }

            let effective_progress = if self.motion.is_reduced_motion() || !support.is_animated() {
                1.0
            } else {
                progress.clamp(0.0, 1.0)
            };
            let eased_progress = self.motion.easing().evaluate(effective_progress);

            let mut entries = Vec::with_capacity(number_of_marks);
            for update in &self.updates {
                entries.push(SVResolvedMark::new(
                    update.key,
                    SVTransitionPhase::Update,
                    sv_interpolate_properties(
                        &update.from,
                        &update.to,
                        eased_progress,
                        effective_progress,
                        self.interpolate,
                    ),
                ));
            }
            for enter in &self.enters {
                entries.push(SVResolvedMark::new(
                    enter.key,
                    SVTransitionPhase::Enter,
                    sv_apply_enter_rule(self.enter_rule, &enter.target, eased_progress),
                ));
            }
            for exit in &self.exits {
                if self.exit_rule != SVExitRule::None && effective_progress < 1.0 {
                    entries.push(SVResolvedMark::new(
                        exit.key,
                        SVTransitionPhase::Exit,
                        sv_apply_exit_rule(self.exit_rule, &exit.source, eased_progress),
                    ));
                }
            }

            Ok(SVTransitionFrame::new(entries))
        })
    }
}
