# Design

## Context

`SVHeatmapPlot` is one of the scientific-core recipes specified in
`docs/FDRs/matrix-heatmap-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate (pivot into a matrix)] -> (rows, columns, values) -> [SVScale::Continuous over a colour map] -> SVMark::Image | SVMark::Rect`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `Matrix`, `Long table`, `Sparse matrix`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

cell colour: interpolates in the colour map's space. cell entering or leaving: enters or exits. matrix growing: the new row or column enters.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/matrix/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the cell geometry be a dedicated `SVMark::Cell` rather than the generic `Image` mark? *Current answer: the `Image` mark with an explicit colour map covers it, but a cell mark with an optional border is simpler for the terminal backend — open.*
- Should the pivot operation be a stat, or should the recipe accept a pre-pivoted matrix only? *Current answer: both; the stat performs the pivot for long tables.*

