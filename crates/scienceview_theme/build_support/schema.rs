//! Canonical token names, shared by the build-time generator (which must emit
//! every one of them) and the runtime compiler/resolver (which must find them).

pub const SV_COLOUR_TOKEN_NAMES: &[&str] = &[
    "colour.background",
    "colour.surface",
    "colour.text",
    "colour.text_muted",
    "colour.grid",
    "colour.axis",
    "colour.accent",
    "colour.success",
    "colour.warning",
    "colour.error",
];

pub const SV_TYPOGRAPHY_TOKEN_NAMES: &[&str] = &[
    "typography.family",
    "typography.size.body",
    "typography.size.title",
    "typography.size.label",
    "typography.weight.regular",
    "typography.weight.bold",
    "typography.line_height",
];

pub const SV_SPACING_TOKEN_NAMES: &[&str] = &[
    "spacing.unit",
    "spacing.x_small",
    "spacing.small",
    "spacing.medium",
    "spacing.large",
    "spacing.x_large",
];

pub const SV_RADIUS_TOKEN_NAMES: &[&str] = &[
    "radius.none",
    "radius.small",
    "radius.medium",
    "radius.large",
];

pub const SV_SHADOW_TOKEN_NAMES: &[&str] = &["shadow.none", "shadow.overlay"];

pub const SV_MOTION_TOKEN_NAMES: &[&str] = &[
    "motion.duration.fast",
    "motion.duration.normal",
    "motion.duration.slow",
    "motion.easing.standard",
    "motion.easing.emphasized",
    "motion.easing.decelerate",
    "motion.reduced",
];

pub const SV_PALETTE_CATEGORICAL_PREFIX: &str = "palette.categorical.";
pub const SV_PALETTE_SEQUENTIAL_PREFIX: &str = "palette.sequential.";
pub const SV_PALETTE_DIVERGING_PREFIX: &str = "palette.diverging.";
pub const SV_PALETTE_CYCLICAL_PREFIX: &str = "palette.cyclical.";

/// Every token the compiled theme must define. A missing entry fails the
/// build, never a figure render.
pub fn sv_required_token_names() -> Vec<&'static str> {
    let mut names = Vec::new();
    names.extend_from_slice(SV_COLOUR_TOKEN_NAMES);
    names.extend_from_slice(SV_TYPOGRAPHY_TOKEN_NAMES);
    names.extend_from_slice(SV_SPACING_TOKEN_NAMES);
    names.extend_from_slice(SV_RADIUS_TOKEN_NAMES);
    names.extend_from_slice(SV_SHADOW_TOKEN_NAMES);
    names.extend_from_slice(SV_MOTION_TOKEN_NAMES);
    names
}
