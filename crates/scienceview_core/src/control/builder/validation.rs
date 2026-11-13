//! Validation: the configuration becomes a control only when it is consistent.

use crate::sv_error::SVError;

use super::definition::SVControlBuilder;
use crate::sv_control::definition::{SVControl, SVControlConfiguration, sv_validated_step};
use crate::sv_control::{SVControlKind, SVControlSize, SVPlaybackCommand};

impl SVControlBuilder {
    /// Validates the configuration and produces the control.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] for an empty label or name,
    /// [`SVError::InvalidControlSize`] for a size the builder would reject and
    /// [`SVError::InvalidPlaybackSpeed`] for a non-positive speed.
    pub fn build(self) -> Result<SVControl, SVError> {
        let name = self.name.unwrap_or_else(|| self.kind.default_name());
        if name.trim().is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: "a control needs a non-empty name".to_string(),
            });
        }
        let label = self.label.unwrap_or_else(|| self.kind.default_label());
        if label.trim().is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: "a control needs a non-empty label".to_string(),
            });
        }
        let (width, height) = self.kind.default_size();
        let size = match self.size {
            Some(size) => size,
            None => SVControlSize::new(width, height)?,
        };
        let step = match self.kind {
            SVControlKind::ZoomIn | SVControlKind::ZoomOut | SVControlKind::Pan => {
                sv_validated_step(self.step)?
            }
            _ => self.step,
        };
        if let SVControlKind::Playback(SVPlaybackCommand::Speed(speed)) = &self.kind {
            if !speed.is_finite() || *speed <= 0.0 {
                return Err(SVError::InvalidPlaybackSpeed);
            }
        }
        let range = match self.bounds {
            Some(bounds) => self.range.clamp_into(bounds),
            None => self.range,
        };

        Ok(SVControl::sv_from_configuration(SVControlConfiguration {
            kind: self.kind,
            name,
            label,
            size,
            placement: self.placement,
            style: self.style,
            enabled: self.enabled,
            target_axis: self.target_axis,
            step,
            bounds: self.bounds,
            range,
            region: self.region,
            action_override: self.action_override,
            script: self.script,
        }))
    }
}

impl Default for SVControlBuilder {
    fn default() -> Self {
        Self::reset()
    }
}
