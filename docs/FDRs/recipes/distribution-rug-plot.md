# FDR-115 — `SVRugPlot`

| Field | Value |
|---|---|
| Id | FDR-115 |
| Name | `SVRugPlot` |
| Family | Distributional |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | `docs/src/development-explained/ch26-rug-plot.md` |
| Example program | `examples/rug_plot.rs` |

## Purpose

`SVRugPlot` draws a small tick for every observation along an axis. It shows
the raw sample that a summary chart hides, and it is normally used *underneath*
another chart to reveal the density of the data.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `rugplot` | <https://seaborn.pydata.org/api.html#seaborn.rugplot> |
| bokeh | `Segment` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.Segment> |

## Recipe

```
data -> [no stat] -> values -> [no move] -> [SVScale] -> SVMark::Line (segment geometry)
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `value` | the observed value | continuous | yes |
| `series` | colour grouping | nominal | no |

Accepted representations: dense, sparse, streaming.

### Match key

`Index` over the observations — a rug tick has no identity beyond its
position in the sample.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x (or y) | `value` | `Continuous` |
| tick colour | `series` | `Nominal` |
| tick height | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | seaborn |
| `series` | `Option<SVColumnRef>` | `None` | seaborn |
| `orientation` (FDR `orient`) | `SVBarOrientation` | `Vertical` | seaborn |
| `height` | `f32` | `0.05` | seaborn `height` |
| `alpha` | `f32` | `1.0` | seaborn |
| `line_width` | `SVLength` | token `mark.line_width` | — |

`height` is a fraction of the plot's perpendicular extent, capped at the depth
of the axis margin: a tick never enters the plot rectangle, even at
`height = 1.0`. A non-finite observation draws no tick; a column with no
drawable observation at all renders an empty scene rather than failing on a
domain it never needed. A sparse column contributes only its stored entries,
so implicit zeros neither produce a tick nor widen the value axis.

## Theme

Consumes `mark.palette`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| tick position | interpolates |
| new observation | the tick enters |
| removed observation | the tick exits |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<line>` per observation; a large sample uses a single path with many subpaths |
| `Html` | same |
| `Native` / `Wasm` | line rasterization |
| `Tui` | one cell per tick |
| `React` / `Vue` / `Angular` | one `<path>` with many subpaths |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] One tick per observation, at the observation's value
      (`the_default_configuration_draws_one_tick_per_observation`).
- [x] The ticks are drawn in the margin of the axis and never overlap the
      plot area's data region
      (`vertical_ticks_sit_in_the_bottom_margin_and_never_enter_the_plot`,
      `horizontal_orientation_places_the_ticks_in_the_left_margin`).
- [x] Dense, sparse and streaming inputs give the same ticks
      (`dense_sparse_and_streaming_inputs_draw_the_same_scene`); a sparse
      column marks only its stored entries
      (`a_sparse_input_marks_only_its_stored_entries`).
- [x] A large sample does not degrade the document size beyond the
      documented bound
      (`a_large_sample_draws_every_tick_in_one_compound_path`).

## Open questions

- Should the rug be composable *into* another recipe — that is, should
      `SVHistogramPlot` accept a `rug` option that adds rug ticks under the
      axis? *Open, and probably yes: it is the most common use of the rug.*
