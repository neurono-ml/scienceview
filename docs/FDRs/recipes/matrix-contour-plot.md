# FDR-132 — `SVContourPlot`

| Field | Value |
|---|---|
| Id | FDR-132 |
| Name | `SVContourPlot` |
| Family | Matrix and field |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVContourPlot` draws the level sets of a scalar field over two continuous
variables, as contour lines or filled bands. Unlike
`SVDensityContourPlot`, the field is given directly — it is the recipe for
plotting a physical potential, a loss surface or a response surface.

## Sources

| Library | Function | Link |
|---|---|---|
| plotly | `go.Contour` | <https://plotly.com/python/reference/#plotly.graph_objects.Contour> |
| seaborn | — | — |
| bokeh | — | — |

## Recipe

```
data -> [SVStat::Aggregate (grid interpolation)] -> scalar field
     -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area
```

## Data contract

| Input | Shape | Notes |
|---|---|---|
| Grid | a dense 2D array | the field on a regular grid |
| Points | `x`, `y`, `value` columns | interpolated onto a grid by the stat |
| Sparse grid | indices and values | only the occupied points contribute |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Index` over the contour paths, in level order.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` | `Continuous` |
| y | `y` | `Continuous` |
| colour | the contour level | `Continuous` over a colour map |
| line width | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `grid` | `SVHeatmapSource` | — | plotly |
| `x`, `y`, `value` | `Option<SVColumnRef>` | `None` | plotly |
| `levels` | `Option<usize>` | `10` | plotly |
| `level_range` | `Option<(f64, f64)>` | `None` | plotly `contours.start`/`end` |
| `level_size` | `Option<f64>` | `None` | plotly `contours.size` |
| `style` | `SVContourStyle` | `Lines` | plotly (`lines`, `fill`) |
| `colour_map` | `SVColourMapRef` | theme default | plotly |
| `line_width` | `SVLength` | token `mark.line_width` | — |
| `interpolation` | `SVGridInterpolation` | `Bilinear` | — |

## Theme

Consumes the continuous colour-map token, axis tokens, text tokens.

## Temporal

| Property | Behaviour |
|---|---|
| contour path | recomputed per frame and morphed by vertex order |
| colour | interpolates with the level |
| level set appearing or disappearing | enters or exits |

The contour-path matching problem is shared with
`SVDensityContourPlot` and is documented there.

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

- [ ] The contour levels follow the requested count, range or spacing.
- [ ] The field is interpolated onto a grid when the input is a point cloud.
- [ ] A filled contour renders as nested closed paths without gaps.
- [ ] Dense, sparse and streaming inputs give the same contours for the same
      field.

## Open questions

- Should the contour extraction (marching squares and relatives) live in
      `core-stat` as a distinct stat, since both contour recipes need it? *Open
      — leaning yes, as `SVStat::Contour`.*
