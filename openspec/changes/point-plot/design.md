# Design

## Context

`SVPointPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-point-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate + interval] -> (estimate, lower, upper) per category -> [SVMove::Dodge] -> [SVScale] -> SVMark::Dot + error bar + optional Line`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `category`, `value`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

point position: interpolates. error bar extent: interpolates. connecting line: interpolates along the category axis.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the error bar be a mark of its own (`SVErrorBar`) shared with the bar plot and the line plot, rather than part of the dot mark? *Current answer: yes — an error bar is a geometric primitive that several recipes need, so it belongs in `core-mark`. This is a change to FDR-001.*

