# Design

## Context

`SVScatterPlot` is one of the scientific-core recipes specified in
`docs/FDRs/relational-scatter-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> values -> [SVMove::Dodge | Jitter] -> [SVScale x, y, colour] -> SVMark::Dot`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `x`, `y`, `series`, `size`, `colour`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

marker position: interpolates. marker size and colour: interpolate. marker shape: swaps.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/relational/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should `SVScatterPlot` accept a matrix of columns and draw a scatterplot matrix, or is that the separate `SVSplomPlot` recipe? *Current answer: `SVSplomPlot` — a matrix of scatters is a different composition, not a mode of this one.*

