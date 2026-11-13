use scienceview_core::sv_error::SVError;

use crate::engine::SVScriptEngineKind;
use crate::runtime::{SVScript, SVScriptRuntime, SVScriptRuntimeBuilder, SVScriptStatePolicy};

fn runtime() -> SVScriptRuntime {
    SVScriptRuntimeBuilder::new()
        .engine(SVScriptEngineKind::Lua)
        .build()
        .expect("the Lua engine is available")
}

fn register(runtime: &SVScriptRuntime, source: &str) {
    runtime
        .register(
            SVScript::builder("counter")
                .source(source)
                .build()
                .expect("the script builds"),
        )
        .expect("the script registers");
}

#[test]
fn reload_preserves_state_by_default_and_can_reset_it() {
    let runtime = runtime();
    register(&runtime, "increment(\"clicks\", 1)");
    runtime.invoke("counter", None).expect("first run");
    runtime.invoke("counter", None).expect("second run");
    assert_eq!(
        runtime.state_of("counter").expect("state").get("clicks"),
        Some(&2.0)
    );

    runtime
        .replace(
            "counter",
            "increment(\"clicks\", 10)",
            SVScriptStatePolicy::Preserve,
        )
        .expect("the replacement loads");
    runtime.invoke("counter", None).expect("third run");
    assert_eq!(
        runtime.state_of("counter").expect("state").get("clicks"),
        Some(&12.0)
    );

    runtime
        .replace(
            "counter",
            "increment(\"clicks\", 10)",
            SVScriptStatePolicy::Reset,
        )
        .expect("the replacement loads");
    runtime.invoke("counter", None).expect("fourth run");
    assert_eq!(
        runtime.state_of("counter").expect("state").get("clicks"),
        Some(&10.0)
    );
}

#[test]
fn reloading_reprepares_the_current_source() {
    let runtime = runtime();
    register(&runtime, r#"dispatch("reset")"#);
    let source = runtime.source_of("counter").expect("source");

    runtime
        .reload("counter", SVScriptStatePolicy::Preserve)
        .expect("the same source prepares again");

    assert_eq!(runtime.source_of("counter").expect("source"), source);
    assert_eq!(
        runtime
            .invoke("counter", None)
            .expect("the script runs")
            .actions()
            .len(),
        1
    );
}

#[test]
fn a_replacement_that_fails_to_parse_leaves_the_old_program_running() {
    let runtime = runtime();
    register(&runtime, r#"dispatch("reset")"#);

    let error = runtime
        .replace(
            "counter",
            r#"dispatch("bogus")"#,
            SVScriptStatePolicy::Preserve,
        )
        .expect_err("the unknown action is rejected");

    assert!(matches!(error, SVError::ScriptUnknownAction { .. }));
    assert_eq!(
        runtime.source_of("counter").expect("source"),
        r#"dispatch("reset")"#
    );
    let outcome = runtime
        .invoke("counter", None)
        .expect("the old program still runs");
    assert_eq!(outcome.actions().len(), 1);
}

#[test]
fn reloading_an_unknown_script_is_a_typed_error() {
    let runtime = runtime();

    assert_eq!(
        runtime.replace("absent", "log(\"hi\")", SVScriptStatePolicy::Preserve),
        Err(SVError::ScriptNotFound {
            name: "absent".to_string(),
        })
    );
    assert_eq!(
        runtime.reload("absent", SVScriptStatePolicy::Preserve),
        Err(SVError::ScriptNotFound {
            name: "absent".to_string(),
        })
    );
}

#[test]
fn unregistering_removes_the_script_and_returns_it() {
    let runtime = runtime();
    register(&runtime, "log(\"hi\")");

    let script = runtime
        .unregister("counter")
        .expect("the script is removed");

    assert_eq!(script.name(), "counter");
    assert_eq!(runtime.script_names(), Ok(Vec::new()));
    assert_eq!(
        runtime.invoke("counter", None),
        Err(SVError::ScriptNotFound {
            name: "counter".to_string(),
        })
    );
}
