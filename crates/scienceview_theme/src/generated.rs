//! The typed tokens emitted by `build.rs` from the authored stylesheet.
//!
//! The generated function is the build-time compilation path; the runtime
//! compiler produces the same value shape from the same source, which is what
//! the determinism tests compare.

include!(concat!(env!("OUT_DIR"), "/generated_theme_tokens.rs"));
