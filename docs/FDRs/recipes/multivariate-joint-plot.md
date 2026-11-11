# FDR-145 — `SVJointPlot`

| Field | Value |
|---|---|
| Id | FDR-145 |
| Name | `SVJointPlot` |
| Family | Multivariate |
| Status | planned |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | _pending_ |
| Example program | _pending_ |

## Purpose

`SVJointPlot` shows the relationship between two variables together with the
marginal distribution of each: a central bivariate plot, with a univariate
distribution along the bottom and along the left.

It is the composition that answers "what does the relationship look like, and
what do the two marginals look like" in a single figure.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `jointplot` | <https://seaborn.pydata.org/api.html#seaborn.jointplot> |
| seaborn | `JointGrid` | <https://seaborn.pydata.org/api.html#seaborn.JointGrid> |
| bokeh | — | — |
| plotly | — | — |

## Recipe

```
data -> [no stat] -> values
     -> central cell:  SVMark::Dot (or a density contour)
     -> marginal cells: the distribution recipe for the variable type
     -> [SVScale per axis] -> three panels sharing a layout
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | the horizontal variable | continuous or nominal | yes |
| `y` | the vertical variable | continuous or nominal | yes |
| `series` | grouping | nominal | no |

Accepted representations: dense, sparse, streaming.

### Match key

`Index` over the observations, shared between the central panel and the
marginals.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x, y in the central panel | the two variables | `Continuous` |
| x in the bottom margin | the distribution of `x` | `Continuous` |
| y in the left margin | the distribution of `y` | `Continuous` |
| colour | `series` | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x`, `y` | `SVColumnRef` | — | seaborn |
| `series` | `Option<SVColumnRef>` | `None` | seaborn |
| `kind` | `SVJointKind` | `Scatter` | seaborn (`scatter`, `kde`, `hist`, `reg`) |
| `marginal_kind` | `SVJointMarginal` | `Auto` | seaborn (`hist`, `kde`, `box`) |
| `marker` | `SVDotShape` | `Circle` | seaborn |
| `alpha` | `f32` | `1.0` | seaborn |
| `reg_model` | `SVFitModel` | `Linear` | seaborn `kind='reg'` |

## Theme

Consumes `mark.palette`, `mark.size`, `mark.area_opacity`, `text.*`, axis
tokens.

## Temporal

| Property | Behaviour |
|---|---|
| point position | interpolates in the central panel |
| marginal distribution | re-computed per frame and morphed |
| kind change | the central panel swaps between the scatter, the contour and the fit |

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | three groups in one document, positioned by the layout engine |
| `Html` | same |
| `Native` / `Wasm` | the three panels are three layout regions |
| `Tui` | three regions of the cell grid |
| `React` / `Vue` / `Angular` | three sub-components |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [ ] The central panel shows the bivariate relationship and the two margins
      show the corresponding marginals.
- [ ] The three panels share their axes, so a point's position in the centre
      agrees with the marginal's scale.
- [ ] The kind option switches the central panel between the scatter, the
      density contour and the regression.
- [ ] Dense, sparse and streaming inputs give the same figure.

## Open questions

- Should the joint plot be a figure with three panels (one figure, three
      layout regions) or three figures composed by the dashboard? *Open — the
      answer determines whether the recipe owns a layout or borrows one.*
