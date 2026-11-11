# FDR-113 — `SVDensityContourPlot`

| Field | Value |
|---|---|
| Id | FDR-113 |
| Name | `SVDensityContourPlot` |
| Family | Distributional |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVDensityContourPlot` shows the two-dimensional density of two continuous
variables as filled contours or contour lines. It is the bivariate counterpart
of `SVDensityPlot`, and it is the most common way to show a joint
distribution when the sample is too sparse for a histogram.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `kdeplot` (bivariate) | <https://seaborn.pydata.org/api.html#seaborn.kdeplot> |
| plotly | `go.Contour`, `go.Histogram2dContour` | <https://plotly.com/python/reference/#plotly.graph_objects.Contour> |

## Recipe

```
data -> [SVStat::KernelDensity (2D)] -> (grid, density)
     -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area
```

The contour extraction is a step between the stat and the mark: the density
kernel produces a scalar field, and the recipe turns level sets of that field
into paths.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | first variable | continuous | yes |
| `y` | second variable | continuous | yes |
| `series` | one field each | nominal | no |
| `weight` | observation weight | continuous | no |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Index` over the grid.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` | `Continuous` |
| y | `y` | `Continuous` |
| colour | the contour level | `Continuous` over a colour map |
| line colour | the contour level | same |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x`, `y` | `SVColumnRef` | — | seaborn, plotly |
| `series` | `Option<SVColumnRef>` | `None` | plotly |
| `levels` | `Option<usize>` | `10` | plotly `contours.start/end/size` |
| `level_range` | `Option<(f64, f64)>` | `None` | plotly |
| `bandwidth` | `Option<f64>` | `None` | seaborn |
| `bw_adjust` | `f64` | `1.0` | seaborn |
| `grid_count` | `usize` | `256` | — |
| `style` | `SVContourStyle` | `Filled` | plotly (`lines`, `fill`) |
| `colour_map` | `SVColourMapRef` | theme default | — |
| `colour_scale` | `SVScaleTransform` | `Linear` | plotly `contours.coloring` |

## Theme

Consumes the continuous colour-map token, so a theme change restyles every
contour plot and its colour bar consistently.

## Temporal

| Property | Behaviour |
|---|---|
| contour paths | interpolate as the density field changes; paths whose level set disappears exit |
| colour | interpolates with the level |
| bandwidth change | the contours morph into the new level sets |

Interpolating contour *paths* directly is hard when the topology changes (a
contour can split into two). The honest behaviour is to recompute the level
sets per frame and morph them by matching their vertices in order, accepting
that a topology change is a visible cut. This is an open question.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` per contour level |
| `Html` | same |
| `Native` / `Wasm` | polyline and polygon rasterization |
| `Tui` | contour lines as polylines across the cell grid |
| `React` / `Vue` / `Angular` | one `<path>` per level |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The contour levels follow the requested count or explicit range.
- [ ] The density integrates to one over the domain.
- [ ] Dense, sparse and streaming inputs give the same contours for the same
      data.
- [ ] A filled contour renders as nested closed paths without gaps.

## Open questions

- How should contour paths be matched between two states when the topology
      changes? *Open — this is the hardest temporal problem in the library.*
