# Design

## Context

`SVParallelCategoriesPlot` is one of the scientific-core recipes specified in
`docs/FDRs/multivariate-parallel-categories-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> values -> [SVScale per axis (nominal bands)] -> SVMark::Ribbon`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `variables`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

ribbon geometry: interpolates. ribbon colour: interpolates. a category appearing on an axis: the bands re-space and the ribbons re-shape.

### D4 — Aggregation

Every axis is counted once through `SVStat::Count`, which also applies the
category order. A ribbon is not drawn per observation: the recipe derives one
group code per observation from the two adjacent-axis category ranks (and the
optional colour rank), counts that derived column through the same kernel, and
draws one ribbon per observed combination. The slice thickness at each band is
the ribbon's count divided by the total count on that side of the band, so the
slices tile the band and encode the count. The recipe never counts observations
itself.

### D5 — Registration

The chart is registered by appending one
line to `src/recipes/multivariate/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the ribbon thickness encode anything, or is it purely a connection? *Open — plotly's parcats does not vary it.*

