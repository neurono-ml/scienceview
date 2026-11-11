# Design

## Context

`SVParallelCoordinatesPlot` is one of the scientific-core recipes specified in
`docs/FDRs/multivariate-parallel-coordinates-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> values -> [SVMove::Stack? no — one line per observation] -> [SVScale per axis] -> SVMark::Line`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `variables`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

line vertices: interpolate. line colour: interpolates. observation entering or exiting: the polyline enters or exits.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/multivariate/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the axes be reorderable by dragging, and should that be an action in the UI kit? *Open.*

