# FDR-104 — `SVStepPlot`

| Field | Value |
|---|---|
| Id | FDR-104 |
| Name | `SVStepPlot` |
| Family | Relational |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

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

with the step geometry selected on the line mark. The step shape is a
*geometry* option of the line, not a separate mark, because it produces the
same vertices plus one extra per corner.

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
| `x`, `y` | `SVColumnRef` | — | bokeh, plotly |
| `series` | `Option<SVColumnRef>` | `None` | all three |
| `step` | `SVStepPosition` | `After` | bokeh `Step` (`before`, `after`, `center`), plotly `hv`/`vh`/`hvh`/`vhv` |
| `where` | `SVStepWhere` | `After` | — |
| `line_width` | `SVLength` | token `mark.line_width` | bokeh, plotly |
| `dash` | `Option<SVDashPattern>` | `None` | bokeh, plotly |
| `markers` | `bool` | `true` | bokeh `Step` draws markers by default |
| `aggregate` | `Option<SVStat>` | `None` | — |

`SVStepPosition` has the values `Before`, `After` and `Center`; `SVStepWhere`
selects the horizontal edge the vertical segment is drawn on (`Horizontal`
before the value, `Vertical` after it, and both). The bokeh vocabulary is kept
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

| Backend | Notes |
|---|---|
| `Svg` | a `<path>` with explicit horizontal and vertical segments |
| `Html` | same |
| `Native` / `Wasm` | polyline rasterization |
| `Tui` | the step is drawn with box-drawing characters |
| `React` / `Vue` / `Angular` | one `<path>` per series |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The drawn geometry matches the requested step position exactly.
- [ ] A step over a streaming source extends without redrawing the history.
- [ ] Dense, sparse and streaming inputs give the same steps.
- [ ] An animated step interpolates its corners.

## Open questions

- Should a step chart carry an explicit `interval` column so that each step
      spans a real duration rather than being centred on its sample? *Open —
      it matters for sampled signals and would change the `x` mapping.*
