# Design

## Context

`SVClusteredHeatmapPlot` is one of the scientific-core recipes specified in
`docs/FDRs/matrix-clustered-heatmap-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate (matrix)] -> matrix -> [clustering: reorder rows and columns] -> [SVScale::Continuous over a colour map] -> SVMark::Image + dendrogram marks`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `Matrix`, `Long table`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

cell colour: interpolates. cell position: interpolates when the clustering changes — this is the showpiece: the matrix visibly re-orders. dendrogram: redraws with the new clustering.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/matrix/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Which hierarchical clustering implementation is used — a bundled one, or delegated to `sciencekit`? *Current answer: delegated to `sciencekit`, which is the sibling project that "learns the models"; `scienceview` draws them. The dependency is behind a Cargo feature.*

