# FDR-113 — `SVDensityContourPlot`

| Field | Value |
|---|---|
| Id | FDR-113 |
| Name | `SVDensityContourPlot` |
| Family | Distributional |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [`ch24-density-contour-plot.md`](../../src/development-explained/ch24-density-contour-plot.md) |
| Example program | [`examples/density_contour_plot.rs`](../../../examples/density_contour_plot.rs) |

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

The closed stat vocabulary has no bivariate density variant, so the 2D density
is a documented composition of the existing one-dimensional `SVStat::Bin`
kernel on each axis, not a new stat implementation:

```
data -> [SVStat::Bin on x] + [SVStat::Bin on y] -> joint grid of counts
     -> density = count / (total weight * cell area)
     -> [Gaussian smoothing when bandwidth is set]
     -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area
```

The contour extraction is a step between the density field and the mark: the
joint aggregation produces a scalar field, and the recipe turns level sets of
that field into paths. `bandwidth` and `bandwidth_adjustment` request one
Gaussian pass over the binned field; there is no hidden kernel-density
implementation.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | first variable | continuous | yes |
| `y` | second variable | continuous | yes |
| `series` | one field each | nominal | no |
| `weight` | observation weight | continuous | no |

Accepted representations: dense, sparse, streaming (windowed). A streaming
window requires a backend that declares the streaming capability; a static
export backend such as `Svg` rejects it at build with
`UnsupportedBackendCapability` instead of silently freezing the first window.

### Missing values

A pair whose `x` or `y` value is not finite is omitted before binning, and an
input with no finite pair fails with `EmptyInput`. A non-finite `series` code or
`weight` is rejected with `NonFiniteValue`, and a negative weight is rejected as
a configuration error: dropping a bad weight would silently change the joint
mass, so the recipe never does it implicitly.

### Match key

`Index` over the grid.

### Parity tolerance

The documented tolerance is zero. Dense, sparse and streaming views of the same
values produce bit-identical grids and contour paths, because all three feed the
same accumulation loop; sparse views are walked through their stored entries and
the implicit-zero run, never densified.

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
| `bandwidth_adjustment` | `f64` | `1.0` | seaborn `bw_adjust` |
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
| `Svg` | one `<path>` per contour level; export only, so a streaming window is rejected at build |
| `Html` | same |
| `Native` / `Wasm` | polyline and polygon rasterization; a streaming window is read from its current chunk |
| `Tui` | contour lines as polylines across the cell grid |
| `React` / `Vue` / `Angular` | one `<path>` per level |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The contour levels follow the requested count or explicit range.
      Evidence: `the_level_count_is_the_requested_one` and
      `an_explicit_level_range_bounds_every_threshold`.
- [x] The density integrates to one over the domain.
      Evidence: `the_density_integrates_to_one_over_the_domain` (and the
      weighted and grouped variants).
- [x] Dense, sparse and streaming inputs give the same contours for the same
      data. Evidence: `dense_sparse_and_streaming_agree_within_the_documented_tolerance`
      and `dense_sparse_and_streaming_produce_the_same_contours`.
- [x] A filled contour renders as nested closed paths without gaps.
      Evidence: `filled_contours_are_filled_paths_and_lines_are_stroked_paths`,
      `every_filled_contour_subpath_is_closed`,
      `an_explicit_range_below_the_field_fills_every_band_as_a_closed_region`,
      `the_contour_stitcher_closes_a_ring_of_cell_segments` and the generated
      figure in `docs/figures/density_contour_plot.svg`.
- [x] The composition really uses the bin kernel, and a constant field or a
      level range outside the observed field is still a valid render.
      Evidence: `the_grid_edges_come_from_the_bin_kernel`,
      `a_constant_density_field_still_renders_the_background_band`,
      `an_all_equal_input_composes_without_a_degenerate_grid_error`,
      `an_explicit_range_above_the_field_leaves_only_the_background`.
- [x] The missing-value policy is explicit and tested.
      Evidence: `a_non_finite_pair_is_skipped_instead_of_failing_the_render`,
      `an_all_non_finite_input_is_an_empty_input_error` and
      `a_non_finite_weight_is_rejected`.

## Implementation notes

The closed stat vocabulary has no bivariate density variant (patterns P5), so
the recipe composes the existing `SVStat::Bin` kernel on each axis and folds
every observation pair into the joint grid. The density is the joint count
divided by the total observation weight and the cell area, which is why it
integrates to one. `bandwidth` and `bandwidth_adjustment` request a Gaussian
smoothing pass over the grid, with the kernel radius capped at the axis length
so an extreme bandwidth stays bounded; there is no hidden kernel-density
implementation.

Contour isolation is a recipe-level marching-squares step: line style emits one
stroked compound path per level, and filled style emits the threshold region
`density >= level` as one closed compound path per level, painted from the
lowest level up so the bands nest without gaps. Transition wiring is deferred:
the geometry is transition-ready, but no contour morphing is claimed.

## Open questions

- How should contour paths be matched between two states when the topology
      changes? *Open — this is the hardest temporal problem in the library.*
