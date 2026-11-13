//! Tests for the embedded runtime contract (spec `render-html-backend`).

use super::{
    SV_HTML_ACTION_EVENT, SV_HTML_DELTA_EVENT, SV_HTML_RUNTIME, SV_HTML_UPDATE_EVENT,
    sv_html_runtime_byte_length,
};

#[test]
fn the_runtime_is_small_and_dependency_free() {
    assert!(sv_html_runtime_byte_length() > 0);
    assert!(
        sv_html_runtime_byte_length() < 8_192,
        "the inlined runtime must stay small, got {}",
        sv_html_runtime_byte_length()
    );
    assert!(!SV_HTML_RUNTIME.contains("import "), "no module imports");
    assert!(!SV_HTML_RUNTIME.contains("http://"), "no external script");
    assert!(!SV_HTML_RUNTIME.contains("https://"), "no external script");
    assert!(
        !SV_HTML_RUNTIME.contains("</script>"),
        "the runtime cannot close its own element"
    );
}

#[test]
fn a_gesture_becomes_an_action_and_a_delta_is_applied() {
    assert!(SV_HTML_RUNTIME.contains("pointerdown"), "{SV_HTML_RUNTIME}");
    assert!(
        SV_HTML_RUNTIME.contains(&format!("\"{SV_HTML_ACTION_EVENT}\"")),
        "{SV_HTML_RUNTIME}"
    );
    assert!(SV_HTML_RUNTIME.contains("applyDelta"), "{SV_HTML_RUNTIME}");
    assert!(
        SV_HTML_RUNTIME.contains("scienceview:delta"),
        "{SV_HTML_RUNTIME}"
    );
    assert!(
        SV_HTML_RUNTIME.contains("view.x += Number(action.dx)"),
        "{SV_HTML_RUNTIME}"
    );
    assert!(
        SV_HTML_RUNTIME.contains("canvas.style.transform"),
        "{SV_HTML_RUNTIME}"
    );
    assert_eq!(SV_HTML_DELTA_EVENT, "scienceview:delta");
}

#[test]
fn an_update_is_applied_in_place_without_a_reload() {
    assert!(
        SV_HTML_RUNTIME.contains(&format!("\"{SV_HTML_UPDATE_EVENT}\"")),
        "{SV_HTML_RUNTIME}"
    );
    assert!(
        SV_HTML_RUNTIME.contains("canvas.innerHTML = payload.markup"),
        "{SV_HTML_RUNTIME}"
    );
    assert!(
        SV_HTML_RUNTIME.contains("MutationObserver"),
        "{SV_HTML_RUNTIME}"
    );
    assert!(
        !SV_HTML_RUNTIME.contains("location.reload"),
        "an update never reloads"
    );
}

#[test]
fn the_animation_frame_only_drives_redraws() {
    assert!(
        SV_HTML_RUNTIME.contains("requestAnimationFrame"),
        "{SV_HTML_RUNTIME}"
    );
    assert!(
        SV_HTML_RUNTIME.contains("payload.clock.reduced_motion"),
        "{SV_HTML_RUNTIME}"
    );
    assert!(
        SV_HTML_RUNTIME.contains("data-scienceview-revision"),
        "{SV_HTML_RUNTIME}"
    );
}
