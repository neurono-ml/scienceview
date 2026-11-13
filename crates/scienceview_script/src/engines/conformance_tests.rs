use std::collections::BTreeMap;
use std::sync::Arc;

use scienceview_core::sv_control::{SVControlAction, SVControlAxis};
use scienceview_core::sv_error::{SVError, SVScriptLocation};

use crate::engine::{
    SVHostExecutionRequest, SVHostScriptExecutor, SVScriptEngine, SVScriptEngineKind,
    SVScriptExecution, SVScriptOutcome, sv_host_executor_engine,
};
use crate::sandbox::SVSandboxPolicy;

use super::{SVLuaScriptEngine, SVTypeScriptScriptEngine};

fn execute(
    engine: &dyn SVScriptEngine,
    source: &str,
    state: &mut BTreeMap<String, f64>,
) -> Result<SVScriptOutcome, SVError> {
    let program = engine.prepare("conformance", source)?;
    let sandbox = SVSandboxPolicy::restricted();
    let mut execution = SVScriptExecution::new("conformance", None, &sandbox, state, 100);
    engine.execute(&program, &mut execution)
}

#[test]
fn both_engines_dispatch_the_same_action() {
    let lua = SVLuaScriptEngine::new();
    let type_script = SVTypeScriptScriptEngine::new();
    let mut lua_state = BTreeMap::new();
    let mut type_script_state = BTreeMap::new();

    let lua_outcome = execute(
        &lua,
        r#"dispatch("zoom", { axis = "y", factor = 2.0, anchor = 1.0 })"#,
        &mut lua_state,
    )
    .expect("the Lua script runs");
    let type_script_outcome = execute(
        &type_script,
        "dispatch(\"zoom\", { axis: \"y\", factor: 2.0, anchor: 1.0 });",
        &mut type_script_state,
    )
    .expect("the TypeScript script runs");

    let expected = SVControlAction::Zoom {
        axis: SVControlAxis::Vertical,
        factor: 2.0,
        anchor: 1.0,
    };
    assert_eq!(lua_outcome.actions()[0].action(), &expected);
    assert_eq!(type_script_outcome.actions()[0].action(), &expected);
}

#[test]
fn state_survives_across_invocations() {
    let engine = SVLuaScriptEngine::new();
    let mut state = BTreeMap::new();

    execute(&engine, "increment(\"hits\", 2)", &mut state).expect("first run");
    execute(&engine, "increment(\"hits\", 3)", &mut state).expect("second run");

    assert_eq!(state.get("hits"), Some(&5.0));
}

#[test]
fn an_engine_rejects_a_program_another_engine_prepared() {
    let lua = SVLuaScriptEngine::new();
    let type_script = SVTypeScriptScriptEngine::new();
    let program = lua
        .prepare("conformance", r#"dispatch("reset")"#)
        .expect("the Lua program prepares");
    let sandbox = SVSandboxPolicy::restricted();
    let mut state = BTreeMap::new();
    let mut execution = SVScriptExecution::new("conformance", None, &sandbox, &mut state, 10);

    let error = type_script
        .execute(&program, &mut execution)
        .expect_err("the kinds must agree");

    assert_eq!(
        error,
        SVError::ScriptEngineMismatch {
            configured: "typescript".to_string(),
            prepared: "lua".to_string(),
        }
    );
}

struct RepeatingExecutor;

impl SVHostScriptExecutor for RepeatingExecutor {
    fn kind(&self) -> SVScriptEngineKind {
        SVScriptEngineKind::TypeScript
    }

    fn execute(&self, request: &SVHostExecutionRequest<'_>) -> Result<SVScriptOutcome, SVError> {
        let mut outcome = SVScriptOutcome::empty();
        outcome.push_action(
            SVControlAction::Custom {
                name: "hosted".to_string(),
                payload: request.source().to_string(),
            },
            SVScriptLocation::new(1, 1, 0, request.source().len()),
        );
        outcome.set_statements_executed(1);
        Ok(outcome)
    }
}

#[test]
fn a_third_party_host_executor_plugs_in_through_the_adapter() {
    let engine = sv_host_executor_engine(Arc::new(RepeatingExecutor));
    let mut state = BTreeMap::new();

    let outcome = execute(&engine, "anything", &mut state).expect("the adapter delegates");

    assert_eq!(engine.kind(), SVScriptEngineKind::TypeScript);
    assert_eq!(
        outcome.actions()[0].action(),
        &SVControlAction::Custom {
            name: "hosted".to_string(),
            payload: "anything".to_string(),
        }
    );
}
