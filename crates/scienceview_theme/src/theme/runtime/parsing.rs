//! Value parsing shared by the runtime conversion of every token family.

use crate::error::SVThemeError;
use crate::source::SVThemeSource;
use crate::tokens::{SVColour, SVTimingFunction};

pub(super) fn sv_value<'a>(
    source: &'a SVThemeSource,
    token_name: &str,
) -> Result<&'a str, SVThemeError> {
    source
        .declaration(token_name)
        .map(|declaration| declaration.token_value.as_str())
        .ok_or_else(|| SVThemeError::MissingToken {
            token_name: token_name.to_string(),
        })
}

pub(super) fn sv_invalid(token_name: &str, value: &str, expected: &str) -> SVThemeError {
    SVThemeError::InvalidValue {
        token_name: token_name.to_string(),
        value: value.to_string(),
        expected: expected.to_string(),
    }
}

pub(super) fn sv_number(source: &SVThemeSource, token_name: &str) -> Result<f64, SVThemeError> {
    let value = sv_value(source, token_name)?;
    let number = value
        .parse::<f64>()
        .map_err(|_| sv_invalid(token_name, value, "a number"))?;
    if !number.is_finite() {
        return Err(sv_invalid(token_name, value, "a finite number"));
    }
    Ok(number)
}

/// Parses a number that can never be negative: a length, a size or a duration.
pub(super) fn sv_non_negative_number(
    source: &SVThemeSource,
    token_name: &str,
) -> Result<f64, SVThemeError> {
    let number = sv_number(source, token_name)?;
    if number < 0.0 {
        return Err(sv_invalid(
            token_name,
            sv_value(source, token_name)?,
            "a number that is not negative",
        ));
    }
    Ok(number)
}

pub(super) fn sv_colour(
    source: &SVThemeSource,
    token_name: &str,
) -> Result<SVColour, SVThemeError> {
    let value = sv_value(source, token_name)?;
    SVColour::from_hex(value).map_err(|_| sv_invalid(token_name, value, "a #rrggbb colour"))
}

pub(super) fn sv_boolean(source: &SVThemeSource, token_name: &str) -> Result<bool, SVThemeError> {
    match sv_value(source, token_name)? {
        "true" => Ok(true),
        "false" => Ok(false),
        value => Err(sv_invalid(token_name, value, "`true` or `false`")),
    }
}

pub(super) fn sv_font_weight(
    source: &SVThemeSource,
    token_name: &str,
) -> Result<u16, SVThemeError> {
    let value = sv_value(source, token_name)?;
    let weight = value
        .parse::<u16>()
        .map_err(|_| sv_invalid(token_name, value, "a u16 weight"))?;
    if !(1..=1000).contains(&weight) {
        return Err(sv_invalid(token_name, value, "a weight within 1..=1000"));
    }
    Ok(weight)
}

pub(super) fn sv_timing_function(
    source: &SVThemeSource,
    token_name: &str,
) -> Result<SVTimingFunction, SVThemeError> {
    let value = sv_value(source, token_name)?;
    match value {
        "linear" => return Ok(SVTimingFunction::Linear),
        "ease" => return Ok(SVTimingFunction::Ease),
        "ease-in" => return Ok(SVTimingFunction::EaseIn),
        "ease-out" => return Ok(SVTimingFunction::EaseOut),
        "ease-in-out" => return Ok(SVTimingFunction::EaseInOut),
        _ => {}
    }
    let Some(arguments) = value
        .strip_prefix("cubic-bezier(")
        .and_then(|rest| rest.strip_suffix(')'))
    else {
        return Err(sv_invalid(
            token_name,
            value,
            "a timing keyword or `cubic-bezier(x1, y1, x2, y2)`",
        ));
    };
    let components: Vec<f64> = arguments
        .split(',')
        .map(|component| {
            component
                .trim()
                .parse::<f64>()
                .map_err(|_| sv_invalid(token_name, value, "four numeric components"))
        })
        .collect::<Result<_, _>>()?;
    if components.len() != 4 {
        return Err(sv_invalid(
            token_name,
            value,
            "four cubic-bezier components",
        ));
    }
    if !(0.0..=1.0).contains(&components[0]) || !(0.0..=1.0).contains(&components[2]) {
        return Err(sv_invalid(
            token_name,
            value,
            "first and third cubic-bezier components within 0..=1",
        ));
    }
    Ok(SVTimingFunction::CubicBezier {
        x1: components[0],
        y1: components[1],
        x2: components[2],
        y2: components[3],
    })
}

pub(super) fn sv_colour_stops(
    token_name: &str,
    value: &str,
) -> Result<Vec<SVColour>, SVThemeError> {
    value
        .split_whitespace()
        .map(|component| {
            SVColour::from_hex(component)
                .map_err(|_| sv_invalid(token_name, value, "a list of #rrggbb colours"))
        })
        .collect()
}
