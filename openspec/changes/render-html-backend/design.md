# Design

## Context

The HTML backend is where the project's two audiences meet: a scientist
wants a single file they can open, and a dashboard wants a page that updates.
Both are the same document with different runtime behaviour.

## Decisions

### D1 — Self-contained, still zero dependencies

CSS and the small runtime are inlined. A self-contained page is a capability, not a convenience: it is what makes the output portable.

### D2 — The Rust model stays the authority

The embedded runtime forwards gestures to the host as actions and applies the view transform they imply. It never re-derives geometry or owns figure data: any change to the figure itself arrives as a fresh payload from the Rust model, consistent with the foundation's Rust-is-the-brain posture.

### D3 — The stylesheet is emitted and overridable

Theme rules are emitted in the document, and the user's own rules can override them, which is the documented theming contract.

### D4 — Animation uses the model clock

The browser's animation frame only drives redraws; the playback position comes from the model.

## Risks / Trade-offs

- An embedded runtime means JavaScript in the output; it stays small and dependency-free.

