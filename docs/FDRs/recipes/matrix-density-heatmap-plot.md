# FDR-133 — `SVDensityHeatmapPlot`

| Field | Value |
|---|---|
| Id | FDR-133 |
| Name | `SVDensityHeatmapPlot` |
| Family | Matrix and field |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVDensityHeatmapPlot` shows the two-dimensional density of two continuous
variables as a raster of coloured cells. It is the binned counterpart of
`SVDensityContourPlot`: cheaper to compute, and it shows the sample support
honestly rather than a smoothed estimate.

## Sources

| Library | Function | Link |
|---|---|---|
| plotly | `go.Histogram2d`, `go.Densitymapbox` | <https://plotly.com/python/reference/#plotly.graph_objects.Histogram2d> |
| seaborn | `histplot` (bivariate) | <https://seaborn.pydata.org/api.html#seaborn.histplot> |
| bokeh | `Quad` over a categorical axis | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.Quad> |

## Recipe

```
data -> [SVStat::Bin (2D)] -> (grid, counts) -> [SVScale::Continuous over a colour map]
     -> SVMark::Image
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | first variable | continuous | yes |
| `y` | second variable | continuous | yes |
| `series` | one raster each | nominal | no |
| `weight` | observation weight | continuous | no |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Derived` — the pair of bin indices.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` | `Continuous` |
| y | `y` | `Continuous` |
| fill colour | the count or density | `Continuous` over a colour map |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x`, `y` | `SVColumnRef` | — | all three |
| `series` | `Option<SVColumnRef>` | `None` | plotly |
| `bin_count_x` | `Option<usize>` | derived from the width | seaborn, plotly |
| `bin_count_y` | `Option<usize>` | derived from the height | seaborn, plotly |
| `normalise` | `SVNormalisation` | `Count` | plotly `histfunc` |
| `colour_map` | `SVColourMapRef` | theme default | — |
| `colour_bar` | `bool` | `true` | — |
| `opacity` | `f32` | `1.0` | — |

## Theme

Consumes the continuous colour-map token, axis tokens, text tokens.

## Temporal

| Property | Behaviour |
|---|---|
| cell colour | interpolates |
| cell entering or leaving | enters or exits |
| grid growing | the new cells enter |
| colour map change | all cells re-colour |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<rect>` per cell, or a generated raster image for a large grid |
| `Html` | same |
| `Native` / `Wasm` | rectangle rasterization |
| `Tui` | one cell per bin |
| `React` / `Vue` / `Angular` | one `<rect>` per cell |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The 2D binning matches the requested bin counts on each axis.
- [ ] The colour bar spans the value domain.
- [ ] Dense, sparse and streaming inputs give the same raster.
- [ ] Empty bins are drawn as the map's lowest value, or transparent when
      configured.

## Open questions

- Should this recipe be a specialisation of `SVHeatmapPlot` — that is, the
      heatmap recipe with a binning stat — rather than a separate type? *Open:
      a shared implementation with a different default stat is tempting, but
      the two have different axis semantics.*
