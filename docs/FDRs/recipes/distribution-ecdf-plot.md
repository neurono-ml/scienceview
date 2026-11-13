# FDR-114 — `SVECDFPlot`

| Field | Value |
|---|---|
| Id | FDR-114 |
| Name | `SVECDFPlot` |
| Family | Distributional |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [Chapter 25 — ECDF plots](https://neurono-ml.github.io/scienceview/development-explained/ch25-ecdf-plot.html) |
| Example program | [`examples/ecdf_plot.rs`](../../../examples/ecdf_plot.rs) |

## Purpose

`SVECDFPlot` draws the empirical cumulative distribution function: the
proportion of observations at or below each value. It is a step function that
requires no parameters, which makes it the honest alternative to a density
estimate, and it is the right chart for comparing distributions directly.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `ecdfplot` | <https://seaborn.pydata.org/api.html#seaborn.ecdfplot> |
| plotly | `go.Scatter` with `ecdf` transform | <https://plotly.com/python/reference/#plotly.graph_objects.Scatter> |

## Recipe

```
data -> [SVStat::EmpiricalCdf] -> (values, cumulative probability)
     -> [no move] -> [SVScale] -> SVMark::Line (step geometry)
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `value` | the observed variable | continuous | yes |
| `series` | one ECDF each | nominal | no |
| `weight` | observation weight | continuous | no |

Accepted representations: dense, sparse, streaming (windowed).

The kernel variant `SVStat::EmpiricalCdf` does not consume observation weights,
so the recipe exposes `value` and `series` only; a weighted ECDF is a core
change to the stat capability first (patterns.md P5).

### Match key

`Category` on the observed value within a series.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | observed value | `Continuous` |
| y | cumulative probability | `Continuous`, domain `[0, 1]` |
| colour | `series` | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | seaborn |
| `series` | `Option<SVColumnRef>` | `None` | seaborn |
| `normalise` | `bool` | `true` | seaborn `proportion` |
| `orient` | `SVBarOrientation` | `Vertical` | seaborn `orient` |
| `line_width` | `SVLength` | token `mark.line_width` | — |

## Theme

Consumes `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| step vertices | interpolate |
| new observation | the step rises at the new value |
| series removed | exits |

Transition and animation wiring is deferred to the transition integration: this
recipe ships the transition-ready step geometry and the current-window
re-render only (recorded in `openspec/changes/ecdf-plot/tasks.md`, task 6b).

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` per series, step geometry |
| `Html` | same |
| `Native` / `Wasm` | polyline rasterization |
| `Tui` | polyline across the cell grid |
| `React` / `Vue` / `Angular` | one `<path>` per series |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The curve is a non-decreasing step function reaching 1 (or the total
      weight) at the maximum. Verified by
      `the_curve_reaches_one_at_the_maximum_and_is_a_step` and
      `normalise_off_reaches_the_observation_count`.
- [x] With `normalise` off, the curve reaches the total weight. Verified by
      `normalise_off_reaches_the_observation_count` (counts, no weights).
- [x] Dense, sparse and streaming inputs give the same ECDF for the same
      data. Verified by `dense_sparse_and_streaming_agree` and
      `large_dense_and_streaming_inputs_produce_the_same_vertices`.
- [x] The step geometry is exact: horizontal then vertical, never sloped.
      Verified by `the_curve_reaches_one_at_the_maximum_and_is_a_step`.

## Open questions

- Should the ECDF support a complementary distribution (the survival
      function) as an option? *Resolved: exposed as `complementary(bool)`,
      derived from the same kernel probabilities.*
