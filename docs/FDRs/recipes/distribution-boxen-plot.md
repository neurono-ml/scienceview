# FDR-118 — `SVBoxenPlot`

| Field | Value |
|---|---|
| Id | FDR-118 |
| Name | `SVBoxenPlot` |
| Family | Distributional |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [Chapter 29 — Boxen plots](https://neurono-ml.github.io/scienceview/development-explained/ch29-boxen-plot.html) |
| Example program | [`examples/boxen_plot.rs`](../../../examples/boxen_plot.rs) |

## Purpose

`SVBoxenPlot` — letter-value plots — draws a distribution as a nest of
letter-value boxes of decreasing depth. Each box shows the median of the data
remaining after the outer boxes are removed, so the plot reveals the shape of
a large distribution with far fewer observations than a violin needs.

It exists in seaborn and nowhere else, but it is a genuine statistical chart
with a distinct encoding, so it earns its own FDR rather than being folded
into the box plot.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `boxenplot` | <https://seaborn.pydata.org/api.html#seaborn.boxenplot> |
| bokeh | — | — |
| plotly | — | — |

## Recipe

```
data -> [SVStat::Aggregate (repeated quantiles over trimmed data)]
     -> [SVMove::Dodge] -> [SVScale] -> SVMark::Box (nested)
```

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
| y | the value range of each letter-value box | `Continuous` |
| box colour | `series` | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | seaborn |
| `category` | `SVColumnRef` | — | seaborn |
| `series` | `Option<SVColumnRef>` | `None` | seaborn |
| `depth` | `usize` | derived from the sample size | seaborn `depth` |
| `tail_rule` | `SVBoxenTailRule` | `Tukey` | seaborn `k_depth` |
| `width` | `f32` | `0.8` | seaborn |
| `orient` | `SVBarOrientation` | `Vertical` | seaborn |
| `show_median` | `bool` | `true` | seaborn |
| `show_outliers` | `bool` | `false` | seaborn |
| `outlier_marker` | `SVDotShape` | `Circle` | — |

## Theme

Consumes `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| box extents | interpolate |
| median | interpolates |
| depth change | boxes enter and exit as the nesting depth changes |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<path>` per box in the nest |
| `Html` | same |
| `Native` / `Wasm` | rectangle rasterization |
| `Tui` | one cell rectangle per box |
| `React` / `Vue` / `Angular` | same structure as SVG |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The nesting depth is derived from the sample size when not given, and
      follows seaborn's rule. `SVBoxenTailRule::Tukey` implements Henderson's
      `floor(log2(n)) - 3`, clamped to at least one level; `Proportion` and
      `Full` are the other tail-stopping rules, and `depth(usize)` overrides
      them. Evidence: `derived_depth_follows_the_henderson_rule`,
      `the_proportion_rule_changes_the_derived_depth` and
      `the_full_rule_extends_to_the_observed_extremes`.
- [x] Each letter-value boundary is the nested quantile produced by
      `SVStat::Percentile` — the central box spans the interquartile range and
      every outer boundary is the quantile of the remaining half of the data.
      Evidence: `default_summary_uses_the_percentile_kernel` and
      `depth_option_adds_nested_levels`.
- [x] Dense, sparse and streaming inputs give the same nest for the same
      data. Evidence: `dense_sparse_and_streaming_agree`,
      `a_sparse_column_is_never_densified` and
      `a_streaming_window_renders_without_expanding_the_stream`.

## Open questions

- Should the derived depth rule be identical to seaborn's, or should it be
      re-derived from the available width? *Current answer: from the sample
      size, as in seaborn; the width rule belongs to the layout engine.*
