//! Build-time compilation of the authored theme stylesheet into typed Rust.
//!
//! The stylesheet is the single source of the visual identity. Compiling it
//! here means every backend receives validated values and a missing or
//! malformed token fails the build instead of a render.

#[path = "build_support/mod.rs"]
mod build_support;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_directory = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set");
    let stylesheet_path = Path::new(&manifest_directory)
        .join("stylesheet")
        .join("scienceview.scss");
    println!("cargo:rerun-if-changed={}", stylesheet_path.display());

    let source_text = fs::read_to_string(&stylesheet_path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", stylesheet_path.display()));
    let parsed =
        build_support::parser::sv_parse_theme_source(&source_text).unwrap_or_else(|error| {
            panic!(
                "{}:{}: {}",
                stylesheet_path.display(),
                error.source_line,
                error.message
            )
        });
    let generated = build_support::generator::sv_generate_theme_source(&parsed)
        .unwrap_or_else(|message| panic!("{}: {}", stylesheet_path.display(), message));

    let output_directory = env::var("OUT_DIR").expect("OUT_DIR is set");
    let output_path = Path::new(&output_directory).join("generated_theme_tokens.rs");
    fs::write(&output_path, generated)
        .unwrap_or_else(|error| panic!("cannot write {}: {error}", output_path.display()));
}
