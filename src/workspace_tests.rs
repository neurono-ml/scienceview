//! Workspace smoke tests.
//!
//! The root `scienceview` package is a facade over the workspace: the core
//! crate must be reachable through it, and no rendering backend may be enabled
//! by the default feature set.

/// Every backend feature declared by the root package.
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
