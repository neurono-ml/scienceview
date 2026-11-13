//! The compiled theme emitted as an overridable stylesheet.
//!
//! Every rule reads a CSS custom property defined on the figure element. A host
//! page overrides one style by writing a more specific rule for that property,
//! which is the documented theming contract (design D3). Nothing here parses a
//! stylesheet: the values come from the compiled [`scienceview_theme`] tokens.

use scienceview_theme::SVThemeTokens;
use scienceview_theme::sv_default_theme;

/// Returns the self-contained stylesheet for the default compiled theme.
pub fn sv_html_stylesheet() -> String {
    sv_stylesheet_for_tokens(&sv_default_theme().tokens())
}

/// Returns the stylesheet for an already compiled token snapshot.
pub fn sv_stylesheet_for_tokens(tokens: &SVThemeTokens) -> String {
    let colour = tokens.colour();
    let typography = tokens.typography();
    let spacing = tokens.spacing();
    let radius = tokens.radius();
    let motion = tokens.motion();
    let mut css = String::new();

    css.push_str(".scienceview-figure{");
    css.push_str("--sv-colour-background:");
    css.push_str(&colour.background().to_hex_string());
    css.push_str(";--sv-colour-surface:");
    css.push_str(&colour.surface().to_hex_string());
    css.push_str(";--sv-colour-text:");
    css.push_str(&colour.text().to_hex_string());
    css.push_str(";--sv-colour-text-muted:");
    css.push_str(&colour.text_muted().to_hex_string());
    css.push_str(";--sv-colour-grid:");
    css.push_str(&colour.grid().to_hex_string());
    css.push_str(";--sv-colour-axis:");
    css.push_str(&colour.axis().to_hex_string());
    css.push_str(";--sv-colour-accent:");
    css.push_str(&colour.accent().to_hex_string());
    css.push_str(";--sv-font-family:");
    css.push_str(&sv_css_string(typography.family()));
    css.push_str(";--sv-font-size-body:");
    css.push_str(&sv_css_pixels(typography.size().body()));
    css.push_str(";--sv-font-size-title:");
    css.push_str(&sv_css_pixels(typography.size().title()));
    css.push_str(";--sv-font-size-label:");
    css.push_str(&sv_css_pixels(typography.size().label()));
    css.push_str(";--sv-font-weight-regular:");
    css.push_str(&typography.weight().regular().to_string());
    css.push_str(";--sv-font-weight-bold:");
    css.push_str(&typography.weight().bold().to_string());
    css.push_str(";--sv-line-height:");
    css.push_str(&sv_css_number(typography.line_height()));
    css.push_str(";--sv-spacing-unit:");
    css.push_str(&sv_css_pixels(spacing.unit()));
    css.push_str(";--sv-spacing-small:");
    css.push_str(&sv_css_pixels(spacing.small()));
    css.push_str(";--sv-spacing-medium:");
    css.push_str(&sv_css_pixels(spacing.medium()));
    css.push_str(";--sv-spacing-large:");
    css.push_str(&sv_css_pixels(spacing.large()));
    css.push_str(";--sv-radius-medium:");
    css.push_str(&sv_css_pixels(radius.medium()));
    css.push_str(";--sv-motion-duration-normal:");
    css.push_str(&sv_css_duration(
        motion.duration(scienceview_theme::SVMotionDuration::Normal),
    ));
    css.push_str(";--sv-motion-easing-standard:");
    css.push_str(&sv_css_easing(
        motion.easing(scienceview_theme::SVMotionEasing::Standard),
    ));
    css.push_str(";box-sizing:border-box;margin:0;padding:var(--sv-spacing-medium);");
    css.push_str("background:var(--sv-colour-background);color:var(--sv-colour-text);");
    css.push_str("font-family:var(--sv-font-family);font-size:var(--sv-font-size-body);");
    css.push_str("line-height:var(--sv-line-height);border-radius:var(--sv-radius-medium);");
    css.push_str("transition:background var(--sv-motion-duration-normal) var(--sv-motion-easing-standard)}\n");

    css.push_str(".scienceview-figure__canvas{display:block;overflow:hidden;line-height:0;");
    css.push_str("touch-action:none;transform-origin:0 0;will-change:transform}\n");
    css.push_str(".scienceview-figure__canvas>svg{display:block;width:100%;height:auto}\n");
    css.push_str(".scienceview-figure__caption{margin-top:var(--sv-spacing-small);");
    css.push_str("color:var(--sv-colour-text);font-size:var(--sv-font-size-title);");
    css.push_str("font-weight:var(--sv-font-weight-bold)}\n");
    css.push_str("@media (prefers-reduced-motion:reduce){.scienceview-figure{");
    css.push_str("--sv-motion-duration-normal:0ms}}\n");
    css.push_str("[data-scienceview-reduced-motion=\"true\"]{--sv-motion-duration-normal:0ms}\n");

    css
}

fn sv_css_string(value: &str) -> String {
    let escaped: String = value
        .chars()
        .map(|character| match character {
            '"' | '\\' => {
                let mut escaped = String::from("\\");
                escaped.push(character);
                escaped
            }
            other => other.to_string(),
        })
        .collect();
    format!("\"{escaped}\"")
}

fn sv_css_number(value: f64) -> String {
    scienceview_svg::sv_format_number(value)
}

fn sv_css_pixels(value: f64) -> String {
    format!("{}px", sv_css_number(value))
}

fn sv_css_duration(duration: std::time::Duration) -> String {
    let milliseconds = duration.as_secs_f64() * 1_000.0;
    format!("{}ms", sv_css_number(milliseconds))
}

fn sv_css_easing(easing: scienceview_theme::SVTimingFunction) -> String {
    let (x1, y1, x2, y2) = easing.control_components();
    format!(
        "cubic-bezier({},{},{},{})",
        sv_css_number(x1),
        sv_css_number(y1),
        sv_css_number(x2),
        sv_css_number(y2)
    )
}
