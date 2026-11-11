# FDR-120 — `SVSwarmPlot`

| Field | Value |
|---|---|
| Id | FDR-120 |
| Name | `SVSwarmPlot` |
| Family | Distributional |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVSwarmPlot` shows every observation of a variable like a strip plot, but
positions the points so that **no two overlap**. The displacement is computed
to avoid collisions rather than at random, so the density of the sample is
read directly from the width of the swarm.

It is the one chart in the catalogue whose geometry is a constraint problem
rather than a statistic, and that is why it needs its own kernel.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `swarmplot` | <https://seaborn.pydata.org/api.html#seaborn.swarmplot> |
| bokeh | — | — |
| plotly | — | — |

## Recipe

```
data -> [no stat] -> values -> [SVMove::Swarm (collision-avoiding displacement)]
     -> [SVScale] -> SVMark::Dot
```

The swarm displacement is a distinct move from `SVMove::Jitter`: it is
deterministic given the data and the order, and it guarantees non-overlap.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `value` | the observed variable | continuous | yes |
| `category` | the categorical axis | nominal | yes |
| `series` | grouping within a category | nominal | no |

Accepted representations: dense, sparse, streaming.

### Match key

`Index` over the observations within a category, in a deterministic order
(sorted by value). The order is part of the definition, because the swarm
layout depends on it.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `category` | `Nominal` |
| y | `value` | `Continuous` |
| horizontal offset | the collision-avoiding displacement | — |
| fill colour | `series` | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | seaborn |
| `category` | `SVColumnRef` | — | seaborn |
| `series` | `Option<SVColumnRef>` | `None` | seaborn |
| `marker` | `SVDotShape` | `Circle` | seaborn |
| `size` | `SVLength` | token `mark.size` | seaborn |
| `orient` | `SVBarOrientation` | `Vertical` | seaborn |
| `swarm_width` | `Option<f32>` | `None` — the category width | seaborn `dodge` |
| `alpha` | `f32` | `1.0` | seaborn |

## Theme

Consumes `mark.palette`, `mark.size`, `text.*`, axis tokens.

## Temporal

| Property | Behaviour |
|---|---|
| point value | interpolates |
| swarm offset | recomputed per frame from the new arrangement — it is a layout, not a stored position |
| colour | interpolates |

Recomputing the swarm per frame is correct but can be visually noisy: when
values change, the whole swarm may re-arrange. The honest behaviour is that
the swarm re-lays out and the points slide; there is no way to keep both
non-overlap and point identity stable when the values themselves move.

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

- [ ] No two points within a category overlap, for any input.
- [ ] The layout is deterministic: the same data produces the same swarm.
- [ ] The swarm respects the configured width.
- [ ] Dense, sparse and streaming inputs give the same swarm for the same
      data.

## Open questions

- What is the complexity bound of the collision-avoiding layout? *Open — a
      naive pairwise algorithm is quadratic, and a large swarm needs a
      smarter sweep.*
- Should the swarm layout be a move in `SVMove`, or a core kernel of its
      own? *Current answer: a move, because it only displaces.*
