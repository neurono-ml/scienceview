# FDR-116 — `SVBoxPlot`

| Field | Value |
|---|---|
| Id | FDR-116 |
| Name | `SVBoxPlot` |
| Family | Distributional |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [Chapter 27 — Box plots](https://neurono-ml.github.io/scienceview/development-explained/ch27-box-plot.html) |
| Example program | [`examples/box_plot.rs`](../../../examples/box_plot.rs) |

## Purpose

`SVBoxPlot` summarises a distribution with five numbers — minimum, first
quartile, median, third quartile, maximum — drawn as a box with whiskers, and
optionally with the outliers that fall outside the whiskers.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `boxplot` | <https://seaborn.pydata.org/api.html#seaborn.boxplot> |
| seaborn | `objects.Box` | <https://seaborn.pydata.org/api.html#seaborn.objects.Box> |
| bokeh | `Box` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.Box> |
| plotly | `go.Box` | <https://plotly.com/python/reference/#plotly.graph_objects.Box> |

## Recipe

```
data -> [SVStat::Percentile + outlier extraction]
     -> [SVMove::Dodge] -> [SVScale] -> SVMark::Ribbon | SVMark::Line | SVMark::Dot
```

`SVMark::Box` is the FDR's shorthand for the box body. The mark vocabulary is
closed and has no `Box` variant, so the body is the filled region between the
quartiles (`SVMark::Ribbon`), the median/whiskers/caps are `SVMark::Line` and
the outliers and mean are `SVMark::Dot`.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `value` | the observed variable | continuous | yes |
| `category` | the categorical axis | nominal | yes |
| `series` | grouping within a category | nominal | no |

Accepted representations: dense, sparse, streaming (windowed).

### Match key

`Category` on the category (plus series, when present).

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `category` | `Nominal` |
| y | the five-number summary | `Continuous` |
| box colour | `series` | `Nominal` |
| outlier colour | `series` | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | all four |
| `category` | `SVColumnRef` | — | all four |
| `series` | `Option<SVColumnRef>` | `None` | all four |
| `quartiles` | `Vec<f64>` | `[0.25, 0.5, 0.75]` | plotly `q1`, `median`, `q3` |
| `whisker` | `SVWhiskerRule` | `Tukey` | seaborn, plotly |
| `whisker_range` | `f64` | unset (`Tukey`) | selects `SVWhiskerRule::Percentile` at this lower percentile |
| `show_outliers` | `bool` | `true` | seaborn, plotly |
| `outlier_marker` | `SVDotShape` | `Circle` | plotly `boxpoints` |
| `notch` | `bool` | `false` | seaborn, plotly `notched` |
| `show_mean` | `bool` | `false` | seaborn `showmeans` |
| `width` | `f32` | `0.8` | seaborn `width` |
| `orient` | `SVBarOrientation` | `Vertical` | seaborn |

`SVWhiskerRule` has `Tukey` (1.5 times the interquartile range), `MinMax`
(the extremes) and `Percentile` (a given percentile of the data).

### Statistics

The box edges and any percentile whisker come from `SVStat::Percentile`; the
recipe never reimplements a quantile estimator. The kernel defines the `p`
quantile by linear interpolation between the two neighbouring ranks: with `n`
sorted observations the position is `p · (n - 1)` and the value is interpolated
between the observations at its floor and ceiling ranks (the R-7 definition,
also the NumPy default). For `1, 2, …, 10` this gives `q1 = 3.25`, the median
`5.5` and `q3 = 7.75`.

`Tukey` then selects the most extreme observations inside
`q1 - 1.5 · IQR` and `q3 + 1.5 · IQR`; `MinMax` selects the extremes;
`Percentile(p)` selects the `p` and `1 - p` quantiles. The observations outside
the selected ends are the outliers.

### Missing observations

A row whose value or category coding is not finite is skipped. When no row
survives, the render fails with `EmptyInput` instead of drawing an empty chart.
The implicit zeros of a sparse input are not observations: only the stored
entries participate, so a sparse column of huge logical length never
materialises its zeros.

## Theme

Consumes `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| box extent | interpolates (the quartiles move), provided by the transition integration |
| whisker ends | interpolate, provided by the transition integration |
| median line | interpolates, provided by the transition integration |
| outlier set | outliers enter and exit individually, provided by the transition integration |
| category added or removed | the box enters or exits, provided by the transition integration |

The recipe composes interpolable `Ribbon` and `Line` paths but ships no
transition API in this change; the transition and animation integration owns
the temporal behaviour.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` for the box body (three for a notch), four for the whisker stems and caps, one for the median, one element per outlier |
| `Html` | same |
| `Native` / `Wasm` | rectangle and line rasterization |
| `Tui` | the box as a cell rectangle, the median as a character row |
| `React` / `Vue` / `Angular` | same structure as SVG |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The five-number summary matches the estimator's definition for a known
      dataset (`box_plot_tests::default_summary_matches_the_percentile_estimator`).
- [x] The whisker rule is applied as specified and the outliers are exactly
      the points outside the whiskers
      (`tukey_rule_marks_points_outside_the_fences_as_outliers`,
      `min_max_rule_draws_the_extremes_and_no_outliers`,
      `percentile_rule_uses_the_requested_percentiles`).
- [x] Dense, sparse and streaming inputs give the same summary for the same
      data (`box_plot_parity_tests::dense_sparse_and_streaming_agree`); the
      sparse support is never densified and a streaming window reports
      `is_approximate`.
- [x] Outliers are drawn as individual marks and can be toggled
      (`show_outliers_can_be_turned_off`,
      `outlier_marker_changes_the_marker_shape`).
- [ ] The temporal behaviour in the table above is provided by the transition
      and animation integration (deferred; the recipe ships no transition API
      in this change).

## Open questions

- Should the box plot support a `notch` shape in every backend? *Resolved for
      this recipe: `notch(true)` draws three stacked ribbon segments around the
      median, using the conventional `1.58 · IQR / √n` half-width clamped to
      half the interquartile range. The shape is backend-agnostic because it is
      made of the same ribbon primitives as the plain box.*
