# Design

## Context

`SVJointPlot` is one of the scientific-core recipes specified in
`docs/FDRs/multivariate-joint-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> values -> central cell: SVMark::Dot (or a density contour) -> marginal cells: the distribution recipe for the variable type -> [SVScale per axis] -> three panels sharing a layout`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `x`, `y`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

point position: interpolates in the central panel. marginal distribution: re-computed per frame and morphed. kind change: the central panel swaps between the scatter, the contour and the fit.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/multivariate/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the joint plot be a figure with three panels (one figure, three layout regions) or three figures composed by the dashboard? *Open — the answer determines whether the recipe owns a layout or borrows one.*

