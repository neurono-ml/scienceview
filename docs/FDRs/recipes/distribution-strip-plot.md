# FDR-119 — `SVStripPlot`

| Field | Value |
|---|---|
| Id | FDR-119 |
| Name | `SVStripPlot` |
| Family | Distributional |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

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

- [ ] Every observation is drawn exactly once.
- [ ] The jitter is reproducible when a seed is given.
- [ ] Dense, sparse and streaming inputs give the same points.
- [ ] In an animation, the points do not drift: the jitter offset depends on
      the match key only.

## Open questions

- Should the jitter be applied in data space or display space? *Open — it is
      the same question as in `SVMove`, and the answer affects whether the
      jitter survives a change of axis range.*
