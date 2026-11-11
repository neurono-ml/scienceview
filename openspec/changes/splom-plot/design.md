# Design

## Context

`SVSplomPlot` is one of the scientific-core recipes specified in
`docs/FDRs/multivariate-splom-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [no stat] -> values -> [SVScale per axis] -> SVMark::Dot, once per cell`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `variables`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

point position: interpolates in every cell. highlight: the highlighted observation moves in every cell at once. variable added: a row and a column are inserted and the grid re-flows.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/multivariate/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the splom share one `SVFigure` with sub-panels, or compose several figures? *Open — this touches the layout engine and the dashboard capability; the grid is arguably a dashboard concern, and this recipe is the single-cell-level description of it.*

