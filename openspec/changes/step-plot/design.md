# Design

## Context

`SVStepPlot` is one of the scientific-core recipes specified in
`docs/FDRs/relational-step-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale] -> SVMark::Line`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `x`, `y`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

step vertices: interpolate — a step's corners move with the values. corner position: moves with the value; the corner's horizontal edge stays on the previous value. new sample: enters, and the step extends to it.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/relational/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should a step chart carry an explicit `interval` column so that each step spans a real duration rather than being centred on its sample? *Open — it matters for sampled signals and would change the `x` mapping.*

