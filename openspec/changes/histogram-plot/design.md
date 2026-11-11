# Design

## Context

`SVHistogramPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-histogram-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Bin] -> (edges, counts) -> [SVMove::Stack | Dodge] -> [SVScale x, y, colour] -> SVMark::Bar`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `series`, `weight`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

bar height: interpolates. bin edge: interpolates when the domain shifts; a bin that changes identity is an exit plus an enter. new bin: enters with the enter rule and grows to its height.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the bin count be derived from the *pixel* width of the axis, or from the data only? *Current answer: derived from the available width, with the data range as the other input — the layout engine owns the width, the stat owns the range.*
- Should `shrink` be a mark option rather than a recipe option? *Current answer: a recipe option, since it changes the bar width, and the bar mark carries a width that the recipe computes.*

