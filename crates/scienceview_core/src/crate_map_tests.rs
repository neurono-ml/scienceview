//! Tests that keep the crate map and the feature map in the core crate's
//! documentation truthful.

const WORKSPACE_CRATES: [&str; 11] = [
    "scienceview_core",
    "scienceview_svg",
    "scienceview_html",
    "scienceview_tui",
    "scienceview_native",
    "scienceview_wasm",
    "scienceview_react",
    "scienceview_vue",
    "scienceview_angular",
    "scienceview_tauri",
    "scienceview_dioxus",
];

const FEATURES: [&str; 16] = [
    "data-ndarray",
    "spec-serde",
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
    "streaming",
    "streaming-tokio",
    "default",
    "scienceview_core",
];

#[test]
fn crate_map_documents_every_workspace_crate() {
    let documentation = include_str!("lib.rs");

    for crate_name in WORKSPACE_CRATES {
        assert!(
            documentation.contains(crate_name),
            "the core crate documentation must mention `{crate_name}`"
        );
    }

    for feature in FEATURES {
        assert!(
            documentation.contains(feature),
            "the core crate documentation must mention the `{feature}` feature"
        );
    }
}
