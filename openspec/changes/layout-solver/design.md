# Design

## Context

The layout engine is what makes ten backends share one geometry. The design
follows the foundation decision: solve in logical pixels, rasterize per grid.

## Decisions

### D1 — Two passes, one tree

Measure collects intrinsic sizes from the content; solve distributes the viewport against constraints. The output is a layout tree that every backend reads. A backend that re-derives a position is a bug.

### D2 — `SVLength` is exhaustive

Pixels, fraction of parent, intrinsic, aspect ratio and fill, with minimum and maximum bounds. Responsiveness is a declared mode, not an emergent behaviour.

### D3 — The viewport is the only environment input

A backend reports size, device pixel ratio or cell grid. Nothing else about the environment leaks into the solver.

### D4 — Terminal rounding happens at rasterization

The solved tree is continuous; the terminal snaps to cells at the last step, so a coarser grid never changes the logical layout.

## Risks / Trade-offs

- Two-pass layout can disagree with font metrics; backends report measurement constraints through the viewport to reduce the gap.

