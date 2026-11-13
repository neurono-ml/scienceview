//! What one script invocation produced.

use scienceview_core::sv_control::SVControlAction;
use scienceview_core::sv_error::SVScriptLocation;

use crate::engine::SVScriptSpan;
use crate::sandbox::SVSandboxCapability;

/// One semantic action a script dispatched, with where it was written.
#[derive(Clone, Debug, PartialEq)]
pub struct SVScriptActionInvocation {
    action: SVControlAction,
    location: SVScriptLocation,
}

impl SVScriptActionInvocation {
    /// Pairs an action with the location that produced it.
    pub fn new(action: SVControlAction, location: SVScriptLocation) -> Self {
        Self { action, location }
    }

    /// Returns the semantic action.
    pub fn action(&self) -> &SVControlAction {
        &self.action
    }

    /// Returns where in the script the action was dispatched.
    pub fn location(&self) -> SVScriptLocation {
        self.location
    }

    /// Returns the byte span of the dispatched action.
    pub fn span(&self) -> SVScriptSpan {
        self.location.into()
    }
}

/// One change a script made to its own persistent state.
#[derive(Clone, Debug, PartialEq)]
pub struct SVScriptStateChange {
    name: String,
    previous: f64,
    current: f64,
    location: SVScriptLocation,
}

impl SVScriptStateChange {
    /// Records one state transition.
    pub fn new(name: String, previous: f64, current: f64, location: SVScriptLocation) -> Self {
        Self {
            name,
            previous,
            current,
            location,
        }
    }

    /// Returns the variable name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the value before the invocation.
    pub fn previous(&self) -> f64 {
        self.previous
    }

    /// Returns the value after the invocation.
    pub fn current(&self) -> f64 {
        self.current
    }

    /// Returns where the change was written.
    pub fn location(&self) -> SVScriptLocation {
        self.location
    }

    /// Returns the byte span of the change.
    pub fn span(&self) -> SVScriptSpan {
        self.location.into()
    }
}

/// One system capability a script asked its host to perform.
///
/// The built-in interpreter performs no I/O: it reports the request here, so a
/// host adapter can service it with exactly the capabilities the sandbox
/// granted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVScriptCapabilityRequest {
    capability: SVSandboxCapability,
    argument: String,
    location: SVScriptLocation,
}

impl SVScriptCapabilityRequest {
    /// Records one capability request.
    pub fn new(
        capability: SVSandboxCapability,
        argument: String,
        location: SVScriptLocation,
    ) -> Self {
        Self {
            capability,
            argument,
            location,
        }
    }

    /// Returns the requested capability.
    pub fn capability(&self) -> SVSandboxCapability {
        self.capability
    }

    /// Returns the argument the script passed (a path, a URL, a command).
    pub fn argument(&self) -> &str {
        &self.argument
    }

    /// Returns where the request was written.
    pub fn location(&self) -> SVScriptLocation {
        self.location
    }

    /// Returns the byte span of the request.
    pub fn span(&self) -> SVScriptSpan {
        self.location.into()
    }
}

/// Everything one invocation produced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVScriptOutcome {
    actions: Vec<SVScriptActionInvocation>,
    state_changes: Vec<SVScriptStateChange>,
    capability_requests: Vec<SVScriptCapabilityRequest>,
    logs: Vec<String>,
    statements_executed: usize,
}

impl SVScriptOutcome {
    /// Returns an empty outcome.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Records one dispatched action.
    pub fn push_action(&mut self, action: SVControlAction, location: SVScriptLocation) {
        self.actions
            .push(SVScriptActionInvocation::new(action, location));
    }

    /// Records one persistent-state change.
    pub fn push_state_change(&mut self, change: SVScriptStateChange) {
        self.state_changes.push(change);
    }

    /// Records one system capability request.
    pub fn push_capability_request(&mut self, request: SVScriptCapabilityRequest) {
        self.capability_requests.push(request);
    }

    /// Records one log line the script emitted.
    pub fn push_log(&mut self, message: String) {
        self.logs.push(message);
    }

    /// Sets how many statements the invocation ran.
    pub fn set_statements_executed(&mut self, count: usize) {
        self.statements_executed = count;
    }

    /// Returns the actions, in script order.
    pub fn actions(&self) -> &[SVScriptActionInvocation] {
        &self.actions
    }

    /// Returns the persistent-state changes, in script order.
    pub fn state_changes(&self) -> &[SVScriptStateChange] {
        &self.state_changes
    }

    /// Returns the capability requests the sandbox granted, in script order.
    pub fn capability_requests(&self) -> &[SVScriptCapabilityRequest] {
        &self.capability_requests
    }

    /// Returns the log lines, in script order.
    pub fn logs(&self) -> &[String] {
        &self.logs
    }

    /// Returns how many statements the invocation ran.
    pub fn statements_executed(&self) -> usize {
        self.statements_executed
    }
}
