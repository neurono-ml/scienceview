//! Executable specification of runtime compilation and in-place restyling.

use crate::{SVColour, SVTheme, SVThemeError, SVTokenValue, sv_compile_theme, sv_default_theme};

/// The checked-in stylesheet, read as the runtime compiler would read a user
/// theme. Using the real file is what makes build-time and runtime parity
/// testable rather than assumed.
const SV_AUTHORED_SOURCE: &str = include_str!("../../stylesheet/scienceview.scss");

fn sv_authored_source_with(from: &str, to: &str) -> String {
    assert!(
        SV_AUTHORED_SOURCE.contains(from),
        "the authored stylesheet must contain `{from}` for this test to be meaningful"
    );
    SV_AUTHORED_SOURCE.replace(from, to)
}

/// A stand-in for a figure: it holds a clone of the theme handle and reads the
/// tokens on every access, exactly as a renderer does.
struct SVProbeFigure {
    theme: SVTheme,
}

impl SVProbeFigure {
    fn background(&self) -> SVColour {
        self.theme.tokens().colour().background()
    }
}

#[test]
fn runtime_compilation_reproduces_the_build_time_theme() {
    let compiled = sv_compile_theme(SV_AUTHORED_SOURCE).expect("the authored source is valid");
    let built = sv_default_theme();
    assert_eq!(compiled.tokens(), built.tokens());
    assert_eq!(compiled.palettes(), built.palettes());
}

#[test]
fn the_builder_can_compile_an_in_memory_source() {
    let recoloured = sv_authored_source_with(
        "$colour.background: #ffffff;",
        "$colour.background: #abcdef;",
    );
    let theme = SVTheme::builder()
        .source(recoloured)
        .build()
        .expect("the recoloured source is valid");
    assert_eq!(
        theme.tokens().colour().background(),
        SVColour::new(0xab, 0xcd, 0xef, 255)
    );
}

#[test]
fn recompilation_adopts_new_tokens_without_rebuilding_the_figure() {
    let theme = sv_default_theme();
    let figure = SVProbeFigure {
        theme: theme.clone(),
    };
    assert_eq!(figure.background(), SVColour::new(255, 255, 255, 255));

    let recoloured = sv_authored_source_with(
        "$colour.background: #ffffff;",
        "$colour.background: #101010;",
    );
    theme
        .recompile(&recoloured)
        .expect("the recoloured source is valid");

    // The figure was not rebuilt: it still holds the same handle.
    assert_eq!(figure.background(), SVColour::new(0x10, 0x10, 0x10, 255));
    assert_eq!(
        theme.resolve("colour.background"),
        Ok(SVTokenValue::Colour(SVColour::new(0x10, 0x10, 0x10, 255)))
    );
}

#[test]
fn recompilation_carries_palette_and_motion_changes() {
    let theme = sv_default_theme();
    let modified = sv_authored_source_with(
        "$motion.duration.normal: 240;",
        "$motion.duration.normal: 900;",
    )
    .replace(
        "$palette.categorical.default: #0969da",
        "$palette.categorical.default: #ff0000",
    );
    theme
        .recompile(&modified)
        .expect("the modified source is valid");
    assert_eq!(
        theme.tokens().motion().durations().normal_milliseconds(),
        900.0
    );
    let palettes = theme.palettes();
    let palette = palettes
        .categorical_palette("default")
        .expect("the palette still exists");
    assert_eq!(palette.colours()[0], SVColour::new(255, 0, 0, 255));
}

#[test]
fn a_broken_recompilation_keeps_the_previous_tokens() {
    let theme = sv_default_theme();
    let before = theme.tokens();
    let broken = sv_authored_source_with(
        "$colour.background: #ffffff;",
        "$colour.background #ffffff;",
    );
    let error = theme
        .recompile(&broken)
        .expect_err("a declaration without a colon must fail");
    assert!(matches!(error, SVThemeError::Syntax { .. }));
    assert_eq!(theme.tokens(), before);
}

#[test]
fn malformed_source_reports_the_line_and_never_panics() {
    let error = sv_compile_theme("$colour.background #ffffff;\n")
        .expect_err("a declaration without a colon must fail");
    let SVThemeError::Syntax { source_line, .. } = error else {
        panic!("the error must be a syntax error");
    };
    assert_eq!(source_line, 1);
}

#[test]
fn a_missing_required_token_is_reported_by_name() {
    let missing = sv_authored_source_with("$spacing.medium: 16;\n", "");
    let error = sv_compile_theme(&missing).expect_err("a missing token must fail");
    assert_eq!(
        error,
        SVThemeError::MissingToken {
            token_name: "spacing.medium".to_string()
        }
    );
}

#[test]
fn an_invalid_colour_is_reported_with_its_token_name() {
    let invalid = sv_authored_source_with(
        "$colour.background: #ffffff;",
        "$colour.background: #zzzzzz;",
    );
    let error = sv_compile_theme(&invalid).expect_err("an invalid colour must fail");
    assert!(matches!(
        error,
        SVThemeError::InvalidValue { ref token_name, .. } if token_name == "colour.background"
    ));
}

#[test]
fn a_duplicate_declaration_is_a_syntax_error() {
    let duplicated = format!("{SV_AUTHORED_SOURCE}\n$colour.background: #ffffff;\n");
    let error = sv_compile_theme(&duplicated).expect_err("a duplicate token must fail");
    assert!(matches!(error, SVThemeError::Syntax { .. }));
}

#[test]
fn runtime_compilation_is_deterministic() {
    let first = sv_compile_theme(SV_AUTHORED_SOURCE).expect("valid source");
    let second = sv_compile_theme(SV_AUTHORED_SOURCE).expect("valid source");
    assert_eq!(first.tokens(), second.tokens());
    assert_eq!(first.palettes(), second.palettes());
}

#[test]
fn a_runtime_theme_can_declare_reduced_motion() {
    let reduced = sv_authored_source_with("$motion.reduced: false;", "$motion.reduced: true;");
    let theme = sv_compile_theme(&reduced).expect("the reduced source is valid");
    assert!(theme.reduced_motion());
    assert_eq!(
        theme.resolve("motion.duration.slow"),
        Ok(SVTokenValue::Duration(std::time::Duration::ZERO))
    );
}

#[test]
fn a_colour_map_with_a_single_stop_is_rejected() {
    let broken = sv_authored_source_with(
        "$palette.sequential.magma: #000004 #3b0f70 #8c2981 #de4968 #fe9f6d #fcfdbf;",
        "$palette.sequential.magma: #000004;",
    );
    let error = sv_compile_theme(&broken).expect_err("a one-stop colour map must fail");
    assert_eq!(
        error,
        SVThemeError::EmptyPalette {
            palette_name: "magma".to_string()
        }
    );
}

#[test]
fn a_non_finite_number_is_rejected_instead_of_reaching_a_token() {
    let broken = sv_authored_source_with("$spacing.medium: 16;", "$spacing.medium: NaN;");
    let error = sv_compile_theme(&broken).expect_err("a non-finite length must fail");
    assert!(matches!(
        error,
        SVThemeError::InvalidValue { ref token_name, .. } if token_name == "spacing.medium"
    ));
}

#[test]
fn a_negative_duration_is_rejected_without_panicking() {
    let broken =
        sv_authored_source_with("$motion.duration.fast: 120;", "$motion.duration.fast: -5;");
    let error = sv_compile_theme(&broken).expect_err("a negative duration must fail");
    assert!(matches!(
        error,
        SVThemeError::InvalidValue { ref token_name, .. } if token_name == "motion.duration.fast"
    ));
}
