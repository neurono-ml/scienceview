# FDR-101 — `SVScatterPlot`

| Field | Value |
|---|---|
| Id | FDR-101 |
| Name | `SVScatterPlot` |
| Family | Relational |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | `docs/src/development-explained/ch16-scatter-plot.md` |
| Example program | `examples/scatter_plot.rs` |

## Purpose

`SVScatterPlot` shows the joint distribution of two continuous variables, with
optional grouping, sizing and colouring. It is the most basic chart in the
library and the reference point for how a recipe is composed: a dot mark, a
scale per axis, a move for grouping, and nothing else.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `scatterplot` | <https://seaborn.pydata.org/api.html#seaborn.scatterplot> |
| seaborn | `objects.Plot` + `Dot` | <https://seaborn.pydata.org/api.html#seaborn.objects.Dot> |
| bokeh | `figure.scatter` / `Circle` | <https://docs.bokeh.org/en/latest/docs/gallery/scatter/scatter.html> |
| plotly | `go.Scatter` (markers) | <https://plotly.com/python/reference/#plotly.graph_objects.Scatter> |

## Recipe

```
data -> [no stat] -> values -> [SVMove::Dodge | Jitter] -> [SVScale x, y, colour] -> SVMark::Dot
```

The stat is absent: a scatter shows the data as it is. Grouping is expressed
as a nominal colour scale, and optional jitter is the only move.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | horizontal position | continuous | yes |
| `y` | vertical position | continuous | yes |
| `series` | grouping, colour and legend entry | nominal | no |
| `size` | marker area | continuous | no |
| `colour` | explicit colour override | any | no |

Accepted representations: dense, sparse, streaming. For a streaming source
the scatter shows the rolling window and updates in place; no special case is
needed because a scatter performs no aggregation.

### Match key

`Identity` when an identifier column is given, `Category` when a categorical
column is given, otherwise `Index`. A scatter of raw samples has no natural
identity, so an animated scatter must be given one — the default is `Index`,
which is stable only if the data order is stable.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` column | `Continuous` |
| y | `y` column | `Continuous` |
| colour | `series` column | `Nominal` over the series palette |
| size | `size` column | `Continuous` |
| opacity | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x`, `y` | `SVColumnRef` | — | all three |
| `series` | `Option<SVColumnRef>` | `None` | all three |
| `size` | `Option<SVColumnRef>` | `None` | seaborn, plotly |
| `colour` | `Option<SVColumnRef>` | `None` | seaborn, plotly |
| `marker` | `SVDotShape` | `Circle` | bokeh, plotly |
| `size_scale` | `Option<SVSizeScale>` | `None` | plotly `sizemode` |
| `alpha` | `f32` | `1.0` | all three |
| `edge_colour` | `Option<SVColour>` | `None` | bokeh `line_color` |
| `edge_width` | `SVLength` | `0` | bokeh `line_width` |
| `jitter` | `Option<SVMove>` | `None` | seaborn |
| `legend` | `SVLegendPlacement` | `Auto` | all three |
| `title`, `x_label`, `y_label` | `String` | empty | all three |

## Theme

Consumes `mark.size`, `mark.palette`, `text.font`, `text.size`, and the axis
tokens. The series palette is a theme token, so a theme change restyles every
scatter in a figure at once.

## Temporal

| Property | Behaviour |
|---|---|
| marker position | interpolates |
| marker size and colour | interpolate |
| marker shape | swaps |
| new series | enters with the enter rule; the legend entry enters with it |
| removed series | exits; the legend entry exits with it |

A scatter is the archetypal animated chart: points move, grow and fade
between states. It requires a match key to do so meaningfully.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one element per point; a large scatter uses a single `<path>` of circles to keep the document small |
| `Html` | same, embedded |
| `Native` / `Wasm` | the CPU renderer rasterizes circles; at high point counts it switches to a point-splat pass |
| `Tui` | one cell per point, using the cell's glyph for the marker shape |
| `React` / `Vue` / `Angular` | one element per point, or a single path |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] Renders with a small and a large dataset, with the same visual result
      up to rasterization (`a_large_scatter_groups_its_circles_into_one_path`).
- [x] Dense, sparse and streaming inputs render the same marks
      (`dense_sparse_and_streaming_inputs_draw_the_same_scene`).
- [x] Grouping by `series` assigns stable palette colours and one legend entry
      per series (`the_series_column_assigns_one_stable_colour_and_legend_entry_per_value`).
- [x] Concurrent rendering of the same figure from several threads is safe
      (`several_threads_render_the_same_scatter_identically`).
- [x] The figure exports to SVG and the streaming path reports render metrics
      (`a_render_records_the_backend_representation_and_point_count`); later
      backends rasterize the same backend-agnostic scene.
- [ ] An animated scatter interpolates positions between two states and
      matches marks by key. The transition kernel exists, but the recipe does
      not expose a keyed transition surface yet; it is deferred to the
      animation recipe change.

## Open questions

- Should `SVScatterPlot` accept a matrix of columns and draw a scatterplot
  matrix, or is that the separate `SVSplomPlot` recipe? *Current answer:
  `SVSplomPlot` — a matrix of scatters is a different composition, not a
  mode of this one.*
