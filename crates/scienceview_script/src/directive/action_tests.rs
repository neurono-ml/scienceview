use scienceview_core::sv_animation::SVAnimationAction;
use scienceview_core::sv_control::{SVControlAction, SVControlAxis, SVSeriesReference};
use scienceview_core::sv_error::SVError;

use crate::sandbox::SVSandboxCapability;

use super::{SVScriptDialect, SVScriptStatement, sv_build_statements, sv_parse_calls};

fn lua(source: &str) -> Vec<SVScriptStatement> {
    sv_build_statements(
        "figure-script",
        sv_parse_calls(source, SVScriptDialect::Lua).expect("the source parses"),
    )
    .expect("the statements resolve")
}

fn lua_error(source: &str) -> SVError {
    sv_build_statements(
        "figure-script",
        sv_parse_calls(source, SVScriptDialect::Lua).expect("the source parses"),
    )
    .expect_err("the statement must be rejected")
}

fn dispatch(statement: &SVScriptStatement) -> &SVControlAction {
    match statement {
        SVScriptStatement::Dispatch { action, .. } => action,
        other => panic!("expected a dispatch, found {other:?}"),
    }
}

#[test]
fn zoom_dispatches_through_the_control_vocabulary() {
    let statements = lua(r#"dispatch("zoom", { axis = "x", factor = 0.8, anchor = 0.5 })"#);

    assert_eq!(
        dispatch(&statements[0]),
        &SVControlAction::Zoom {
            axis: SVControlAxis::Horizontal,
            factor: 0.8,
            anchor: 0.5,
        }
    );
}

#[test]
fn range_and_series_actions_carry_typed_payloads() {
    let statements = lua(
        r#"dispatch("set_range", { axis = "y", minimum = -1.0, maximum = 1.0 })
dispatch("set_series_visibility", { series = "temperature", visible = false })
dispatch("set_playback", { command = "step_forward" })"#,
    );

    match dispatch(&statements[0]) {
        SVControlAction::SetRange { axis, range } => {
            assert_eq!(*axis, SVControlAxis::Vertical);
            assert_eq!(range.lower(), -1.0);
            assert_eq!(range.upper(), 1.0);
        }
        other => panic!("expected a range action, found {other:?}"),
    }
    assert_eq!(
        dispatch(&statements[1]),
        &SVControlAction::SetSeriesVisibility {
            series: SVSeriesReference::Name("temperature".to_string()),
            visible: false,
        }
    );
    assert_eq!(
        dispatch(&statements[2]),
        &SVControlAction::SetPlayback(SVAnimationAction::Step { forward: true })
    );
}

#[test]
fn an_unknown_action_is_a_typed_error_with_its_span() {
    let error = lua_error(r#"dispatch("explode", { force = 1.0 })"#);

    let SVError::ScriptUnknownAction {
        script,
        location,
        action,
    } = error
    else {
        panic!("expected an unknown-action error, found {error:?}");
    };
    assert_eq!(script, "figure-script");
    assert_eq!(action, "explode");
    assert_eq!(location.line, 1);
    assert!(location.end > location.start);
}

#[test]
fn an_unknown_field_is_rejected_by_the_action() {
    let error =
        lua_error(r#"dispatch("zoom", { axis = "x", factor = 0.8, anchor = 0.5, depth = 2 })"#);

    let SVError::ScriptRuntimeFailure { message, .. } = error else {
        panic!("expected a runtime failure, found {error:?}");
    };
    assert!(
        message.contains("does not accept the field `depth`"),
        "{message}"
    );
}

#[test]
fn an_inverted_range_is_rejected() {
    let error = lua_error(r#"dispatch("set_range", { axis = "x", minimum = 5.0, maximum = 1.0 })"#);

    let SVError::ScriptRuntimeFailure { message, .. } = error else {
        panic!("expected a runtime failure, found {error:?}");
    };
    assert!(message.contains("invalid range"), "{message}");
}

#[test]
fn statements_resolve_state_logs_and_capabilities() {
    let statements = lua("increment(\"clicks\", 1)\nlog(\"hi\")\nread_file(\"data.csv\")");

    assert!(matches!(
        statements[0],
        SVScriptStatement::Increment { ref name, amount, .. } if name == "clicks" && amount == 1.0
    ));
    assert!(matches!(
        statements[1],
        SVScriptStatement::Log { ref message, .. } if message == "hi"
    ));
    assert!(matches!(
        statements[2],
        SVScriptStatement::Capability { capability: SVSandboxCapability::Filesystem, ref argument, .. }
            if argument == "data.csv"
    ));
}

#[test]
fn an_unknown_statement_names_the_supported_set() {
    let error = lua_error("borrow_the_moon()");

    let SVError::ScriptRuntimeFailure { message, .. } = error else {
        panic!("expected a runtime failure, found {error:?}");
    };
    assert!(
        message.contains("unknown statement `borrow_the_moon`"),
        "{message}"
    );
    assert!(message.contains("dispatch"), "{message}");
}
