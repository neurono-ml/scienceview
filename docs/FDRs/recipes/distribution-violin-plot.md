# FDR-117 — `SVViolinPlot`

| Field | Value |
|---|---|
| Id | FDR-117 |
| Name | `SVViolinPlot` |
| Family | Distributional |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [Chapter 28 — Violin plots](https://neurono-ml.github.io/scienceview/development-explained/ch28-violin-plot.html) |
| Example program | [`examples/violin_plot.rs`](../../../examples/violin_plot.rs) |

## Purpose

`SVViolinPlot` shows a distribution as a symmetric density silhouette, one
per category. It reveals the shape of a distribution — multimodality, skew,
bimodality — that a box plot hides, at the cost of a bandwidth choice.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `violinplot` | <https://seaborn.pydata.org/api.html#seaborn.violinplot> |
| bokeh | — (composed from `VArea`) | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.VArea> |
| plotly | `go.Violin` | <https://plotly.com/python/reference/#plotly.graph_objects.Violin> |

## Recipe

```
data -> [SVStat::KernelDensity per category] -> (grid, density per category)
     -> [SVMove::Dodge] -> [SVScale] -> SVMark::Area
```

The violin is a mirrored density: the area mark's baseline is the category
centre and the density is scaled to the category half-width. That is a
*scaling* of the density, which is why the recipe computes it rather than
delegating to a generic scale.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `value` | the observed variable | continuous | yes |
| `category` | the categorical axis | nominal | yes |
| `series` | grouping within a category | nominal | no |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Category` on the category (plus series).

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `category` | `Nominal` |
| y | `value` | `Continuous` |
| width | the density at that value | scaled to the category slot |
| fill colour | `series` | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | seaborn, plotly |
| `category` | `SVColumnRef` | — | seaborn, plotly |
| `series` | `Option<SVColumnRef>` | `None` | seaborn, plotly |
| `bandwidth` | `Option<f64>` | `None` | seaborn, plotly |
| `bw_adjust` | `f64` | `1.0` | seaborn |
| `width` | `f32` | `0.8` | seaborn, plotly `width` |
| `scale` | `SVViolinScale` | `Area` | seaborn `scale`, plotly `scalemode` |
| `show_box` | `bool` | `false` | seaborn `inner='box'` |
| `show_points` | `bool` | `false` | seaborn `inner='point'` |
| `orientation` | `SVBarOrientation` | `Vertical` | seaborn |

`SVViolinScale` has `Area` (each violin has equal area), `Count` (area
proportional to the sample count) and `Width` (all violins the same maximum
width).

## Theme

Consumes `mark.palette`, `mark.area_opacity`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| violin silhouette | interpolates; the width at each grid point moves |
| fill colour | interpolates |
| category added | the violin enters and grows to its width |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one closed `<path>` per violin |
| `Html` | same |
| `Native` / `Wasm` | filled polygon rasterization |
| `Tui` | shaded cells forming the silhouette |
| `React` / `Vue` / `Angular` | one `<path>` per violin |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The silhouette matches the density estimate for the chosen bandwidth
      (`violin_plot_tests::default_estimate_matches_the_kernel_density`,
      `bandwidth_option_changes_the_silhouette`,
      `bandwidth_adjustment_scales_the_derived_bandwidth`).
- [x] The scale mode is honoured: equal area, proportional to count, or equal
      width (`width_option_scales_the_maximum_half_width`,
      `width_scale_mode_makes_every_violin_the_same_width`,
      `count_scale_mode_grows_the_area_with_the_sample_count`).
- [x] The violin is symmetric about the category centre
      (`vertical_violin_is_symmetric_about_the_category_centre`).
- [x] Dense, sparse and streaming inputs give the same silhouette for the
      same data (`violin_plot_parity_tests::dense_sparse_and_streaming_agree`);
      the sparse support is never densified and a streaming window reports
      `is_approximate`.
- [x] The optional inner box and points do not change the silhouette
      (`inner_box_and_points_add_geometry_without_moving_the_silhouette`).
- [x] The split mode places two series in the two halves of one category
      (`split_series_share_a_category_and_opposite_halves`).
- [x] Both orientations render (`horizontal_orientation_swaps_the_value_axis`).

## Open questions

- Should the violin support "split" mode, where two series share one
      category and are drawn against each other? *Resolved: `split(true)` draws
      at most two series on opposite sides of the category centre; three or
      more series are rejected in favour of the dodge layout.*
