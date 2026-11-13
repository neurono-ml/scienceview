use super::{SVScriptDialect, sv_parse_calls};
use crate::directive::parser::SVScriptValue;

#[test]
fn lua_reads_a_dispatch_call_with_a_table() {
    let calls = sv_parse_calls(
        r#"dispatch("zoom", { axis = "x", factor = 0.8, anchor = 0.5 })"#,
        SVScriptDialect::Lua,
    )
    .expect("the statement parses");

    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "dispatch");
    assert_eq!(calls[0].positional.len(), 1);
    assert_eq!(
        calls[0].positional[0].value,
        SVScriptValue::String("zoom".to_string())
    );
    assert_eq!(calls[0].fields.len(), 3);
    assert_eq!(calls[0].fields[1].name, "factor");
    assert_eq!(calls[0].fields[1].value.value, SVScriptValue::Number(0.8));
    assert!(calls[0].location.end > calls[0].location.start);
}

#[test]
fn typescript_reads_a_dispatch_call_with_an_object_literal() {
    let calls = sv_parse_calls(
        "dispatch(\"pan\", { axis: \"y\", fraction: -0.25 }); // shift down",
        SVScriptDialect::TypeScript,
    )
    .expect("the statement parses");

    assert_eq!(calls.len(), 1);
    assert_eq!(
        calls[0].positional[0].value,
        SVScriptValue::String("pan".to_string())
    );
    assert_eq!(calls[0].fields[1].value.value, SVScriptValue::Number(-0.25));
}

#[test]
fn each_dialect_rejects_the_other_field_separator() {
    let lua_error = sv_parse_calls(r#"dispatch("zoom", { axis: "x" })"#, SVScriptDialect::Lua)
        .expect_err("Lua tables use `=`");
    assert!(lua_error.message.contains("`=`"), "{}", lua_error.message);

    let type_script_error = sv_parse_calls(
        r#"dispatch("zoom", { axis = "x" })"#,
        SVScriptDialect::TypeScript,
    )
    .expect_err("TypeScript objects use `:`");
    assert!(
        type_script_error.message.contains("`:`"),
        "{}",
        type_script_error.message
    );
}

#[test]
fn comments_are_skipped_in_both_dialects() {
    let lua = sv_parse_calls(
        "-- a comment\ndispatch(\"reset\")\n-- another\nlog(\"done\")",
        SVScriptDialect::Lua,
    )
    .expect("comments are skipped");
    assert_eq!(lua.len(), 2);

    let type_script = sv_parse_calls(
        "// a comment\n/* block\n comment */\ndispatch(\"reset\");\nlog(\"done\");",
        SVScriptDialect::TypeScript,
    )
    .expect("comments are skipped");
    assert_eq!(type_script.len(), 2);
}

#[test]
fn an_unterminated_string_reports_its_position() {
    let error = sv_parse_calls("dispatch(\"zoom)\n", SVScriptDialect::Lua)
        .expect_err("the quote never closes");

    assert_eq!(error.location.line, 1);
    assert_eq!(error.location.column, 10);
    assert!(error.message.contains("unterminated"));
}

#[test]
fn two_statements_on_one_line_are_rejected() {
    let error = sv_parse_calls(r#"log("a") log("b")"#, SVScriptDialect::Lua)
        .expect_err("statements need a line break");

    assert!(error.message.contains("line break"), "{}", error.message);
}

#[test]
fn a_table_is_only_allowed_as_the_second_argument() {
    let error = sv_parse_calls(r#"dispatch({ axis = "x" }, "zoom")"#, SVScriptDialect::Lua)
        .expect_err("the table must come last");

    assert!(error.message.contains("table"), "{}", error.message);
}
