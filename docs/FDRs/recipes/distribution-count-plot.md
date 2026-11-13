# FDR-122 — `SVCountPlot`

| Field | Value |
|---|---|
| Id | FDR-122 |
| Name | `SVCountPlot` |
| Family | Distributional |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [Chapter 33 — Count plots](../../src/development-explained/ch33-count-plot.md) |
| Example program | [`examples/count_plot.rs`](../../../examples/count_plot.rs) |

## Purpose

`SVCountPlot` shows how many observations fall into each category. It is a
bar chart whose value is always a count, which makes it the categorical
counterpart of the histogram and the simplest possible categorical chart.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `countplot` | <https://seaborn.pydata.org/api.html#seaborn.countplot> |
| bokeh | `VBar` over a categorical axis | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.VBar> |
| plotly | `go.Bar` with pre-aggregated values, `px.histogram` | <https://plotly.com/python/reference/#plotly.graph_objects.Bar> |

## Recipe

```
data -> [SVStat::Count] -> (count per category) -> [SVMove::Stack | Dodge]
     -> [SVScale] -> SVMark::Bar
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `category` | the counted variable | nominal | yes |
| `series` | grouping within a category | nominal | no |
| `weight` | observation weight | continuous | no — deferred: the shipped `SVStat::Count` kernel does not consume weights, so the builder rejects the option with a typed configuration error naming the kernel gap; weighted counting is a follow-up change to `statistical-transforms` |

Accepted representations: dense, sparse, streaming. A sparse category column
contributes its stored entries and every implicit zero as a category-zero
observation, without being densified.

### Match key

`Category` on the category (plus series).

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `category` | `Nominal` |
| y | the count | `Continuous` |
| fill colour | `series` | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `category` | `SVColumnRef` | — | all three |
| `series` | `Option<SVColumnRef>` | `None` | seaborn, plotly |
| `orient` | `SVBarOrientation` | `Vertical` | seaborn |
| `width` | `f32` | `0.8` | seaborn |
| `normalise` | `SVNormalisation` | `Count` | seaborn `normalize` |
| `order` | `SVCountOrder` | `DataOrder` | seaborn, plotly |
| `colour` | `Option<SVColour>` | theme palette | bokeh, plotly |
| `multiple` | `SVCountMultiple` | `Layer` | seaborn, plotly |
| `missing_value_policy` | `SVCountMissingValuePolicy` | `Omit` | — |
| `category_labels`, `series_labels` | `&[&str]` | empty (numeric codes) | — |
| `title`, `x_label`, `y_label` | `Option<&str>` | `None` | — |

## Theme

Consumes `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| bar height | interpolates |
| category ordering change | the bars slide to their new positions |
| new category | enters |
| new observation | the bar grows |

Category reordering is a nice, small animation: the bars keep their identity
and slide, because the category is the match key.

**Not shipped by this change:** the transition and animation driver belongs to
the transition integration change. The scene is transition-ready — the category
is the match key, so a state change can interpolate the bar height and slide
the bars — but this change runs no animation and claims no animated reordering.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<rect>` per bar |
| `Html` | same |
| `Native` / `Wasm` | rectangle rasterization |
| `Tui` | one cell column per bar |
| `React` / `Vue` / `Angular` | one `<rect>` per bar |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The count equals the number of observations in the category.
      `count_plot_tests::the_default_configuration_counts_every_category`,
      `parity_tests::a_large_dense_column_counts_without_losing_observations`.
- [x] With `normalise`, the counts become fractions of the total.
      `count_plot_tests::probability_normalisation_divides_by_the_grand_total`.
- [x] The category order follows the requested ordering rule.
      `count_plot_tests::the_ascending_order_sorts_the_categories`,
      `count_plot_tests::the_descending_order_reverses_the_categories`.
- [x] Dense, sparse and streaming inputs give the same counts.
      `parity_tests::dense_sparse_and_streaming_inputs_draw_the_same_scene`,
      `parity_tests::mixed_representations_agree_after_validating_lengths`,
      `parity_tests::a_sparse_input_with_implicit_zeros_counts_them_as_category_zero`.
- [x] A configured weight is rejected with the typed kernel-gap error.
      `count_plot_tests::a_configured_weight_is_rejected_with_the_kernel_gap`.
- [ ] Animated reordering and entry. **Not shipped by this change:** the
      geometry is transition-ready but the animation driver belongs to the
      transition integration change.

## Open questions

- Should the count support a `normalise` that divides by the category total
      rather than the grand total (a conditional proportion)? *Open.*
