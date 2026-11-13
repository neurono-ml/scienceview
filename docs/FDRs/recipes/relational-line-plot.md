# FDR-102 — `SVLinePlot`

| Field | Value |
|---|---|
| Id | FDR-102 |
| Name | `SVLinePlot` |
| Family | Relational |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [`ch17-line-plot.md`](../../src/development-explained/ch17-line-plot.md) |
| Example program | [`examples/line_plot.rs`](../../../examples/line_plot.rs) |

## Purpose

`SVLinePlot` shows how a continuous value evolves along an ordered axis —
usually time. It draws one line per series, and it aggregates within a series
when several observations share the same `x`, which is what separates it from a
plain polyline.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `lineplot` | <https://seaborn.pydata.org/api.html#seaborn.lineplot> |
| seaborn | `objects.Line` | <https://seaborn.pydata.org/api.html#seaborn.objects.Line> |
| bokeh | `figure.line` / `MultiLine` | <https://docs.bokeh.org/en/latest/docs/gallery/lines/line.html> |
| plotly | `go.Scatter` (lines) | <https://plotly.com/python/reference/#plotly.graph_objects.Scatter> |

## Recipe

```
data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale x, y, colour] -> SVMark::Line
```

The stat is the heart of the recipe: seaborn's `lineplot` estimates a mean
(and a confidence interval) per `x` value within each series, while plotly and
bokeh draw the points as given. Both behaviours are needed, so the stat is
explicit and optional.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | horizontal position, the ordering axis | continuous or temporal | yes |
| `y` | vertical position | continuous | yes |
| `series` | grouping, one line each | nominal | no |
| `group` | grouping for the aggregation | nominal | no |

Accepted representations: dense, sparse, streaming. A streaming source is the
natural case: the line grows as the window fills.

### Match key

`Category` on the `x` value within a `(series, group)` pair. Points are matched
by their position on the ordering axis, which is what allows a line to grow at
its end rather than being redrawn.

A `group` column splits an already drawn series into one line per group. Two
groups observed at the same `x` therefore become two independent lines with
their own identities, never two vertices of one line (a vertical spike). An
aggregated group with a single `x` collapses to one sample.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` column | `Continuous` or `Temporal` |
| y | `y` column | `Continuous` |
| colour | `series` column | `Nominal` |
| line width | constant | — |
| dash | `series` (optional) | `Nominal` over dash patterns |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x`, `y` | `&str` column names | — | all three |
| `series` | `Option<&str>` | `None` | all three |
| `aggregate` | `Option<SVStat>` | `None` | seaborn |
| `estimator` | `SVAggregateMethod` | `Mean` | seaborn |
| `error` | `Option<SVAggregateErrorKind>` | `None` | seaborn |
| `ci` | `Option<f64>` | `None` — draw the interval | seaborn |
| `markers` | `bool` | `false` | seaborn, plotly |
| `line_width` | `SVLength` | token `mark.line_width` | bokeh, plotly |
| `dash` | `Option<SVDashPattern>` | `None` — encode the series | bokeh, plotly |
| `interpolate` | `SVLineInterpolation` | `Linear` | plotly `line_shape` (`linear`, `spline`, `hv`, `vh`, `hvh`, `vhv`) |
| `sort_by_x` | `bool` | `true` | seaborn `sort` |
| `legend` | `SVLegendPlacement` | `Auto` | all three |
| `x_scale_kind` | `SVScaleKind` | `Continuous` | seaborn |
| `temporal_unit` | `SVTemporalUnit` | `Auto` | seaborn |

`x`, `y`, `series` and `group` are column names resolved against the attached
`SVDataSource`. A mistyped name fails at build with the typed
`SVError::ColumnNotFound`, and columns that do not share one length fail with
`SVError::MismatchedLengths`.

When no explicit `dash` is configured, a multi-series chart cycles the
documented patterns across the series, so two lines stay distinguishable
without colour alone. `legend(false)` is accepted as `SVLegendPlacement::Hidden`;
otherwise the legend is placed at the requested `Auto`, `Right`, `Left`, `Top`,
`Bottom` or `Inside` position.

## Theme

Consumes `mark.line_width`, `mark.palette`, `text.*` and the axis tokens.

## Temporal

The shipped geometry is **transition-ready**: every series carries a stable
`(series, group)` key, the segments keep their observed positions, and the
dash/colour identity is resolved before rendering. A future animation layer can
therefore match points by `x` and swap the dash without re-deriving a position.

| Property | Shipped behaviour |
|---|---|
| point position | stable observed position; keyed interpolation deferred |
| line width and colour | stable per series; interpolation deferred |
| dash pattern | stable per series; swapping deferred |
| new point at the end of the line | reads the current streaming window and appends it |
| removed point | drops out of the next window read |

A growing streaming line is the archetypal live chart. The recipe exposes the
keyed identity the transition layer will consume, but no keyed transition
surface is wired in this change; the animation recipe change owns that wiring.

## Scene contract

The composition is backend-agnostic: it carries the solved viewport, the plot
rectangle, the decorations and the mark geometries. The SVG path serializes
that scene and the native path rasterizes the same scene into an RGBA frame, so
the two cannot drift. A backend that is not wired in this change is rejected at
render time with an explicit `SVError::UnsupportedBackendCapability`, never
silently degraded.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` per series; implemented in this change |
| `Native` | the CPU renderer rasterizes the same composed scene; implemented in this change |
| `Html` | same document, embedded; deferred, rejected explicitly |
| `Tui` | box-drawing characters across the cell grid; deferred, rejected explicitly |
| `React` / `Vue` / `Angular` | one `<path>` per series; deferred, rejected explicitly |
| `Tauri` / `Dioxus` / `Wasm` | same scene as native; deferred, rejected explicitly |

## Acceptance

- [x] With no aggregate, the line passes through the data points exactly.
      Evidence: `default_composition_draws_one_line_through_the_observations`.
- [x] With an aggregate, the drawn value is the estimator over the group and
      the optional interval is drawn.
      Evidence: `aggregation_reports_the_estimator_and_draws_the_interval`.
- [x] A `group` splits a series into one line per `(series, group)` with its
      own identity, and a single-`x` group collapses to one sample.
      Evidence: `a_group_splits_into_one_line_per_series_and_group` and
      `two_groups_over_several_x_values_keep_independent_lines`.
- [x] Dense, sparse and streaming inputs give the same line for the same
      data, including structured sparse columns that are never densified.
      Evidence: `dense_sparse_and_streaming_agree_within_the_documented_tolerance`,
      `sparse_x_with_streaming_y_matches_the_dense_baseline` and
      `a_structurally_sparse_y_matches_the_dense_zero_filled_baseline`.
- [x] A streaming line grows in place as the window fills.
      Evidence: `a_streaming_figure_reads_its_current_window_without_blocking`
      and `a_streaming_window_reports_a_growing_drawn_mark_count`.
- [x] The chart reads named columns from a data source and reports a typed
      `ColumnNotFound` for a mistyped name.
      Evidence: `a_mistyped_column_reports_the_typed_column_not_found_error`.
- [x] The SVG and native backends serialise/rasterise the same composed scene,
      and an unwired backend fails explicitly.
      Evidence: `the_svg_backend_matches_the_checked_in_figure_through_the_public_path`,
      `the_same_scene_rasterizes_natively` and
      `an_export_only_backend_rejects_a_streaming_column_at_build`.
- [x] The operation span reports the drawn marks, not the logical input length.
      Evidence: `aggregation_reports_drawn_marks_not_the_logical_length`.
- [x] The chart saves its rendered document and carries the execution mode.
      Evidence: `save_writes_the_rendered_document` and
      `execution_mode_is_carried_by_the_built_chart`.
- [ ] An animated line interpolates point positions and swaps the dash pattern.
      The geometry is transition-ready (stable `(series, group)` keys and dash
      identity), but the recipe exposes no keyed transition surface yet; the
      wiring is deferred to the animation recipe change. This criterion is
      intentionally not ticked.

## Open questions

- Should the confidence interval be a separate mark (`Ribbon`) inside this
      recipe, or a sibling recipe (`SVBandPlot`) that the line recipe may
      include? *Current answer: a sibling, so a band can be used alone.*


## Implementation notes

The recipe lives in `src/recipes/relational/line_plot/` (public path
`scienceview::sv_recipes::sv_relational::sv_line_plot`) and composes the core
kernels only: `SVStat::Aggregate` per `(series, group, x)`, continuous or
temporal `x` with a continuous `y` scale, `SVMark::Line`, `SVMark::Dot` for
markers, `SVMark::Ribbon` for the aggregated interval, and the decoration
surface for axes, grid, legend and title. Columns are resolved by name from the
attached `SVDataSource`; the in-memory `SVDataTable` and the borrowed-slice,
sparse and streaming adapters all work through the same trait. Ordering is a
stable sort by `x`; non-finite observations split the line into contiguous
segments unless `bridge_missing(true)` is requested. The `interpolate` option
expands step and Catmull-Rom vertices in data space before the line mark draws
them, so the mark kernel is never reimplemented.

The composed scene is backend-agnostic. `SVLinePlotComposition::to_svg()` emits
the document and `to_native_frame()` rasterises the same decorations and mark
geometries; `SVLinePlot::render()` dispatches on the configured kind and rejects
an unwired backend explicitly. The documentation figure is
`docs/figures/line_plot_recipe.svg` and the example is `examples/line_plot.rs`.
