use scienceview_core::sv_error::{SVError, SVScriptLocation};

use super::{SVSandboxCapability, SVSandboxPolicy};

fn location() -> SVScriptLocation {
    SVScriptLocation::new(1, 1, 0, 1)
}

#[test]
fn restricted_policy_grants_figure_access_only() {
    let policy = SVSandboxPolicy::restricted();

    assert!(policy.grants(SVSandboxCapability::Graphics));
    for capability in [
        SVSandboxCapability::Filesystem,
        SVSandboxCapability::Network,
        SVSandboxCapability::Process,
        SVSandboxCapability::Environment,
    ] {
        assert!(!policy.grants(capability));
        assert_eq!(
            policy.require(capability, "budget", location()),
            Err(SVError::ScriptSandboxViolation {
                script: "budget".to_string(),
                location: location(),
                capability: capability.to_string(),
            })
        );
    }
    assert!(policy.is_restricted());
}

#[test]
fn widening_grants_exactly_the_requested_capability() {
    let policy = SVSandboxPolicy::restricted().widen(SVSandboxCapability::Filesystem);

    assert!(policy.grants(SVSandboxCapability::Filesystem));
    assert!(!policy.grants(SVSandboxCapability::Network));
    assert!(
        policy
            .require(SVSandboxCapability::Filesystem, "loader", location())
            .is_ok()
    );
    assert!(!policy.is_restricted());
    assert_eq!(
        policy.granted().collect::<Vec<_>>(),
        vec![
            SVSandboxCapability::Graphics,
            SVSandboxCapability::Filesystem
        ]
    );
}

#[test]
fn figure_access_survives_a_policy_built_without_it() {
    let policy = SVSandboxPolicy::default();

    assert!(policy.grants(SVSandboxCapability::Graphics));
    assert!(policy.is_restricted());
}
