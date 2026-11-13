# Design

## Context

`SVStripPlot` is one of the scientific-core recipes specified in
`docs/FDRs/recipes/distribution-strip-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> values -> [SVMove::Jitter (+ Dodge)] -> [SVScale] -> SVMark::Dot`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `category`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

point position: the value interpolates; the jitter offset does not — it is recomputed from the match key, so the point does not drift between frames. colour and size: interpolate. new observation: the point enters.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the jitter be applied in data space or display space? *Open — it is the same question as in `SVMove`, and the answer affects whether the jitter survives a change of axis range.*

