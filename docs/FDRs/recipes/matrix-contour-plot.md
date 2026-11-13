# FDR-132 — `SVContourPlot`

| Field | Value |
|---|---|
| Id | FDR-132 |
| Name | `SVContourPlot` |
| Family | Matrix and field |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [Chapter 36 — Contour plots](../../src/development-explained/ch36-contour-plot.md) |
| Example program | [`examples/contour_plot.rs`](../../../examples/contour_plot.rs) |

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
data -> [SVStat::Bin (grid) + SVStat::Aggregate (cell mean)] -> scalar field
     -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area
```

## Data contract

| Input | Shape | Notes |
|---|---|---|
| Grid | a dense 2D array | the field on a regular grid |
| Points | `x`, `y`, `value` columns | interpolated onto a grid by the stat |
| Sparse grid | indices and values | a sparse value column keeps its stored entries and answers every implicit position with zero; it is never materialised |

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
| `matrix` | `(usize, usize)` shape over the `value` column | `None` (point cloud) | — |
| `x`, `y`, `value` | `Option<SVColumnRef>` | `value` required; `x`/`y` for a point cloud | plotly |
| `levels` | `usize` | `10` | plotly |
| `level_range` | `Option<(f64, f64)>` | `None` | plotly `contours.start`/`end` |
| `level_size` | `Option<f64>` | `None` | plotly `contours.size` |
| `style` | `SVContourStyle` | `Lines` | plotly (`lines`, `fill`) |
| `labels` | `bool` | `false` | — |
| `missing_policy` | `SVContourMissingPolicy` | `Omit` | — |
| `colour_map` | `SVColourMap` | theme default | plotly |
| `line_width` | `f64` | token `mark.line_width` | — |
| `interpolation` | — | — | deferred: the point cloud is folded by cell mean for now |

## Theme

Consumes the continuous colour-map token, axis tokens, text tokens.

## Temporal

This change wires no animation. Every temporal row below is a design target, not
a shipped behaviour; the recipe recomputes the whole level set on each
`compose`.

| Property | Intended behaviour (not wired) |
|---|---|
| contour path | recomputed per frame and morphed by vertex order |
| colour | interpolates with the level |
| level set appearing or disappearing | enters or exits |

The contour-path matching problem is shared with
`SVDensityContourPlot` and is documented there. Interpolating contour *paths*
directly is hard when the topology changes (a contour can split into two); the
honest target is to recompute the level sets per frame and match their vertices
in order, accepting that a topology change is a visible cut. This is explicitly
out of scope for this change.

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

- [x] The contour levels follow the requested count, range or spacing (`contour_plot_tests::the_levels_follow_the_requested_range_and_spacing`, `a_level_count_changes_the_number_of_thresholds`).
- [x] The field is binned onto a regular grid when the input is a point cloud (`contour_plot_tests::a_point_cloud_is_binned_onto_a_grid_through_the_bin_kernel`, `a_point_cloud_cell_is_the_mean_aggregate_of_its_observations`).
- [x] A filled contour renders as nested closed paths without gaps (`contour_plot_tests::a_filled_style_paints_nested_bands_instead_of_lines`, `every_filled_subpath_is_closed`).
- [x] Dense, sparse and streaming inputs give the same contours for the same
      field (`parity_tests::dense_and_sparse_fields_agree`,
      `dense_and_streaming_contours_produce_the_same_primitives`,
      `a_structurally_sparse_matrix_is_never_densified`).

## Open questions

- Should the contour extraction (marching squares and relatives) live in
      `core-stat` as a distinct stat, since both contour recipes need it? *Open
      — leaning yes, as `SVStat::Contour`.* This change keeps the extraction in
      the recipe's `geometry` submodule (see the change design) because adding
      a stat variant is a core change (patterns P5).
