//! Runs one Lua script and one TypeScript script against the same figure state.
//!
//! Run with `cargo run -p scienceview_script --example scripting_engines`.

use scienceview_core::sv_control::{SVControlAxis, SVControlSession, SVControlState};
use scienceview_core::sv_scale::SVDomain;
use scienceview_script::{
    SVScript, SVScriptEngineKind, SVScriptRuntimeBuilder, SVScriptStatePolicy, sv_dispatch_outcome,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = SVControlState::new();
    state.register_axis(SVControlAxis::Horizontal, SVDomain::new(0.0, 10.0)?)?;
    let mut session = SVControlSession::new(state);

    for (kind, name, source) in [
        (
            SVScriptEngineKind::Lua,
            "lua-zoom",
            r#"dispatch("zoom", { axis = "x", factor = 0.5, anchor = 0.5 })"#,
        ),
        (
            SVScriptEngineKind::TypeScript,
            "typescript-pan",
            "dispatch(\"pan\", { axis: \"x\", fraction: 0.1 });",
        ),
    ] {
        let runtime = SVScriptRuntimeBuilder::new().engine(kind).build()?;
        runtime.register(SVScript::builder(name).source(source).build()?)?;
        let outcome = runtime.invoke(name, None)?;
        let applied = sv_dispatch_outcome(&mut session, name, &outcome)?;
        println!(
            "{kind} script `{name}` applied {applied} action(s); visible x: {:?}",
            session.state().visible_range(SVControlAxis::Horizontal)
        );
        runtime.replace(name, "log(\"reloaded\")", SVScriptStatePolicy::Preserve)?;
    }

    Ok(())
}
