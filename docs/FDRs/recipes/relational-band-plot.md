# FDR-105 — `SVBandPlot`

| Field | Value |
|---|---|
| Id | FDR-105 |
| Name | `SVBandPlot` |
| Family | Relational |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | `docs/src/development-explained/ch20-band-plot.md` |
| Example program | `examples/band_plot.rs` |

## Purpose

`SVBandPlot` fills the region between two boundary lines — an uncertainty
interval, a confidence band, a range between two series. It is the visual form
of an interval estimate, and it is what `SVLinePlot` uses to show its
confidence interval.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `objects.Band` | <https://seaborn.pydata.org/api.html#seaborn.objects.Band> |
| plotly | `go.Scatter` with `fill='tonexty'` | <https://plotly.com/python/reference/#plotly.graph_objects.Scatter> |
| bokeh | `Patches` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.Patches> |

## Recipe

```
data -> [SVStat::Aggregate + error] -> (lower, upper) -> [no move] -> [SVScale] -> SVMark::Ribbon
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | ordering axis | continuous or temporal | yes |
| `lower` | lower boundary | continuous | explicit mode |
| `upper` | upper boundary | continuous | explicit mode |
| `values` | observations the derived mode aggregates | continuous | derived mode |
| `series` | one band each | grouping (a numeric column is also accepted) | no |

The two modes are mutually exclusive. The **explicit** mode names `lower` and
`upper`; the **derived** mode selects `derive()` and names `values`. Given
`values` and a grouping, the stat computes the estimate and the interval. Both
modes are supported, and the derived mode is what `SVLinePlot` uses internally.

Accepted representations: dense, sparse and streaming. A sparse column
contributes only its stored indices (implicit zeros are not expanded into
rows); a streaming column contributes its current window and the result reports
`is_approximate()`.

Column existence and column roles are validated at `build`, not at `render`
(patterns P7): a missing column returns `SVError::ColumnNotFound`, and a column
of the wrong role returns `SVError::InvalidConfiguration`.

### Match key

`Category` on the `x` value within a series.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` | `Continuous` or `Temporal` |
| y | `lower`/`upper` | `Continuous` |
| fill colour | `series` | `Nominal` |
| opacity | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `data_source` | `&dyn SVDataSource` | — | all three |
| `backend` | `SVBackendKind` | — (required) | all three |
| `execution_mode` | `SVExecutionMode` | `Automatic` | foundation |
| `x` | `&str` | — | all three |
| `lower`, `upper` | `Option<&str>` | `None` | all three |
| `derive` | `bool` (selected by `derive()`) | `false` | seaborn |
| `values` | `Option<&str>` | `None` | seaborn |
| `series` | `Option<&str>` | `None` | all three |
| `error` | `SVAggregateErrorKind` | `StandardDeviation` | seaborn (`ci`) |
| `confidence` | `Option<f64>` (sets `Percentile`) | `None` | seaborn |
| `opacity` | `Option<f32>` | `None` (token `mark.area_opacity`) | all three |
| `fill` | `bool` | `true` | plotly `fill` |
| `border` | `bool` | `false` | plotly `line` |
| `dashed_borders` | `bool` | `true` | plotly `line.dash` |
| `centre_line` | `bool` | `false` | seaborn |
| `missing_value_policy` | `SVBandMissingValuePolicy` | `Omit` | recipe |
| `title` | `Option<&str>` | `None` | all three |
| `x_title` | `Option<&str>` | `None` | all three |
| `y_title` | `Option<&str>` | `None` | all three |
| `x_scale_kind` | `SVScaleKind` | `Continuous` | core-scale |
| `temporal_unit` | `SVTemporalUnit` | `Day` | core-scale |
| `viewport` | `SVViewport` | `640 × 400` | layout |

`x_scale_kind` accepts `Continuous` and `Temporal`; `Nominal` and `Boolean` are
rejected at build because a band has no ordered between-ness to fill. The
`derive` option is a mode, not a caller-supplied `SVStat`: only
`SVStat::Aggregate` reports the `(value, interval)` pair a ribbon consumes, so
the recipe builds that stat from `error` and `confidence` (design D5).

## Theme

Consumes `mark.area_opacity`, `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| boundary | interpolates (provided by the transition integration) |
| fill colour and opacity | interpolate (provided by the transition integration) |
| band appearing | enters (provided by the transition integration) |
| interval widening | the band grows from its centre (provided by the transition integration) |

The recipe composes interpolable `Ribbon`/`Line` paths and introduces no
discrete boundary property. It does **not** ship a transition API in this
change; the transition and animation integration owns the temporal behaviour
above. See the deferred acceptance item.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | implemented: one closed `<path>` per band, `to_svg()` |
| `Native` | implemented: the same geometry rasterized into `SVNativeFrame`, `to_native_frame()` |
| `Html` / `Wasm` | geometry composes, but no band-plot serializer ships here; `save` fails with `InvalidConfiguration` |
| `Tui` | geometry composes, but no band-plot serializer ships here; `save` fails with `InvalidConfiguration` |
| `React` / `Vue` / `Angular` / `Tauri` / `Dioxus` | geometry composes, but no band-plot serializer ships here; `save` fails with `InvalidConfiguration` |

A serializer never ignores the configured kind: asking `to_svg()` for a
non-`Svg` figure, `to_native_frame()` for a non-`Native` figure, or `save()`
for `Native` (which declares no export capability) returns a typed error. Later
backends that declare support for the ribbon mark add their serializer in
their own change.

## Acceptance

- [x] Explicit bounds are drawn exactly — `explicit_bounds_are_drawn_exactly`
  in `band_plot_tests.rs` asserts **both** boundaries at every scaled x
  position, not only the upper one.
- [x] The derived mode produces the interval the estimator defines and marks
      it as approximate when the source is a stream —
  `derived_mode_aggregates_mean_and_standard_deviation` compares the band with
  `SVStat::Aggregate`; `confidence_selects_a_percentile_interval` asserts the
  exact centred 90% interval; `SVBandRenderOutput::is_approximate` reports the
  streaming window.
- [x] Dense, sparse and streaming inputs give the same band for the same
      data — `dense_sparse_and_streaming_agree` compares every path vertex
      within `1e-9`; `implicit_sparse_zeros_match_the_dense_equivalent` covers
      implicit zeros.
- [x] A sparse input is never densified —
      `a_huge_sparse_column_with_few_stored_entries_draws_only_those_entries`
      and `derived_huge_sparse_input_aggregates_only_its_stored_entries` render
      ten million logical positions with a handful of stored entries and draw
      only those entries.
- [x] The SVG and native serializers render the same composed geometry —
      `svg_output_is_self_contained_and_deterministic` and
      `native_frame_matches_the_composed_geometry`.
- [ ] An animated band interpolates both boundaries — **deferred** to the
      transition integration. The recipe composes interpolable paths and ships
      no transition API or test in this change, so the claim is not ticked.

## Implementation

- Module: `src/recipes/relational/band_plot/`, exposed as
  `scienceview::sv_recipes::sv_relational::sv_band_plot` through `#[path]`
  (builder, composition, configuration, data contract, derived contract,
  decorations, output).
- Tests: `band_plot_tests.rs`, `band_plot_derived_tests.rs`,
  `band_plot_missing_tests.rs`, `band_plot_parity_tests.rs`,
  `band_plot_concurrency_tests.rs`, `band_plot_series_tests.rs`,
  `band_plot_observability_tests.rs`, `band_plot_svg_tests.rs` and
  `band_plot_native_tests.rs`.
- Figure: `docs/figures/band_plot.svg`, regenerated by `examples/band_plot.rs`
  through the public `to_svg()` serializer.
- Missing values are expressed as non-finite numbers; the explicit policy is
  `SVBandMissingValuePolicy::Omit` (drop the sample and split the band at the
  gap, in both the explicit and the derived mode) or `Error` (reject with
  `SVError::InvalidConfiguration`).

## Open questions

- Should the band expose a `confidence` level that changes the interval
      interactively, and should that be a control in the UI kit? *Open — it
      would be a natural companion to the range slider.*
