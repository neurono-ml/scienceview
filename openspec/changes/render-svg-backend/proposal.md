# Proposal

## Why

The SVG backend is the first rendering target and the one the documentation
generates its figures with. It is also the simplest proof that the architecture
holds: a self-contained output, no dependencies, no clock, driven only by the
layout tree and the marks.

## What Changes

- Introduce `scienceview_svg`, a zero-dependency crate implementing `SVRenderBackend`.
- Render the layout tree, every mark variant and every decoration as self-contained SVG.
- Compose large point sets into single paths to keep the document small.
- Become the figure-generation contract used by the `examples/` and the `figures` CI job.

## Capabilities

### New Capabilities
- `render-svg-backend`: render svg backend.

### Modified Capabilities

None.

## Impact

- New crate `crates/scienceview_svg`.
- Every recipe change generates its documentation figure through this backend.
- No dependency may be added to it: self-containment is a capability, not a preference.
