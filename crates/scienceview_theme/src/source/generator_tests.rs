//! Executable specification of the build-time compiler's validation.
//!
//! `build.rs` itself cannot be unit tested, so the same generator the build
//! script drives is compiled into the test build and asked about the authored
//! source directly. This keeps the build-time path covered by tests as well as
//! the runtime path.

use super::generator::sv_generate_theme_source;
use super::sv_parse_theme_source;

const SV_AUTHORED_SOURCE: &str = include_str!("../../stylesheet/scienceview.scss");

fn sv_generate_with(from: &str, to: &str) -> Result<String, String> {
    assert!(
        SV_AUTHORED_SOURCE.contains(from),
        "the authored stylesheet must contain `{from}` for this test to be meaningful"
    );
    let source = SV_AUTHORED_SOURCE.replace(from, to);
    let parsed = sv_parse_theme_source(&source).expect("the edited source still parses");
    sv_generate_theme_source(&parsed)
}

#[test]
fn the_build_time_compiler_accepts_the_authored_stylesheet() {
    let parsed = sv_parse_theme_source(SV_AUTHORED_SOURCE).expect("the authored source parses");
    let generated = sv_generate_theme_source(&parsed).expect("the authored source compiles");
    assert!(generated.contains("fn sv_generated_theme_data"));
}

#[test]
fn the_build_time_compiler_rejects_a_one_stop_colour_map() {
    let error = sv_generate_with(
        "$palette.sequential.magma: #000004 #3b0f70 #8c2981 #de4968 #fe9f6d #fcfdbf;",
        "$palette.sequential.magma: #000004;",
    )
    .expect_err("a one-stop colour map must fail the build");
    assert!(error.contains("at least two stops"), "{error}");
}

#[test]
fn the_build_time_compiler_rejects_a_non_finite_number() {
    let error = sv_generate_with("$spacing.medium: 16;", "$spacing.medium: NaN;")
        .expect_err("a non-finite length must fail the build");
    assert!(error.contains("spacing.medium"), "{error}");
}

#[test]
fn the_build_time_compiler_rejects_a_negative_duration() {
    let error = sv_generate_with("$motion.duration.fast: 120;", "$motion.duration.fast: -5;")
        .expect_err("a negative duration must fail the build");
    assert!(error.contains("motion.duration.fast"), "{error}");
}
