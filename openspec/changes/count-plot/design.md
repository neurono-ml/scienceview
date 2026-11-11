# Design

## Context

`SVCountPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-count-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Count] -> (count per category) -> [SVMove::Stack | Dodge] -> [SVScale] -> SVMark::Bar`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `category`, `series`, `weight`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

bar height: interpolates. category ordering change: the bars slide to their new positions. new category: enters.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the count support a `normalise` that divides by the category total rather than the grand total (a conditional proportion)? *Open.*

