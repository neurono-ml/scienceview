# FDR-103 — `SVAreaPlot`

| Field | Value |
|---|---|
| Id | FDR-103 |
| Name | `SVAreaPlot` |
| Family | Relational |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | `docs/src/development-explained/ch18-area-plot.md` |
| Example program | `examples/area_plot.rs` |

## Purpose

`SVAreaPlot` fills the region between a line and a baseline. It shows
magnitude over an ordered axis, and it is the base form of a stacked area
chart.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `objects.Area` | <https://seaborn.pydata.org/api.html#seaborn.objects.Area> |
| bokeh | `VArea` / `HArea` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.VArea> |
| plotly | `go.Scatter` with `fill` | <https://plotly.com/python/reference/#plotly.graph_objects.Scatter> |

## Recipe

```
data -> [SVStat::Aggregate] -> values -> [SVMove::Stack] -> [SVScale] -> SVMark::Area
```

The stacking move is optional. Without it, each area is drawn against the
baseline; with it, the areas are stacked and the baseline of each is the
cumulative sum of the previous ones.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | ordering axis | continuous or temporal | yes |
| `y` | value | continuous | yes |
| `series` | one area each, stacked in this order | nominal | no |

Accepted representations: dense, sparse, streaming (windowed, because
stacking needs the group total).

### Match key

`Category` on the `x` value within a series.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` | `Continuous` or `Temporal` |
| y | `y` | `Continuous` |
| fill colour | `series` | `Nominal` |
| opacity | constant, usually translucent | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x`, `y` | `SVColumnRef` | — | all three |
| `series` | `Option<SVColumnRef>` | `None` | all three |
| `stack` | `bool` | `false` | seaborn `Stack`, plotly `stackgroup` |
| `baseline` | `SVBaselineMode` | `Zero` | plotly `fill` (`tozeroy`, `tonexty`, `toself`) |
| `opacity` | `f32` | token `mark.area_opacity` | all three |
| `line` | `bool` | `true` | plotly `line` |
| `interpolate` | `SVAreaInterpolation` | `Linear` | plotly `line_shape` |

## Theme

Consumes `mark.area_opacity`, `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| area boundary | interpolates |
| fill colour | interpolates |
| stack baseline | interpolates — the areas slide into their stacked positions |
| series entering the stack | enters and the areas above it shift up |

The stacking move makes this the clearest example of a transition that is
about *position* rather than about data: when a series is added, every area
above it moves, and the animation shows that movement.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one closed `<path>` per area |
| `Html` | same |
| `Native` / `Wasm` | filled polygon rasterization with even-odd winding |
| `Tui` | filled cells with background colours, or a shaded character ramp |
| `React` / `Vue` / `Angular` | one `<path>` per area |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] Areas are drawn against the configured baseline.
      Evidence: `the_configured_baseline_moves_the_area`.
- [x] With stacking enabled, the areas tile without gaps or overlaps.
      Evidence: `the_stacked_areas_tile_against_the_cumulative_baseline`; the
      cumulative baseline is the upper boundary of the lower series.
- [x] Dense, sparse and streaming inputs give the same areas.
      Evidence: `dense_sparse_and_streaming_representations_agree`.
- [ ] An animated stack shows the areas sliding into their stacked positions.
      The render operation recomputes the stack and the areas take their new
      positions on each frame; the transition kernel owns the interpolation
      between frames, which this recipe does not yet wire.

## Amendment

The recipe shipped with one composition correction. `SVMark::Area` closes
against a scalar baseline (`Zero`, `Minimum`, `Maximum` or `Value`), so it
cannot express the per-point cumulative baseline a stack produces. A stacked
area therefore composes `SVMark::Ribbon` between the cumulative baseline and
the cumulative top produced by `SVMove::Stack`; a non-stacked area composes
`SVMark::Area` against the configured baseline. The stack itself is never
reimplemented. A per-point baseline on the area mark would be a core change
with its own spec delta.

## Open questions

- Should `baseline` support a value that varies per `x` (a baseline series),
      as plotly's `fill` does not but some scientific plots require? *Open.*
