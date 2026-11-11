//! Value parsing and emission shared by the build-time compiler.
//!
//! Kept apart from the generator so each file stays small and readable.

use super::ast::SVThemeSource;

pub(super) fn sv_value<'a>(source: &'a SVThemeSource, token_name: &str) -> &'a str {
    // Presence is checked before generation starts.
    &source
        .declaration(token_name)
        .expect("required token presence is validated before generation")
        .token_value
}

pub(super) fn sv_emit_colour(token_name: &str, value: &str) -> Result<String, String> {
    let (red, green, blue, alpha) =
        sv_parse_hex_colour(value).map_err(|message| format!("token `{token_name}`: {message}"))?;
    Ok(format!(
        "crate::tokens::SVColour::new({red}, {green}, {blue}, {alpha})"
    ))
}

pub(super) fn sv_parse_number(token_name: &str, value: &str) -> Result<f64, String> {
    let number = value
        .parse::<f64>()
        .map_err(|_| format!("token `{token_name}`: `{value}` is not a number"))?;
    if !number.is_finite() {
        return Err(format!(
            "token `{token_name}`: `{value}` is not a finite number"
        ));
    }
    Ok(number)
}

/// Parses a number that can never be negative: a length, a size or a duration.
pub(super) fn sv_parse_non_negative_number(token_name: &str, value: &str) -> Result<f64, String> {
    let number = sv_parse_number(token_name, value)?;
    if number < 0.0 {
        return Err(format!(
            "token `{token_name}`: `{value}` must not be negative"
        ));
    }
    Ok(number)
}

pub(super) fn sv_emit_f64(value: f64) -> String {
    format!("{value:?}")
}

pub(super) fn sv_parse_weight(token_name: &str, value: &str) -> Result<u16, String> {
    let weight = value
        .parse::<u16>()
        .map_err(|_| format!("token `{token_name}`: `{value}` is not a font weight"))?;
    if !(1..=1000).contains(&weight) {
        return Err(format!(
            "token `{token_name}`: `{value}` is outside the 1..=1000 font-weight range"
        ));
    }
    Ok(weight)
}

pub(super) fn sv_emit_shadow(token_name: &str, value: &str) -> Result<String, String> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() != 4 {
        return Err(format!(
            "token `{token_name}`: expected `offset_x offset_y blur_radius colour`, found `{value}`"
        ));
    }
    let offset_x = sv_emit_f64(sv_parse_number(token_name, parts[0])?);
    let offset_y = sv_emit_f64(sv_parse_number(token_name, parts[1])?);
    let blur_radius = sv_emit_f64(sv_parse_non_negative_number(token_name, parts[2])?);
    let colour = sv_emit_colour(token_name, parts[3])?;
    Ok(format!(
        "crate::tokens::SVShadow::new({offset_x}, {offset_y}, {blur_radius}, {colour})"
    ))
}

pub(super) fn sv_parse_boolean(token_name: &str, value: &str) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!(
            "token `{token_name}`: expected `true` or `false`, found `{value}`"
        )),
    }
}

pub(super) fn sv_emit_easing(token_name: &str, value: &str) -> Result<String, String> {
    let keyword = match value {
        "linear" => Some("Linear"),
        "ease" => Some("Ease"),
        "ease-in" => Some("EaseIn"),
        "ease-out" => Some("EaseOut"),
        "ease-in-out" => Some("EaseInOut"),
        _ => None,
    };
    if let Some(keyword) = keyword {
        return Ok(format!("crate::tokens::SVTimingFunction::{keyword}"));
    }
    let Some(arguments) = value
        .strip_prefix("cubic-bezier(")
        .and_then(|rest| rest.strip_suffix(')'))
    else {
        return Err(format!(
            "token `{token_name}`: expected a timing keyword or `cubic-bezier(x1, y1, x2, y2)`, found `{value}`"
        ));
    };
    let components: Vec<f64> = arguments
        .split(',')
        .map(|component| {
            component
                .trim()
                .parse::<f64>()
                .map_err(|_| format!("token `{token_name}`: `{component}` is not a number"))
        })
        .collect::<Result<_, _>>()?;
    if components.len() != 4 {
        return Err(format!(
            "token `{token_name}`: `cubic-bezier` needs four components, found {}",
            components.len()
        ));
    }
    if !(0.0..=1.0).contains(&components[0]) || !(0.0..=1.0).contains(&components[2]) {
        return Err(format!(
            "token `{token_name}`: the first and third `cubic-bezier` components must be within 0..=1"
        ));
    }
    Ok(format!(
        "crate::tokens::SVTimingFunction::CubicBezier {{ x1: {}, y1: {}, x2: {}, y2: {} }}",
        sv_emit_f64(components[0]),
        sv_emit_f64(components[1]),
        sv_emit_f64(components[2]),
        sv_emit_f64(components[3])
    ))
}

pub(super) fn sv_emit_colour_list(token_name: &str, value: &str) -> Result<String, String> {
    let mut emitted = Vec::new();
    for component in value.split_whitespace() {
        emitted.push(sv_emit_colour(token_name, component)?);
    }
    if emitted.is_empty() {
        return Err(format!("token `{token_name}`: the palette is empty"));
    }
    Ok(emitted.join(", "))
}

pub(super) fn sv_parse_hex_colour(value: &str) -> Result<(u8, u8, u8, u8), String> {
    let Some(hex) = value.strip_prefix('#') else {
        return Err(format!("`{value}` is not a `#rrggbb` colour"));
    };
    let digits: Vec<u8> = hex
        .chars()
        .map(|character| {
            character
                .to_digit(16)
                .map(|digit| digit as u8)
                .ok_or_else(|| format!("`{value}` contains a non-hexadecimal digit"))
        })
        .collect::<Result<_, _>>()?;
    match digits.len() {
        3 => Ok((digits[0] * 17, digits[1] * 17, digits[2] * 17, 255)),
        6 => Ok((
            digits[0] * 16 + digits[1],
            digits[2] * 16 + digits[3],
            digits[4] * 16 + digits[5],
            255,
        )),
        8 => Ok((
            digits[0] * 16 + digits[1],
            digits[2] * 16 + digits[3],
            digits[4] * 16 + digits[5],
            digits[6] * 16 + digits[7],
        )),
        _ => Err(format!("`{value}` must have 3, 6 or 8 hexadecimal digits")),
    }
}
