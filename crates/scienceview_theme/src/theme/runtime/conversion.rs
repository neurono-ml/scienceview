//! Converts a parsed stylesheet into the typed theme value.
//!
//! This is the runtime twin of `build_support/generator.rs`. Both consume the
//! same parsed source; the determinism tests compare their output, so the two
//! paths cannot drift.

use crate::error::SVThemeError;
use crate::palettes::{SVColourMapKind, SVColourPalettes, SVContinuousPalette, SVDiscretePalette};
use crate::source::{
    SV_COLOUR_TOKEN_NAMES, SV_MOTION_TOKEN_NAMES, SV_PALETTE_CATEGORICAL_PREFIX,
    SV_PALETTE_CYCLICAL_PREFIX, SV_PALETTE_DIVERGING_PREFIX, SV_PALETTE_SEQUENTIAL_PREFIX,
    SV_RADIUS_TOKEN_NAMES, SV_SHADOW_TOKEN_NAMES, SV_SPACING_TOKEN_NAMES, SVThemeSource,
    sv_required_token_names,
};
use crate::theme::SVThemeData;
use crate::theme::runtime::parsing::{
    sv_boolean, sv_colour, sv_colour_stops, sv_font_weight, sv_invalid, sv_non_negative_number,
    sv_timing_function, sv_value,
};
use crate::tokens::{
    SVColour, SVColourTokens, SVFontSizes, SVFontWeights, SVMotionDurations, SVMotionEasings,
    SVMotionTokens, SVRadiusTokens, SVShadow, SVShadowTokens, SVSpacingTokens, SVThemeTokens,
    SVTypographyTokens,
};

pub(crate) fn sv_theme_data_from_source(
    source: &SVThemeSource,
) -> Result<SVThemeData, SVThemeError> {
    for token_name in sv_required_token_names() {
        if source.declaration(token_name).is_none() {
            return Err(SVThemeError::MissingToken {
                token_name: token_name.to_string(),
            });
        }
    }
    let tokens = SVThemeTokens {
        colour: sv_colour_tokens(source)?,
        typography: sv_typography_tokens(source)?,
        spacing: sv_spacing_tokens(source)?,
        radius: sv_radius_tokens(source)?,
        shadow: sv_shadow_tokens(source)?,
        motion: sv_motion_tokens(source)?,
    };
    Ok(SVThemeData {
        tokens,
        palettes: sv_palettes(source)?,
    })
}

fn sv_colour_tokens(source: &SVThemeSource) -> Result<SVColourTokens, SVThemeError> {
    Ok(SVColourTokens {
        background: sv_colour(source, SV_COLOUR_TOKEN_NAMES[0])?,
        surface: sv_colour(source, SV_COLOUR_TOKEN_NAMES[1])?,
        text: sv_colour(source, SV_COLOUR_TOKEN_NAMES[2])?,
        text_muted: sv_colour(source, SV_COLOUR_TOKEN_NAMES[3])?,
        grid: sv_colour(source, SV_COLOUR_TOKEN_NAMES[4])?,
        axis: sv_colour(source, SV_COLOUR_TOKEN_NAMES[5])?,
        accent: sv_colour(source, SV_COLOUR_TOKEN_NAMES[6])?,
        success: sv_colour(source, SV_COLOUR_TOKEN_NAMES[7])?,
        warning: sv_colour(source, SV_COLOUR_TOKEN_NAMES[8])?,
        error: sv_colour(source, SV_COLOUR_TOKEN_NAMES[9])?,
    })
}

fn sv_typography_tokens(source: &SVThemeSource) -> Result<SVTypographyTokens, SVThemeError> {
    let family = sv_value(source, "typography.family")?.to_string();
    let regular = sv_font_weight(source, "typography.weight.regular")?;
    let bold = sv_font_weight(source, "typography.weight.bold")?;
    Ok(SVTypographyTokens {
        family,
        size: SVFontSizes {
            body: sv_non_negative_number(source, "typography.size.body")?,
            title: sv_non_negative_number(source, "typography.size.title")?,
            label: sv_non_negative_number(source, "typography.size.label")?,
        },
        weight: SVFontWeights { regular, bold },
        line_height: sv_non_negative_number(source, "typography.line_height")?,
    })
}

fn sv_spacing_tokens(source: &SVThemeSource) -> Result<SVSpacingTokens, SVThemeError> {
    Ok(SVSpacingTokens {
        unit: sv_non_negative_number(source, SV_SPACING_TOKEN_NAMES[0])?,
        x_small: sv_non_negative_number(source, SV_SPACING_TOKEN_NAMES[1])?,
        small: sv_non_negative_number(source, SV_SPACING_TOKEN_NAMES[2])?,
        medium: sv_non_negative_number(source, SV_SPACING_TOKEN_NAMES[3])?,
        large: sv_non_negative_number(source, SV_SPACING_TOKEN_NAMES[4])?,
        x_large: sv_non_negative_number(source, SV_SPACING_TOKEN_NAMES[5])?,
    })
}

fn sv_radius_tokens(source: &SVThemeSource) -> Result<SVRadiusTokens, SVThemeError> {
    Ok(SVRadiusTokens {
        none: sv_non_negative_number(source, SV_RADIUS_TOKEN_NAMES[0])?,
        small: sv_non_negative_number(source, SV_RADIUS_TOKEN_NAMES[1])?,
        medium: sv_non_negative_number(source, SV_RADIUS_TOKEN_NAMES[2])?,
        large: sv_non_negative_number(source, SV_RADIUS_TOKEN_NAMES[3])?,
    })
}

fn sv_shadow(source: &SVThemeSource, token_name: &str) -> Result<SVShadow, SVThemeError> {
    let value = sv_value(source, token_name)?;
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() != 4 {
        return Err(sv_invalid(
            token_name,
            value,
            "`offset_x offset_y blur_radius colour`",
        ));
    }
    Ok(SVShadow::new(
        parts[0]
            .parse()
            .map_err(|_| sv_invalid(token_name, value, "numeric shadow offsets"))?,
        parts[1]
            .parse()
            .map_err(|_| sv_invalid(token_name, value, "numeric shadow offsets"))?,
        {
            let blur_radius = parts[2]
                .parse::<f64>()
                .map_err(|_| sv_invalid(token_name, value, "a numeric blur radius"))?;
            if !blur_radius.is_finite() || blur_radius < 0.0 {
                return Err(sv_invalid(
                    token_name,
                    value,
                    "a finite, non-negative blur radius",
                ));
            }
            blur_radius
        },
        SVColour::from_hex(parts[3])
            .map_err(|_| sv_invalid(token_name, value, "a shadow colour"))?,
    ))
}

fn sv_shadow_tokens(source: &SVThemeSource) -> Result<SVShadowTokens, SVThemeError> {
    Ok(SVShadowTokens {
        none: sv_shadow(source, SV_SHADOW_TOKEN_NAMES[0])?,
        overlay: sv_shadow(source, SV_SHADOW_TOKEN_NAMES[1])?,
    })
}

fn sv_motion_tokens(source: &SVThemeSource) -> Result<SVMotionTokens, SVThemeError> {
    Ok(SVMotionTokens {
        durations: SVMotionDurations {
            fast_milliseconds: sv_non_negative_number(source, SV_MOTION_TOKEN_NAMES[0])?,
            normal_milliseconds: sv_non_negative_number(source, SV_MOTION_TOKEN_NAMES[1])?,
            slow_milliseconds: sv_non_negative_number(source, SV_MOTION_TOKEN_NAMES[2])?,
        },
        easings: SVMotionEasings {
            standard: sv_timing_function(source, SV_MOTION_TOKEN_NAMES[3])?,
            emphasized: sv_timing_function(source, SV_MOTION_TOKEN_NAMES[4])?,
            decelerate: sv_timing_function(source, SV_MOTION_TOKEN_NAMES[5])?,
        },
        reduced_motion: sv_boolean(source, SV_MOTION_TOKEN_NAMES[6])?,
    })
}

fn sv_palettes(source: &SVThemeSource) -> Result<SVColourPalettes, SVThemeError> {
    let mut categorical = Vec::new();
    let mut continuous = Vec::new();
    for declaration in &source.declarations {
        let token_name = declaration.token_name.as_str();
        if let Some(palette_name) = token_name.strip_prefix(SV_PALETTE_CATEGORICAL_PREFIX) {
            categorical.push(SVDiscretePalette::new(
                palette_name,
                sv_colour_stops(token_name, &declaration.token_value)?,
            )?);
            continue;
        }
        let kind = [
            (SV_PALETTE_SEQUENTIAL_PREFIX, SVColourMapKind::Sequential),
            (SV_PALETTE_DIVERGING_PREFIX, SVColourMapKind::Diverging),
            (SV_PALETTE_CYCLICAL_PREFIX, SVColourMapKind::Cyclical),
        ]
        .into_iter()
        .find_map(|(prefix, kind)| token_name.strip_prefix(prefix).map(|name| (kind, name)));
        if let Some((kind, palette_name)) = kind {
            continuous.push(SVContinuousPalette::new(
                palette_name,
                kind,
                sv_colour_stops(token_name, &declaration.token_value)?,
            )?);
        }
    }
    Ok(SVColourPalettes {
        categorical,
        continuous,
    })
}
