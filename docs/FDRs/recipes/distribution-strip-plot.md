# FDR-119 — `SVStripPlot`

| Field | Value |
|---|---|
| Id | FDR-119 |
| Name | `SVStripPlot` |
| Family | Distributional |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [Chapter 30 — Strip plots](../../src/development-explained/ch30-strip-plot.md) |
| Example program | [`examples/strip_plot.rs`](../../../examples/strip_plot.rs) |

## Purpose

`SVStripPlot` shows every observation of a variable, positioned along the
categorical axis, with a small random displacement so that overlapping
observations remain visible. It is the raw-data counterpart of the box and
violin plots: the summary disappears and the sample appears.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `stripplot` | <https://seaborn.pydata.org/api.html#seaborn.stripplot> |
| seaborn | `objects.Dots` | <https://seaborn.pydata.org/api.html#seaborn.objects.Dots> |
| bokeh | `Circle` with jitter | <https://docs.bokeh.org/en/latest/docs/gallery/scatter/scatter.html> |
| plotly | `go.Box` with `boxpoints='all'` / `px.strip` | <https://plotly.com/python/reference/#plotly.graph_objects.Box> |

## Recipe

```
data -> [no stat] -> values -> [SVMove::Jitter (+ Dodge)]
     -> [SVScale] -> SVMark::Dot
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `value` | the observed variable | continuous | yes |
| `category` | the categorical axis | nominal | yes |
| `series` | grouping within a category | nominal | no |

Accepted representations: dense, sparse, streaming.

### Match key

`Index` over the observations, or `Identity` when an identifier column is
given.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `category` | `Nominal` |
| y | `value` | `Continuous` |
| fill colour | `series` | `Nominal` |
| jitter offset | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | all four |
| `category` | `SVColumnRef` | — | all four |
| `series` | `Option<SVColumnRef>` | `None` | all four |
| `jitter` | `f32` | `0.1` | seaborn |
| `jitter_mode` | `SVJitterMode` | `Uniform` | seaborn |
| `seed` | `Option<u64>` | `None` | — |
| `marker` | `SVDotShape` | `Circle` | bokeh, plotly |
| `alpha` | `f32` | `1.0` | seaborn |
| `size` | `SVLength` | token `mark.size` | bokeh, plotly |
| `orient` | `SVBarOrientation` | `Vertical` | seaborn |
| `missing_value_policy` | `SVStripMissingValuePolicy` | `Omit` | — |

## Theme

Consumes `mark.palette`, `mark.size`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| point position | the value interpolates; the jitter offset does not — it is recomputed from the match key, so the point does not drift between frames |
| colour and size | interpolate |
| new observation | the point enters |

The jitter rule is worth stating plainly, because it is the opposite of the
usual intuition: in an animated strip plot, points must **not** slide. The
jitter offset is a function of the mark's identity, not of the frame index.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one element per point |
| `Html` | same |
| `Native` / `Wasm` | circle rasterization |
| `Tui` | one cell per point |
| `React` / `Vue` / `Angular` | one element per point |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] Every observation is drawn exactly once — `strip_plot_tests::the_default_configuration_draws_one_dot_per_observation`, `large_strip_tests::a_large_sample_composes_one_dot_per_observation`.
- [x] The jitter is reproducible when a seed is given — `strip_plot_tests::a_seeded_jitter_is_reproducible_across_compositions`.
- [x] Dense, sparse and streaming inputs give the same points — `parity_tests::dense_sparse_and_streaming_inputs_draw_the_same_scene`, `parity_tests::a_sparse_and_dense_representation_of_the_same_observations_agree_on_the_jitter`.
- [x] In an animation, the points do not drift: the jitter offset depends on the match key only — `strip_plot_tests::the_jitter_offset_follows_the_match_key_and_not_the_value` (the offset follows the row and category keys, never the value; the transition wiring is deferred to the transition integration, the geometry is transition-ready).

## Open questions

- Should the jitter be applied in data space or display space? *Open — it is
      the same question as in `SVMove`, and the answer affects whether the
      jitter survives a change of axis range.*
