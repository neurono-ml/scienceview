# FDR-104 — `SVStepPlot`

| Field | Value |
|---|---|
| Id | FDR-104 |
| Name | `SVStepPlot` |
| Family | Relational |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [`ch19 — Step plots`](../../src/development-explained/ch19-step-plot.md) |
| Example program | [`examples/step_plot.rs`](../../../examples/step_plot.rs) |

## Purpose

`SVStepPlot` draws a value that changes in discrete steps along an ordered
axis. It is the correct form for quantities that are held constant between
events — a sampled signal, a state machine trace, a price that updates per
interval — where a sloped line would imply interpolation that does not exist.

## Sources

| Library | Function | Link |
|---|---|---|
| bokeh | `Step` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.Step> |
| plotly | `go.Scatter` with `line_shape` `hv`/`vh`/`hvh`/`vhv` | <https://plotly.com/python/reference/#plotly.graph_objects.Scatter> |
| seaborn | — (no dedicated function) | <https://seaborn.pydata.org/api.html> |

## Recipe

```
data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale] -> SVMark::Line
```

with the step geometry expanded by the recipe into explicit line vertices.
The step shape is a *geometry* expansion of the line, not a separate mark,
because it produces the same vertices plus one extra per corner.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | ordering axis | continuous or temporal | yes |
| `y` | value, held constant until the next `x` | continuous | yes |
| `series` | one step line each | nominal | no |

Accepted representations: dense, sparse, streaming.

### Match key

`Category` on the `x` value within a series.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` | `Continuous` or `Temporal` |
| y | `y` | `Continuous` |
| colour | `series` | `Nominal` |
| line width | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x`, `y` | named column | — | bokeh, plotly |
| `series` | `Option<named column>` | `None` | all three |
| `step` | `SVStepPosition` | `After` | bokeh `Step` (`before`, `after`, `center`), plotly `hv`/`vh`/`hvh`/`vhv` |
| `line_width` | `SVLength` | token `mark.line_width` | bokeh, plotly |
| `dash` | `Option<SVDashPattern>` | `None` | bokeh, plotly |
| `markers` | `bool` | `true` | bokeh `Step` draws markers by default |
| `aggregate` | `Option<SVStat>` | `None` | — |
| `duplicate_policy` | `SVStepDuplicatePolicy` | `LastWins` | — |
| `x_scale` | `SVScaleKind` | `Continuous` | the `x` type is continuous or temporal |

`SVStepPosition` has the values `Before`, `After` and `Center`. The three
values fully express the pre/post/mid geometry, so the provisional
`SVStepWhere` column was folded into `SVStepPosition` rather than shipping a
second control that selects the same corner. The bokeh vocabulary is kept
because it names the geometry precisely.

## Theme

Consumes `mark.line_width`, `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| step vertices | interpolate — a step's corners move with the values |
| corner position | moves with the value; the corner's horizontal edge stays on the previous value |
| new sample | enters, and the step extends to it |
| colour, width | interpolate |

## Backend notes

### Recipe-level scene contract

Every operation resolves the source columns once and composes an
`SVStepPlotComposition` — the scales, the expanded step vertices and the
decoration primitives. The scene is backend-neutral and owns no pixels. The
configured `SVBackendKind` selects the serializer per operation: this change
ships the SVG serializer, so `render`, `update` and `save` return
`SVError::BackendUnavailable` for every kind whose serializer does not exist
yet. `build()` accepts any explicit kind because the scene does not depend on
it; no operation degrades silently to SVG.

| Backend | Notes |
|---|---|
| `Svg` | a `<path>` with explicit horizontal and vertical segments |
| `Html` | same scene, through the HTML serializer that a later change ships |
| `Native` / `Wasm` | polyline rasterization of the same vertices |
| `Tui` | the step is drawn with box-drawing characters |
| `React` / `Vue` / `Angular` | one `<path>` per series |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The drawn geometry matches the requested step position exactly —
      `option_tests::each_step_position_expands_different_vertices`.
- [ ] A step over a streaming source extends without redrawing the history.
      Deferred: `update` re-resolves the source and recomposes the current
      window (`svg_tests::update_rerenders_the_current_window_after_a_live_append`),
      so an appended sample changes the output and preserves the prior samples,
      but this change ships no incremental append buffer. The incremental path
      waits for the transition integration.
- [x] Dense, sparse and streaming inputs give the same steps —
      `parity_tests::dense_sparse_and_streaming_agree`.
- [ ] An animated step interpolates its corners. Deferred to the transition
      integration: the expansion already turns every corner into an explicit
      `SVMark::Line` vertex, but this change ships no transition API or
      transition test. The animation chapter will consume the same vertices.

## Implementation

- Module: `scienceview::sv_recipes::sv_relational::sv_step_plot`, stored under
  `src/recipes/relational/step_plot/` and exposed through `#[path]`
  (`builder/`, `core_implementation/`, `extraction/`, `composition.rs`,
  `decorations.rs`, `render_svg.rs`).
- Tests: `src/recipes/relational/step_plot/step_plot_tests/` — options, data
  contract, parity, concurrency, SVG and observability.
- Observability: each operation span records the configured backend kind, the
  representation, the number of observations the scene actually draws (not the
  logical column length) and the duration.
- Example: `examples/step_plot.rs` → `docs/figures/step_plot.svg`.
- Chapter: `docs/src/development-explained/ch19-step-plot.md`.

## Open questions

- Should a step chart carry an explicit `interval` column so that each step
      spans a real duration rather than being centred on its sample? *Open —
      it matters for sampled signals and would change the `x` mapping.*
