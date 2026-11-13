# Design

## Context

`SVBoxPlot` is one of the scientific-core recipes specified in
`docs/FDRs/recipes/distribution-box-plot.md`. This change implements exactly
that record on top of the core kernels already landed by the earlier waves, and
touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Percentile + outlier extraction] -> [SVMove::Dodge] -> [SVScale] -> SVMark::Ribbon | SVMark::Line | SVMark::Dot`. The mark vocabulary is closed and has no `Box` variant, so the FDR's `SVMark::Box` shorthand is realised with the existing marks: the ribbon draws the filled body,
lines draw the median, whiskers and caps, and dots draw the outliers and the mean. It introduces no new mark, stat, move or scale variant; if implementation reveals a missing variant, that is a change to the core capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `category`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

The geometry is interpolable, but this change ships no transition API. The
temporal behaviour recorded in the FDR — the box extent, whisker ends, median
line, individual outlier entry and exit, and category entry and exit — is
provided by the transition and animation integration in a later change.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

### D5 — Notch and missing policy

A notched box is three stacked ribbon segments around the median, using the
conventional `1.58 · IQR / √n` half-width clamped to half the interquartile
range. It is backend-agnostic because it uses the same ribbon primitive as the
plain box; no new mark variant is added. Rows with a non-finite value or
category coding are skipped, an input with no surviving row fails with
`EmptyInput`, and a sparse column contributes only its stored entries.

## Open Questions

- Should the box plot support a `notch` shape in every backend? *Resolved: the
  notch uses three ribbon primitives and therefore renders wherever the plain
  box renders.*

