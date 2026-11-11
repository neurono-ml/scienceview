//! Tests for the closed backend kind enumeration.
//!
//! The selection must offer every supported backend and never an automatic or
//! default variant (spec `backend-selection`).

use crate::sv_backend::SVBackendKind;

#[test]
fn every_supported_backend_kind_is_offered() {
    let names: Vec<&str> = SVBackendKind::ALL
        .iter()
        .map(|backend_kind| backend_kind.as_str())
        .collect();

    assert_eq!(
        names,
        [
            "svg", "html", "tui", "native", "wasm", "react", "vue", "angular", "tauri", "dioxus"
        ]
    );
}

#[test]
fn there_is_no_automatic_or_default_variant() {
    for spelling in ["automatic", "auto", "default", "Automatic", "Default"] {
        assert!(
            SVBackendKind::from_name(spelling).is_none(),
            "`{spelling}` must not resolve to a backend kind"
        );
    }

    assert!(
        SVBackendKind::ALL
            .iter()
            .all(|backend_kind| backend_kind.as_str() != "automatic")
    );
}

#[test]
fn every_kind_round_trips_through_its_name() {
    for backend_kind in SVBackendKind::ALL {
        assert_eq!(
            SVBackendKind::from_name(backend_kind.as_str()),
            Some(backend_kind)
        );
        assert_eq!(backend_kind.to_string(), backend_kind.as_str());
    }
}

#[test]
fn an_unknown_name_is_none_rather_than_a_fallback() {
    assert_eq!(SVBackendKind::from_name("gpu"), None);
    assert_eq!(SVBackendKind::from_name(""), None);
}
