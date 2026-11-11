# Design

## Context

`SVBoxPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-box-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate (quantiles) + outlier extraction] -> [SVMove::Dodge] -> [SVScale] -> SVMark::Box | SVMark::Dot (outliers)`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `category`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

box extent: interpolates (the quartiles move). whisker ends: interpolate. median line: interpolates.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the box plot support a `notch` shape in every backend? *Open — the notch requires a confidence interval on the median, which is an extra statistical assumption.*

