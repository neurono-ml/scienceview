# FDR-103 — `SVAreaPlot`

| Field | Value |
|---|---|
| Id | FDR-103 |
| Name | `SVAreaPlot` |
| Family | Relational |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

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
| `interpolate` | `SVLineInterpolation` | `Linear` | plotly `line_shape` |

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

- [ ] Areas are drawn against the configured baseline.
- [ ] With stacking enabled, the areas tile without gaps or overlaps.
- [ ] Dense, sparse and streaming inputs give the same areas.
- [ ] An animated stack shows the areas sliding into their stacked positions.

## Open questions

- Should `baseline` support a value that varies per `x` (a baseline series),
      as plotly's `fill` does not but some scientific plots require? *Open.*
