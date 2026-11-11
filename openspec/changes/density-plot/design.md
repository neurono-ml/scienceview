# Design

## Context

`SVDensityPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-density-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::KernelDensity] -> (grid, density) -> [no move] -> [SVScale] -> SVMark::Line | SVMark::Area`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `series`, `weight`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

density curve: interpolates point by point over the fixed grid. fill: interpolates with the curve. bandwidth change: the curve morphs into the new estimate.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the bandwidth be chosen automatically by a better rule (for example, a plug-in selector), and if so is that a different kernel? *Open.*
- Should the density kernel support a weighted variant natively, or through the `weight` column? *Current answer: through the column — the kernel takes weights rather than growing a second variant.*

