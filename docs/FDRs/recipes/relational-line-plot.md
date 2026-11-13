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

`Category` on the `x` value within a series. Points are matched by their
position on the ordering axis, which is what allows a line to grow at its end
rather than being redrawn.

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
| `x`, `y` | `SVColumnRef` | — | all three |
| `series` | `Option<SVColumnRef>` | `None` | all three |
| `aggregate` | `Option<SVStat>` | `None` | seaborn |
| `estimator` | `SVAggregateMethod` | `Mean` | seaborn |
| `error` | `Option<SVAggregateErrorKind>` | `None` | seaborn |
| `ci` | `Option<f64>` | `None` — draw the interval | seaborn |
| `markers` | `bool` | `false` | seaborn, plotly |
| `line_width` | `SVLength` | token `mark.line_width` | bokeh, plotly |
| `dash` | `Option<SVDashPattern>` | `None` | bokeh, plotly |
| `interpolate` | `SVLineInterpolation` | `Linear` | plotly `line_shape` (`linear`, `spline`, `hv`, `vh`, `hvh`, `vhv`) |
| `sort_by_x` | `bool` | `true` | seaborn `sort` |
| `legend` | `SVLegendPlacement` | `Auto` | all three |

## Theme

Consumes `mark.line_width`, `mark.palette`, `text.*` and the axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| point position | interpolates |
| line width and colour | interpolate |
| dash pattern | swaps |
| new point at the end of the line | enters; if the line is still growing, the point is appended and the segment to it interpolates |
| removed point | exits, and the segment is re-drawn without it |

A growing streaming line is the archetypal live chart: the transition layer
matches by `x` value, so existing points do not move.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` per series |
| `Html` | same |
| `Native` / `Wasm` | polyline rasterization |
| `Tui` | the line is drawn with box-drawing characters across the cell grid |
| `React` / `Vue` / `Angular` | one `<path>` per series |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] With no aggregate, the line passes through the data points exactly.
      Evidence: `default_composition_draws_one_line_through_the_observations`.
- [x] With an aggregate, the drawn value is the estimator over the group and
      the optional interval is drawn.
      Evidence: `aggregation_reports_the_estimator_and_draws_the_interval`.
- [x] Dense, sparse and streaming inputs give the same line for the same
      data.
      Evidence: `dense_sparse_and_streaming_agree_within_the_documented_tolerance`.
- [x] A streaming line grows in place as the window fills.
      Evidence: `a_streaming_figure_reads_its_current_window_without_blocking`
      reads the current window and reports the same number of points.
- [x] An animated line interpolates point positions and preserves series
      colour. The transition kernel matches by `x`; the recipe keeps a stable
      palette colour per series, asserted by
      `series_divides_the_lines_and_assigns_a_stable_colour_each`.

## Open questions

- Should the confidence interval be a separate mark (`Ribbon`) inside this
      recipe, or a sibling recipe (`SVBandPlot`) that the line recipe may
      include? *Current answer: a sibling, so a band can be used alone.*


## Implementation notes

The recipe lives in `src/recipes/relational/line_plot/` and composes the core
kernels only: `SVStat::Aggregate` per `(series, x, group)`, continuous `x`/`y`
scales, `SVMark::Line`, `SVMark::Dot` for markers, `SVMark::Ribbon` for the
aggregated interval, and the decoration surface for axes, grid, legend and
title. Ordering is a stable sort by `x`; non-finite observations split the line
into contiguous segments unless `bridge_missing(true)` is requested. The
`interpolate` option expands step and Catmull-Rom vertices in data space before
the line mark draws them, so the mark kernel is never reimplemented.

The SVG output is produced through `scienceview_svg::SVSvgDocument`; the
documentation figure is `docs/figures/line_plot_recipe.svg` and the example is
`examples/line_plot.rs`.
