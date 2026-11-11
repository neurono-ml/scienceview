# FDR-112 — `SVDensityPlot`

| Field | Value |
|---|---|
| Id | FDR-112 |
| Name | `SVDensityPlot` |
| Family | Distributional |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVDensityPlot` shows a smoothed estimate of the distribution of a continuous
variable, drawn as a curve (or filled to the baseline). It answers "what does
the shape of this distribution look like" in a way a histogram cannot, at the
cost of a bandwidth choice.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `kdeplot` (univariate) | <https://seaborn.pydata.org/api.html#seaborn.kdeplot> |
| seaborn | `objects.KDE` | <https://seaborn.pydata.org/api.html#seaborn.objects.KDE> |
| plotly | `go.Histogram` with `histfunc='density'` / `go.Scatter` | <https://plotly.com/python/reference/#plotly.graph_objects.Histogram> |

## Recipe

```
data -> [SVStat::KernelDensity] -> (grid, density) -> [no move]
     -> [SVScale] -> SVMark::Line | SVMark::Area
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `value` | the variable whose density is estimated | continuous | yes |
| `series` | one density each | nominal | no |
| `weight` | observation weight | continuous | no |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Index` over the density grid. The grid is a fixed lattice, so the match key
is the grid position.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | the density grid | `Continuous` |
| y | density | `Continuous` |
| colour | `series` | `Nominal` |
| fill opacity | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | seaborn |
| `series` | `Option<SVColumnRef>` | `None` | seaborn, plotly |
| `bandwidth` | `Option<f64>` | `None` — rule of thumb | seaborn `bw_adjust` |
| `bw_adjust` | `f64` | `1.0` | seaborn |
| `kernel` | `SVKernelShape` | `Gaussian` | seaborn |
| `grid_count` | `usize` | `256` | — |
| `cut` | `f64` | `3.0` | seaborn `cut` |
| `fill` | `bool` | `false` | seaborn `fill` |
| `normalise` | `SVNormalisation` | `Density` | seaborn `common_norm` |
| `line_width` | `SVLength` | token `mark.line_width` | — |

## Theme

Consumes `mark.palette`, `mark.area_opacity`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| density curve | interpolates point by point over the fixed grid |
| fill | interpolates with the curve |
| bandwidth change | the curve morphs into the new estimate |
| grid extension | the curve extends, interpolating from the last shared grid point |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` per series |
| `Html` | same |
| `Native` / `Wasm` | polyline rasterization |
| `Tui` | polyline across the cell grid |
| `React` / `Vue` / `Angular` | one `<path>` per series |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The density integrates to one, within the documented tolerance.
- [ ] The bandwidth follows the rule of thumb when not given, and scales
      linearly with `bw_adjust`.
- [ ] A sparse input is never densified; the kernel works from the sparse
      values directly.
- [ ] A streaming input is estimated over the window and says so.
- [ ] An animated density interpolates over the grid rather than re-appearing.

## Open questions

- Should the bandwidth be chosen automatically by a better rule (for example,
      a plug-in selector), and if so is that a different kernel? *Open.*
- Should the density kernel support a weighted variant natively, or through
      the `weight` column? *Current answer: through the column — the kernel
      takes weights rather than growing a second variant.*
