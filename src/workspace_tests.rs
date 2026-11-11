//! Workspace smoke tests.
//!
//! The root `scienceview` package is a facade over the workspace: the core
//! crate must be reachable through it, and no rendering backend may be enabled
//! by the default feature set.

/// Every backend feature declared by the root package.
#[cfg(not(any(
    feature = "render-svg",
    feature = "render-html",
    feature = "render-tui",
    feature = "render-native",
    feature = "render-react",
    feature = "render-vue",
    feature = "render-angular",
    feature = "render-tauri",
    feature = "render-dioxus",
    feature = "wasm",
)))]
const BACKEND_FEATURES: [&str; 10] = [
    "render-svg",
    "render-html",
    "render-tui",
    "render-native",
    "render-react",
    "render-vue",
    "render-angular",
    "render-tauri",
    "render-dioxus",
    "wasm",
];

#[cfg(not(any(
    feature = "render-svg",
    feature = "render-html",
    feature = "render-tui",
    feature = "render-native",
    feature = "render-react",
    feature = "render-vue",
    feature = "render-angular",
    feature = "render-tauri",
    feature = "render-dioxus",
    feature = "wasm",
)))]
fn backend_feature_is_enabled(feature: &str) -> bool {
    match feature {
        "render-svg" => cfg!(feature = "render-svg"),
        "render-html" => cfg!(feature = "render-html"),
        "render-tui" => cfg!(feature = "render-tui"),
        "render-native" => cfg!(feature = "render-native"),
        "render-react" => cfg!(feature = "render-react"),
        "render-vue" => cfg!(feature = "render-vue"),
        "render-angular" => cfg!(feature = "render-angular"),
        "render-tauri" => cfg!(feature = "render-tauri"),
        "render-dioxus" => cfg!(feature = "render-dioxus"),
        "wasm" => cfg!(feature = "wasm"),
        unknown => panic!("unknown backend feature {unknown}"),
    }
}

#[test]
fn core_crate_is_reachable_through_the_facade() {
    assert_eq!(scienceview_core::sv_crate_name(), "scienceview_core");
    assert_eq!(crate::scienceview_core::sv_crate_name(), "scienceview_core");
}

#[allow(unused_imports)]
#[test]
fn enabled_backends_are_reachable_through_the_facade() {
    #[cfg(feature = "render-angular")]
    use crate::scienceview_angular;
    #[cfg(feature = "render-dioxus")]
    use crate::scienceview_dioxus;
    #[cfg(feature = "render-html")]
    use crate::scienceview_html;
    #[cfg(feature = "render-native")]
    use crate::scienceview_native;
    #[cfg(feature = "render-react")]
    use crate::scienceview_react;
    #[cfg(feature = "render-svg")]
    use crate::scienceview_svg;
    #[cfg(feature = "render-tauri")]
    use crate::scienceview_tauri;
    #[cfg(feature = "render-tui")]
    use crate::scienceview_tui;
    #[cfg(feature = "render-vue")]
    use crate::scienceview_vue;
    #[cfg(feature = "wasm")]
    use crate::scienceview_wasm;
}

#[cfg(not(any(
    feature = "render-svg",
    feature = "render-html",
    feature = "render-tui",
    feature = "render-native",
    feature = "render-react",
    feature = "render-vue",
    feature = "render-angular",
    feature = "render-tauri",
    feature = "render-dioxus",
    feature = "wasm",
)))]
#[test]
fn backend_features_are_off_by_default() {
    let enabled: Vec<&str> = BACKEND_FEATURES
        .into_iter()
        .filter(|feature| backend_feature_is_enabled(feature))
        .collect();

    assert!(
        enabled.is_empty(),
        "the default build must not enable any backend, found {enabled:?}"
    );
}

#[test]
fn toolchain_is_pinned_to_the_declared_minimum() {
    let manifest_directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let toolchain_path = manifest_directory.join("rust-toolchain.toml");
    let toolchain = std::fs::read_to_string(&toolchain_path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", toolchain_path.display()));

    assert!(
        toolchain.contains("channel = \"1.85\""),
        "rust-toolchain.toml must pin channel 1.85, found:\n{toolchain}"
    );
    assert!(
        toolchain.contains("\"rustfmt\"") && toolchain.contains("\"clippy\""),
        "rust-toolchain.toml must install rustfmt and clippy, found:\n{toolchain}"
    );
}

#[test]
fn lint_configuration_denies_warnings() {
    let manifest_directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = manifest_directory.join("Cargo.toml");
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", manifest_path.display()));

    assert!(
        manifest.contains("[workspace.lints.rust]"),
        "the workspace must configure rust lints"
    );
    assert!(
        manifest.contains("warnings = \"deny\""),
        "rust warnings must be errors"
    );
    assert!(
        manifest.contains("[workspace.lints.clippy]"),
        "the workspace must configure clippy lints"
    );
    assert!(
        manifest.contains("all = \"deny\""),
        "every clippy lint group must be an error"
    );
}

#[test]
fn readme_presents_the_project_badges_and_workspace_layout() {
    let readme = include_str!("../README.md");
    let badges = [
        "actions/workflows/ci.yml/badge.svg",
        "img.shields.io/crates/v/scienceview.svg",
        "img.shields.io/docsrs/scienceview",
        "img.shields.io/badge/rust-1.85",
    ];

    for badge in badges {
        assert!(
            readme.contains(badge),
            "README.md must carry the `{badge}` badge"
        );
    }

    assert!(
        readme.contains("## Workspace layout"),
        "README.md must describe the workspace layout"
    );
}

#[test]
fn changelog_records_the_workspace_bootstrap() {
    let changelog = include_str!("../CHANGELOG.md");

    assert!(
        changelog.contains("## [Unreleased]"),
        "CHANGELOG.md must keep an Unreleased section"
    );
    assert!(
        changelog.contains("Cargo workspace"),
        "CHANGELOG.md must record the workspace bootstrap"
    );
}
