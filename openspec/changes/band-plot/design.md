# Design

## Context

`SVBandPlot` is one of the scientific-core recipes specified in
`docs/FDRs/relational-band-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate + error] -> (lower, upper) -> [no move] -> [SVScale] -> SVMark::Ribbon`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `x`, `lower`, `upper`, `series`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour is deferred

The FDR records that the boundary interpolates, the fill colour and opacity
interpolate, the band enters and the interval widens from its centre. This
change composes interpolable `Ribbon` and `Line` paths but ships **no**
transition or animation API, and no test claims one. The transition and
animation integration owns that surface in a later change; the FDR acceptance
item is deliberately unticked.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/relational/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

### D5 — Derived mode is a pair, not an arbitrary stat

Only `SVStat::Aggregate` reports both an estimate and an uncertainty interval,
which are the two boundaries a band consumes. The builder therefore selects the
derived mode explicitly (`derive()` plus `values(...)`) and constructs the
aggregate from `error` and `confidence`; a caller-supplied arbitrary `SVStat`
would have to be inspected at render time and could silently produce no band.

### D6 — Missing values are explicit

A non-finite boundary is a missing sample. `SVBandMissingValuePolicy::Omit`
(default) drops the sample and splits the band at the gap so the fill never
bridges absent data; `SVBandMissingValuePolicy::Error` rejects the render with
an explicit configuration error naming the column and position. The derived
mode applies the same policy before handing observations to
`SVStat::Aggregate`, which would otherwise reject a non-finite value itself.

### D7 — Sparse columns contribute their stored entries

Column access is representation-aware: a dense column contributes every
logical position, a sparse column contributes only its stored indices and a
streaming column contributes its current window. Neither the grouped explicit
path nor the derived path expands a sparse column into a dense `Vec`. The
`points` span field counts the finite samples actually drawn, so an omitted
boundary is not counted and a sparse column of ten million logical values with
three stored entries reports three points.

### D8 — The native serializer reuses the composed geometry

`render` returns backend-agnostic mark and decoration primitives. `to_svg`
writes them with `SVSvgDocument`; `to_native_frame` draws the very same
primitives with `SVRasterCanvas` and returns the frame. Each serializer checks
the configured kind and fails with a typed error when asked for the wrong one,
so the configured kind is never ignored. `Native` declares no export
capability, so `save` reports `UnsupportedBackendCapability`.

### D9 — Build-time validation

The x scale (continuous or temporal), the column existence and the column
roles are validated in `build`. A nominal or boolean x axis and a
missing/wrong-role column therefore fail before any data is read (patterns P7).

### D10 — Public naming

The recipe folder keeps its short physical path and is exposed through
`#[path]` as `scienceview::sv_recipes::sv_relational::sv_band_plot`. Recipe
items are not re-exported into the family scope, and the missing-value policy
is named `SVBandMissingValuePolicy`, so it cannot collide with a sibling
recipe's policy.

## Open Questions

- Should the band expose a `confidence` level that changes the interval interactively, and should that be a control in the UI kit? *Open — it would be a natural companion to the range slider.*

