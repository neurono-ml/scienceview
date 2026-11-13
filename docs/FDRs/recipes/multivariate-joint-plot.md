# FDR-145 — `SVJointPlot`

| Field | Value |
|---|---|
| Id | FDR-145 |
| Name | `SVJointPlot` |
| Family | Multivariate |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [Chapter 44 — Joint plots](../../src/development-explained/ch44-joint-plot.md) |
| Example program | [`examples/joint_plot.rs`](../../../examples/joint_plot.rs) |

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

- [x] The central panel shows the bivariate relationship and the two margins
      show the corresponding marginals. Evidence:
      `joint_plot_tests::the_default_composition_builds_the_three_panels`
      asserts the top marginal sits above the central panel and the right
      marginal to its right.
- [x] The three panels share their axes, so a point's position in the centre
      agrees with the marginal's scale. Evidence:
      `joint_plot_tests::the_three_panels_share_the_central_axes` asserts the
      scene's horizontal and vertical scales span exactly the central panel.
- [x] The kind option switches the central panel between the scatter, the
      hexbin, the density contour and the regression. Evidence:
      `joint_plot_tests::every_central_kind_composes`.
- [x] Dense, sparse and streaming inputs give the same figure. Evidence:
      `parity_tests::dense_sparse_and_streaming_agree`.

The scene's serializers are covered by
`render_tests::the_svg_kind_serializes_the_svg_document`,
`render_tests::the_native_kind_rasterizes_the_native_frame` and
`render_tests::a_kind_without_a_recipe_serializer_reports_its_capability`; the
operation span by
`observability_tests::a_compose_records_the_backend_representation_and_point_count`;
thread safety by
`concurrency_tests::the_same_figure_renders_from_several_threads`.

Transition and animation wiring is deferred to the transition integration; the
composed geometry is transition-ready.

## Open questions

- Should the joint plot be a figure with three panels (one figure, three
      layout regions) or three figures composed by the dashboard? **Resolved in
      this implementation: one figure with three layout regions.** The recipe
      names the regions and hands their relative weights to `sv_layout`, which
      solves the rectangles, so a viewport change re-solves the whole figure.
