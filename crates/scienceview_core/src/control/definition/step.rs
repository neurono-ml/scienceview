//! Step validation shared by the builder and the runtime mutator.

pub(crate) fn sv_validated_step(step: f64) -> Result<f64, crate::sv_error::SVError> {
    if step.is_finite() && step > 0.0 {
        return Ok(step);
    }
    Err(crate::sv_error::SVError::InvalidConfiguration {
        message: format!("a control step must be finite and greater than zero, received {step}"),
    })
}
