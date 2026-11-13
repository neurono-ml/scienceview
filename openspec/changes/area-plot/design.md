# Design

## Context

`SVAreaPlot` is one of the scientific-core recipes specified in
`docs/FDRs/relational-area-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate] -> values -> [SVMove::Stack] -> [SVScale] -> SVMark::Area`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

Implementation correction: `SVMark::Area` closes against a scalar baseline, so
it cannot draw the per-point cumulative baseline a stack produces. A
non-stacked area composes `SVMark::Area`; a stacked area composes
`SVMark::Ribbon` between the cumulative baseline and the cumulative top that
`SVMove::Stack` returns. `SVMove::Stack` is still the only stacking
implementation.

### D2 — Data contract

The chart consumes `x`, `y`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

area boundary: interpolates. fill colour: interpolates. stack baseline: interpolates — the areas slide into their stacked positions.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/relational/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should `baseline` support a value that varies per `x` (a baseline series), as plotly's `fill` does not but some scientific plots require? *Open.*

