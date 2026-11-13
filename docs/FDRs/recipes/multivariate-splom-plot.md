# FDR-143 — `SVSplomPlot`

| Field | Value |
|---|---|
| Id | FDR-143 |
| Name | `SVSplomPlot` |
| Family | Multivariate |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [`ch42-splom-plot.md`](../../src/development-explained/ch42-splom-plot.md) |
| Example program | [`examples/splom_plot.rs`](../../../examples/splom_plot.rs) |

## Purpose

`SVSplomPlot` is a scatterplot matrix: a grid of scatter plots, one per pair
of variables, with the observations shared across the cells. It is the
compact way to see every pairwise relationship in a data set at once.

## Sources

| Library | Function | Link |
|---|---|---|
| plotly | `go.Splom` | <https://plotly.com/python/reference/#plotly.graph_objects.Splom> |
| seaborn | `pairplot` (the categorical variant) | <https://seaborn.pydata.org/api.html#seaborn.pairplot> |
| bokeh | — | — |

## Recipe

```
data -> [no stat] -> values -> [SVScale per axis] -> SVMark::Dot, once per cell
```

The grid composition is a layout concern. The individual cells are scatter
plots, which is `SVScatterPlot`; the recipe composes them into a matrix.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| variables | one column per axis | continuous | at least two |

Accepted representations: dense, sparse, streaming.

### Match key

`Index` over the observations, shared across cells — the same observation must
be identifiable in every cell, which is what lets the highlighting follow it.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x, y within a cell | two of the variables | `Continuous` |
| diagonal cell | the distribution of one variable | `Continuous` |
| point colour | an optional grouping | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `variables` | `Vec<SVColumnRef>` | — | plotly |
| `colour` | `Option<SVColumnRef>` | `None` | plotly |
| `diagonal` | `SVSplomDiagonal` | `Histogram` | plotly (`histogram`, `box`, `violin`) |
| `marker` | `SVDotShape` | `Circle` | plotly |
| `size` | `SVLength` | token `mark.size` | plotly |
| `opacity` | `f32` | `1.0` | plotly |
| `show_upper` | `bool` | `true` | seaborn `pairplot` |
| `show_lower` | `bool` | `true` | seaborn |
| `show_diagonal` | `bool` | `true` | seaborn |

## Theme

Consumes `mark.palette`, `mark.size`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| point position | interpolates in every cell |
| highlight | the highlighted observation moves in every cell at once |
| variable added | a row and a column are inserted and the grid re-flows |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<rect>` per point per cell; a large matrix produces a large document |
| `Html` | same |
| `Native` / `Wasm` | point splatting per cell |
| `Tui` | one character cell per point per cell; only a small matrix is practical |
| `React` / `Vue` / `Angular` | one `<svg>` per cell, or one with nested groups |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The grid has one cell per variable pair, with the diagonal showing the
      univariate distribution. Covered by
      `the_default_composition_draws_every_pair_and_the_diagonal`,
      `a_density_diagonal_changes_the_rendered_document` and
      `an_empty_diagonal_draws_no_diagonal_primitives`.
- [ ] The same observation is identifiable across cells, so a highlight
      follows it. The observations are shared and keep one index in every
      cell, but the highlight surface is deferred to the interactivity wave.
- [x] The upper and lower triangles can be enabled independently. Covered by
      `the_triangles_are_independently_switchable` and
      `the_diagonal_can_be_switched_off`.
- [x] Dense, sparse and streaming inputs give the same matrix. Covered by
      `dense_sparse_and_streaming_inputs_draw_the_same_scene`,
      `a_sparse_input_is_never_densified` and
      `a_sparse_input_with_implicit_zeros_keeps_its_logical_observations`.

## Implementation notes

- The grid is solved through `sv_layout`: one row node per variable, one cell
  node per pair, and every backend receives the same solved rectangles.
- `scale_mode` selects between a domain per variable (`Independent`, the
  reference-library behaviour) and one domain common to all variables
  (`Shared`).
- `missing_value_policy` drops or rejects a non-finite observation in every
  cell at once. A dropped observation produces a shorter filtered copy; an
  untouched sparse column is never expanded.
- The diagonal composes `SVStat::Bin` (histogram) or
  `SVStat::KernelDensity` (density) with the bar or area mark; no statistic is
  reimplemented.
- `render()` dispatches on the configured kind: SVG document, native frame, or
  a typed `UnsupportedBackendCapability` error. Transition and animation
  wiring is deferred; the geometry is transition-ready.

## Open questions

- Should the splom share one `SVFigure` with sub-panels, or compose several
      figures? *Open — this touches the layout engine and the dashboard
      capability; the grid is arguably a dashboard concern, and this recipe
      is the single-cell-level description of it.*
