use std::sync::Arc;
use std::thread;

use scienceview_core::sv_error::SVError;

use scienceview_core::sv_control::{
    SVControlAxis, SVControlOrigin, SVControlSession, SVControlState,
};
use scienceview_core::sv_scale::SVDomain;

use crate::engine::SVScriptEngineKind;
use crate::runtime::sv_dispatch_outcome;
use crate::runtime::{SVScript, SVScriptRuntime, SVScriptRuntimeBuilder};
use crate::sandbox::SVSandboxCapability;

fn runtime() -> SVScriptRuntime {
    SVScriptRuntimeBuilder::new()
        .engine(SVScriptEngineKind::Lua)
        .build()
        .expect("the Lua engine is available")
}

fn register(runtime: &SVScriptRuntime, name: &str, source: &str) {
    runtime
        .register(
            SVScript::builder(name)
                .source(source)
                .build()
                .expect("the script builds"),
        )
        .expect("the script registers");
}

#[test]
fn a_registered_script_dispatches_on_invocation() {
    let runtime = runtime();
    register(
        &runtime,
        "zoom-in",
        r#"dispatch("zoom", { axis = "x", factor = 0.8, anchor = 0.5 })"#,
    );

    let outcome = runtime.invoke("zoom-in", None).expect("the script runs");

    assert_eq!(outcome.actions().len(), 1);
    assert_eq!(outcome.statements_executed(), 1);
    assert_eq!(runtime.invocation_count("zoom-in"), Ok(1));
    assert_eq!(runtime.script_names(), Ok(vec!["zoom-in".to_string()]));
}

#[test]
fn an_unknown_script_is_a_typed_error() {
    let runtime = runtime();

    assert_eq!(
        runtime.invoke("absent", None),
        Err(SVError::ScriptNotFound {
            name: "absent".to_string(),
        })
    );
}

#[test]
fn a_duplicate_name_is_refused() {
    let runtime = runtime();
    register(&runtime, "counter", "increment(\"n\", 1)");

    let error = runtime
        .register(
            SVScript::builder("counter")
                .source("increment(\"n\", 2)")
                .build()
                .expect("the script builds"),
        )
        .expect_err("the name is taken");

    assert_eq!(
        error,
        SVError::ScriptAlreadyRegistered {
            name: "counter".to_string(),
        }
    );
}

#[test]
fn a_sandbox_refusal_does_not_terminate_the_runtime() {
    let runtime = runtime();
    register(&runtime, "reader", "read_file(\"data.csv\")");
    register(&runtime, "safe", r#"dispatch("reset")"#);

    let error = runtime
        .invoke("reader", None)
        .expect_err("the sandbox refuses");
    let SVError::ScriptSandboxViolation { capability, .. } = error else {
        panic!("expected a sandbox violation, found {error:?}");
    };
    assert_eq!(capability, "filesystem");

    // The failing script is still registered and the runtime keeps working.
    assert_eq!(runtime.script_names().expect("names").len(), 2);
    assert!(runtime.invoke("safe", None).is_ok());
    assert!(runtime.invoke("reader", None).is_err());
}

#[test]
fn widening_grants_the_capability_request() {
    let runtime = runtime();
    runtime
        .register(
            SVScript::builder("reader")
                .source("read_file(\"data.csv\")")
                .widen(SVSandboxCapability::Filesystem)
                .build()
                .expect("the script builds"),
        )
        .expect("the script registers");

    let outcome = runtime
        .invoke("reader", None)
        .expect("the capability is granted");

    assert_eq!(outcome.capability_requests().len(), 1);
    assert_eq!(
        outcome.capability_requests()[0].capability(),
        SVSandboxCapability::Filesystem
    );
    assert_eq!(outcome.capability_requests()[0].argument(), "data.csv");
}

#[test]
fn a_long_script_is_bounded_by_the_statement_budget() {
    let runtime = SVScriptRuntimeBuilder::new()
        .engine(SVScriptEngineKind::Lua)
        .statement_budget(8)
        .build()
        .expect("the runtime builds");
    let source = vec!["log(\"tick\")"; 50].join("\n");
    register(&runtime, "chatty", &source);

    let error = runtime
        .invoke("chatty", None)
        .expect_err("the budget stops it");

    assert_eq!(
        error,
        SVError::ScriptBudgetExceeded {
            script: "chatty".to_string(),
            budget: 8,
        }
    );
    // The budget applies per invocation: the next one gets the same allowance.
    assert!(runtime.invoke("chatty", None).is_err());
}

#[test]
fn a_shared_runtime_serves_concurrent_invocations() {
    let runtime = Arc::new(runtime());
    for name in ["alpha", "beta", "gamma", "delta"] {
        register(&runtime, name, "increment(\"runs\", 1)");
    }

    let mut handles = Vec::new();
    for worker in 0..8 {
        let runtime = Arc::clone(&runtime);
        handles.push(thread::spawn(move || {
            for step in 0..25 {
                let name = ["alpha", "beta", "gamma", "delta"][(worker + step) % 4];
                runtime
                    .invoke(name, None)
                    .expect("a concurrent invocation succeeds");
            }
        }));
    }
    for handle in handles {
        handle.join().expect("the worker does not panic");
    }

    for name in ["alpha", "beta", "gamma", "delta"] {
        assert_eq!(
            runtime
                .state_of(name)
                .expect("state is readable")
                .get("runs"),
            Some(&50.0),
            "script {name} saw every increment once"
        );
    }
}

#[test]
fn script_actions_drive_the_control_session() {
    let runtime = runtime();
    register(
        &runtime,
        "shrink",
        r#"dispatch("zoom", { axis = "x", factor = 0.5, anchor = 0.5 })"#,
    );
    let mut state = SVControlState::new();
    state
        .register_axis(
            SVControlAxis::Horizontal,
            SVDomain::new(0.0, 10.0).expect("a finite domain"),
        )
        .expect("the axis registers");
    let mut session = SVControlSession::new(state);

    let outcome = runtime.invoke("shrink", None).expect("the script runs");
    let applied =
        sv_dispatch_outcome(&mut session, "shrink", &outcome).expect("the action applies");

    assert_eq!(applied, 1);
    let visible = session
        .state()
        .visible_domain(SVControlAxis::Horizontal)
        .expect("the axis is registered");
    assert_eq!(visible.lower(), 2.5);
    assert_eq!(visible.upper(), 7.5);
    assert!(matches!(
        session.last_events()[0].origin(),
        SVControlOrigin::Script(name) if name == "shrink"
    ));
}
