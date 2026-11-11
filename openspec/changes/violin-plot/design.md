# Design

## Context

`SVViolinPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-violin-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::KernelDensity per category] -> (grid, density per category) -> [SVMove::Dodge] -> [SVScale] -> SVMark::Area`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `category`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

violin silhouette: interpolates; the width at each grid point moves. fill colour: interpolates. category added: the violin enters and grows to its width.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the violin support "split" mode, where two series share one category and are drawn against each other? *Open — plotly supports it; it is a variant of the dodge move.*

