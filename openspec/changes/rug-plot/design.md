# Design

## Context

`SVRugPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-rug-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> values -> [no move] -> [SVScale] -> SVMark::Line (segment geometry)`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

tick position: interpolates. new observation: the tick enters. removed observation: the tick exits.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the rug be composable *into* another recipe — that is, should `SVHistogramPlot` accept a `rug` option that adds rug ticks under the axis? *Open, and probably yes: it is the most common use of the rug.*

