//! Mapping a `dispatch` call onto the control action vocabulary.

use scienceview_core::sv_animation::SVAnimationAction;
use scienceview_core::sv_control::{
    SVControlAction, SVControlAxis, SVControlRange, SVSelectionRegion, SVSeriesReference,
};
use scienceview_core::sv_error::SVError;

use super::arguments::{SVScriptArguments, sv_failure};
use super::parser::SVParsedCall;

/// Resolves a dispatch call into one semantic action.
///
/// The name is the vocabulary's own name, so a script cannot invent a mutation:
/// every action a script produces is an action a built-in control can produce.
pub(crate) fn sv_action_from_call(
    script: &str,
    call: &SVParsedCall,
) -> Result<SVControlAction, SVError> {
    let arguments = SVScriptArguments::new(script, call);
    match call.name.as_str() {
        "zoom" => {
            arguments.require_only("zoom", &["axis", "factor", "anchor"])?;
            let factor = arguments.number("factor")?;
            if factor <= 0.0 {
                return Err(sv_failure(
                    script,
                    call.location,
                    "action `zoom` needs a factor greater than zero".to_string(),
                ));
            }
            let anchor = arguments.number("anchor")?;
            if !(0.0..=1.0).contains(&anchor) {
                return Err(sv_failure(
                    script,
                    call.location,
                    "action `zoom` needs an anchor in 0..=1".to_string(),
                ));
            }
            Ok(SVControlAction::Zoom {
                axis: sv_axis(script, &arguments)?,
                factor,
                anchor,
            })
        }
        "pan" => {
            arguments.require_only("pan", &["axis", "fraction"])?;
            Ok(SVControlAction::Pan {
                axis: sv_axis(script, &arguments)?,
                fraction: arguments.number("fraction")?,
            })
        }
        "reset" => {
            arguments.require_empty("reset")?;
            Ok(SVControlAction::Reset)
        }
        "toggle_series" => {
            arguments.require_only("toggle_series", &["series"])?;
            Ok(SVControlAction::ToggleSeries(SVSeriesReference::Name(
                arguments.name_value("series")?,
            )))
        }
        "set_series_visibility" => {
            arguments.require_only("set_series_visibility", &["series", "visible"])?;
            Ok(SVControlAction::SetSeriesVisibility {
                series: SVSeriesReference::Name(arguments.name_value("series")?),
                visible: arguments.boolean("visible")?,
            })
        }
        "set_range" => {
            arguments.require_only("set_range", &["axis", "minimum", "maximum"])?;
            let minimum = arguments.number("minimum")?;
            let maximum = arguments.number("maximum")?;
            Ok(SVControlAction::SetRange {
                axis: sv_axis(script, &arguments)?,
                range: sv_range(script, call, minimum, maximum)?,
            })
        }
        "select_region" => {
            arguments.require_only(
                "select_region",
                &["x_minimum", "x_maximum", "y_minimum", "y_maximum"],
            )?;
            let horizontal = sv_range(
                script,
                call,
                arguments.number("x_minimum")?,
                arguments.number("x_maximum")?,
            )?;
            let vertical = sv_range(
                script,
                call,
                arguments.number("y_minimum")?,
                arguments.number("y_maximum")?,
            )?;
            Ok(SVControlAction::SelectRegion(SVSelectionRegion::new(
                horizontal, vertical,
            )))
        }
        "clear_selection" => {
            arguments.require_empty("clear_selection")?;
            Ok(SVControlAction::ClearSelection)
        }
        "set_playback" => {
            arguments.require_only("set_playback", &["command"])?;
            Ok(SVControlAction::SetPlayback(sv_playback_command(
                script,
                call,
                &arguments.name_value("command")?,
            )?))
        }
        "custom" => {
            arguments.require_only("custom", &["name", "payload"])?;
            Ok(SVControlAction::Custom {
                name: arguments.string("name")?,
                payload: arguments.optional_string("payload")?,
            })
        }
        other => Err(SVError::ScriptUnknownAction {
            script: script.to_string(),
            location: call.location,
            action: other.to_string(),
        }),
    }
}

fn sv_axis(script: &str, arguments: &SVScriptArguments<'_>) -> Result<SVControlAxis, SVError> {
    match arguments.name_value("axis")?.as_str() {
        "x" | "horizontal" => Ok(SVControlAxis::Horizontal),
        "y" | "vertical" => Ok(SVControlAxis::Vertical),
        other => Err(sv_failure(
            script,
            arguments.field_location("axis"),
            format!("`{other}` is not an axis; use `x` or `y`"),
        )),
    }
}

fn sv_range(
    script: &str,
    call: &SVParsedCall,
    minimum: f64,
    maximum: f64,
) -> Result<SVControlRange, SVError> {
    SVControlRange::new(minimum, maximum)
        .map_err(|error| sv_failure(script, call.location, format!("invalid range: {error}")))
}

fn sv_playback_command(
    script: &str,
    call: &SVParsedCall,
    command: &str,
) -> Result<SVAnimationAction, SVError> {
    match command {
        "play" => Ok(SVAnimationAction::Play),
        "pause" => Ok(SVAnimationAction::Pause),
        "toggle" => Ok(SVAnimationAction::Toggle),
        "stop" => Ok(SVAnimationAction::Stop),
        "step_forward" => Ok(SVAnimationAction::Step { forward: true }),
        "step_backward" => Ok(SVAnimationAction::Step { forward: false }),
        other => Err(sv_failure(
            script,
            call.location,
            format!(
                "`{other}` is not a playback command; use play, pause, toggle, stop, step_forward or step_backward"
            ),
        )),
    }
}
