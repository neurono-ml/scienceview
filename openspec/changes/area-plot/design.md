# Design

## Context

`SVAreaPlot` is one of the scientific-core recipes specified in
`docs/FDRs/relational-area-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Aggregate] -> values -> [SVMove::Stack] -> [SVScale] -> SVMark::Area`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

Implementation correction: `SVMark::Area` closes against a scalar baseline, so
it cannot draw the per-point cumulative baseline a stack produces. A
non-stacked area composes `SVMark::Area`; a stacked area composes
`SVMark::Ribbon` between the cumulative baseline and the cumulative top that
`SVMove::Stack` returns. `SVMove::Stack` is still the only stacking
implementation.

### D2 — Data contract

The chart consumes `x`, `y`, `series`. Dense, sparse and streaming
representations are accepted; the recipe dispatches on the representation once
per operation and never densifies or blocks (patterns.md P6).

Implementation correction: the first preparation expanded every sparse column
into a `Vec<f64>`. The preparation now resolves each column once into a
representation-aware accessor — a dense slice, a sparse view, or a streaming
snapshot — and samples only the logical positions the geometry needs: every
stored sparse entry plus the two positions that border each implicit-zero run.
Zeros inside a run are collinear in data space, so they add no pixel. A
million-slot column with three stored values therefore draws seven vertices,
and the companion allocation probe pins the operation below one mebibyte.
Stacking runs on the same reduced grid, so it does not reintroduce the
logical-length expansion, and a non-finite stored value is still reported under
`SVAreaMissingValuePolicy::Error` before the stack aligns anything.

### D3 — Temporal behaviour

area boundary: interpolates. fill colour: interpolates. stack baseline: interpolates — the areas slide into their stacked positions.

### D4 — Registration and naming

The chart is registered by appending one line to the family module
`src/recipes/relational/mod.rs`. No central registry exists, because a central
registry would serialise every recipe change against every other one
(patterns.md P4).

The public module path follows the project prefix rule:
`sv_recipes::sv_relational::sv_area_plot`, declared with `#[path]` while the
physical folders keep their conventional names. The family re-export carries
only recipe-qualified names; the missing-value policy is
`SVAreaMissingValuePolicy`, so it cannot collide with a sibling recipe at the
family scope.

### D5 — Backend dispatch and scene contract

Every operation composes one backend-agnostic `SVAreaRendering` scene: viewport,
background, and the decoration and mark primitives in final draw order. The
scene is not serialized by the builder and not tied to a concrete backend.

`render()` dispatches on the configured kind. `Svg` replays the scene through
`SVSvgDocument`; `Native` replays it through `SVRasterCanvas`; every other kind
fails at build with `UnsupportedBackendCapability { capability: Export }`
instead of receiving bytes another backend produced. `save()` exports only the
SVG document, because the native backend presents a frame rather than a file.
`show()` remains unsupported: the recipe owns no window.

### D6 — In-place update deferred

The first implementation exposed `update(appended_points)` but ignored the
argument, so the method promised an append-based update it did not perform. The
method is removed. Rendering a streaming source already reads its current
bounded window on every `render()`, and the in-place update path plus its
transition wiring is deferred to the streaming/transition change that can
observe appended points and interpolate between frames.

## Open Questions

- Should `baseline` support a value that varies per `x` (a baseline series), as plotly's `fill` does not but some scientific plots require? *Open.*

