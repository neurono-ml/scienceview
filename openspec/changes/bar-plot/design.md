# Design

## Context

`SVBarPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-bar-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate] -> (value per category, optional interval) -> [SVMove::Dodge | Stack] -> [SVScale] -> SVMark::Bar (+ error bars)`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `category`, `value`, `series`, `weight`; weights are accepted by `Mean` and `Sum` only and rejected at build time for every other estimator. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

bar height: interpolates. error bar extent: interpolates. colour: interpolates.
The change produces transition-ready geometry but ships no animation driver; the
driver and the slide-into-stacked-position behaviour belong to the transition
integration change.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

### D5 — Options beyond the FDR table's first draft

The FDR configuration table was extended to match the shipped surface:
`arrangement` (`Dodge`/`Stack`), `ordering` (`Data`/`Ascending`/`Descending`),
`missing_value_policy` (`Omit`/`Error`), `confidence_interval`, the labels, the
titles and the viewport. `SVFillPattern` does not exist in the core kernel, so
the pattern option is explicitly deferred rather than invented inside the recipe
(patterns P5).

## Open Questions

- Should the aggregate be mandatory, or should a bar chart accept raw values and draw one bar per observation? *Current answer: the aggregate is mandatory; the raw case is the strip plot.*

