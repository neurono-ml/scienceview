# Proposal

## Why

The `layout-engine` capability requires a resolution-independent measure/solve
pipeline so that the same figure behaves predictably on a browser, a window and
a terminal cell grid. Without it every backend would invent its own geometry,
and the terminal rendering would drift from the SVG one.

## What Changes

- Introduce `SVLength` (pixels, fractions, intrinsic, aspect ratio, fill) and the constraints model.
- Implement the two-pass measure/solve pipeline over a backend-supplied viewport.
- Implement the responsiveness modes: fixed, fit-to-parent, aspect-ratio constrained, content derived.
- Invalidate and re-solve the layout tree on viewport change.

## Capabilities

### New Capabilities
- `layout-solver`: layout solver.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/layout/`.
- Backends rasterize the resolved tree; they never re-derive positions.
- Decoration placement and tick counts consume the solved tree.
