//! The contract every framework wrapper implements, with React as its
//! instance.
//!
//! The three backend crates repeat this type and these values. A change to the
//! contract therefore changes all three together, which is what makes porting
//! an application a wrapper-only edit.

use scienceview_core::sv_backend::SVBackendKind;

/// The properties, events and actions of a framework figure component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SVReactComponentContract {
    /// The framework whose wrapper implements the contract.
    pub framework: &'static str,
    /// The accepted properties, in documented order.
    pub properties: [&'static str; 4],
    /// The events the component emits.
    pub events: [&'static str; 1],
    /// The action types forwarded through the event.
    pub actions: [&'static str; 2],
    /// The backend kind that produces this wrapper.
    pub backend_kind: SVBackendKind,
}

/// The shared contract as the React wrapper implements it.
pub const SV_REACT_COMPONENT_CONTRACT: SVReactComponentContract = SVReactComponentContract {
    framework: "react",
    properties: ["data", "markup", "theme", "title"],
    events: ["scienceview-action"],
    actions: ["select", "zoom"],
    backend_kind: SVBackendKind::React,
};
