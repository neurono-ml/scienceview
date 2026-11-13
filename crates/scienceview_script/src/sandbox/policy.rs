//! The capability policy one script runs under.

use std::collections::BTreeSet;

use scienceview_core::sv_error::SVError;

use super::SVSandboxCapability;

/// The exact set of capabilities one script holds.
///
/// A policy starts restricted — [`SVSandboxCapability::Graphics`] only — and
/// widening is additive and explicit. There is no "allow everything" switch, so
/// a script is granted *exactly* what its author asked for and nothing else.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SVSandboxPolicy {
    granted: BTreeSet<SVSandboxCapability>,
}

impl SVSandboxPolicy {
    /// Returns the restricted default: figure access only.
    pub fn restricted() -> Self {
        let mut granted = BTreeSet::new();
        granted.insert(SVSandboxCapability::Graphics);
        Self { granted }
    }

    /// Grants one capability, in addition to the ones already held.
    #[must_use]
    pub fn widen(mut self, capability: SVSandboxCapability) -> Self {
        self.granted.insert(capability);
        self
    }

    /// Grants one capability in place.
    pub fn grant(&mut self, capability: SVSandboxCapability) {
        self.granted.insert(capability);
    }

    /// Returns whether the script holds a capability.
    ///
    /// Figure access is the reason a script exists, so it is always held even
    /// when a policy was assembled from an empty set.
    pub fn grants(&self, capability: SVSandboxCapability) -> bool {
        capability == SVSandboxCapability::Graphics || self.granted.contains(&capability)
    }

    /// Returns the capabilities held, in deterministic order.
    pub fn granted(&self) -> impl Iterator<Item = SVSandboxCapability> + '_ {
        self.granted.iter().copied()
    }

    /// Returns `true` when the script holds nothing beyond figure access.
    pub fn is_restricted(&self) -> bool {
        self.granted
            .iter()
            .all(|capability| *capability == SVSandboxCapability::Graphics)
    }

    /// Refuses a capability the policy does not grant.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ScriptSandboxViolation`] when the capability is not
    /// held.
    pub fn require(
        &self,
        capability: SVSandboxCapability,
        script: &str,
        location: scienceview_core::sv_error::SVScriptLocation,
    ) -> Result<(), SVError> {
        if self.grants(capability) {
            return Ok(());
        }
        Err(SVError::ScriptSandboxViolation {
            script: script.to_string(),
            location,
            capability: capability.to_string(),
        })
    }
}
