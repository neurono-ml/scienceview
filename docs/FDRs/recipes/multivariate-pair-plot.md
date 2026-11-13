# FDR-144 — `SVPairPlot`

| Field | Value |
|---|---|
| Id | FDR-144 |
| Name | `SVPairPlot` |
| Family | Multivariate |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [ch43 — Pair plot](../../src/development-explained/ch43-pair-plot.md) |
| Example program | [`examples/pair_plot.rs`](../../../examples/pair_plot.rs) |

## Purpose

`SVPairPlot` is the pair scatterplot matrix: a grid whose cells show the
relationship between every pair of continuous variables, with the categories of
a third variable distinguished by colour, and the diagonal showing the
distribution of each variable per category.

It differs from `SVSplomPlot` in the way seaborn's `pairplot` differs from
plotly's `splom`: seaborn's is designed for categorical data, with dodging,
jitter and per-category distributions on the diagonal. This change implements
the continuous-variable form; the nominal-variable form is a documented
follow-up.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `pairplot` | <https://seaborn.pydata.org/api.html#seaborn.pairplot> |
| seaborn | `PairGrid` | <https://seaborn.pydata.org/api.html#seaborn.PairGrid> |
| plotly | `px.scatter_matrix` | <https://plotly.com/python/reference/#plotly.graph_objects.Splom> |
| bokeh | — | — |

## Recipe

```
data -> [no stat] -> values
     -> [SVMove::Jitter or Dodge within each cell]
     -> [SVScale per axis] -> SVMark::Dot (or a chart-owned hexbin/binned-contour cell), one cell per pair
```

The off-diagonal cell also accepts a chart-owned hexagonal lattice (hexbin) or a
chart-owned binned contour. Both are geometry composed from the closed kernels —
`SVStat::Bin` for the edges and counts, the scale kernel for the placement — not
new kernel variants (patterns P5). This mirrors the geometry the `SVHexbinPlot`
and `SVDensityContourPlot` recipes already own; those recipes compose their own
figure, so they cannot be delegated a single pair cell while keeping the shared
per-variable domain.

The diagonal uses the distribution recipe appropriate to the variable type:
a density estimate (`Auto`, the implemented choice) or a histogram for a
continuous variable. Categorical variables and their count-plot diagonal are
deferred to a follow-up: the current data boundary resolves numeric columns.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x_variables` | the horizontal axes | continuous | at least one |
| `y_variables` | the vertical axes | continuous | at least one |
| `hue` | the grouping, by colour | nominal, encoded numerically | no |

Nominal variables (and a count-plot diagonal for them) are a documented
follow-up, not part of this change.

Accepted representations: dense, sparse, streaming.

### Match key

`Index` over the observations, shared across cells.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x, y within a cell | two of the variables | `Continuous` |
| point colour | `hue` | group palette |
| diagonal cell | the distribution of one variable | per variable type |
| hexbin / contour cell | local observation density | continuous colour scale |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `variables` | `&[&str]` | every `X`/`Y` column | convenience |
| `x_variables` | `Vec<SVColumnRef>` | all continuous | seaborn |
| `y_variables` | `Vec<SVColumnRef>` | the `x_variables` list | seaborn |
| `hue` | `Option<SVColumnRef>` | `None` | seaborn |
| `diagonal_kind` | `SVPairDiagonalKind` | `Auto` → `Density` | seaborn |
| `off_diagonal_kind` | `SVPairOffDiagonalKind` | `Scatter` | seaborn `kind` |
| `marker` | `SVDotShape` | `Circle` | seaborn |
| `jitter` | `Option<SVMove>` | `None` | seaborn |
| `alpha` | `f32` | `0.85` | seaborn |
| `missing_value_policy` | `SVPairMissingValuePolicy` | `Omit` | seaborn `dropna` |
| `title` | `Option<String>` | `None` | — |
| `backend` | `SVBackendKind` | required | — |

Implemented surface: `variables`, `x_variables`, `y_variables`, `hue`,
`diagonal_kind`, `off_diagonal_kind`, `marker`, `jitter` (accepting `Jitter` and
`Dodge`), `alpha`, `missing_value_policy`, `title`, `backend` and
`execution_mode`. The FDR's `dodge` boolean is replaced by the explicit
`jitter(...)` displacement (a continuous axis has no categorical slot to dodge
into). `palette` is not yet a builder option: the hue palette is the built-in
scale palette, and a theme-aware palette override is a follow-up. The
categorical diagonal forms (`SVDiagonalCategorical`) are deferred with nominal
variables.

## Theme

Consumes the built-in decoration style (`text.*` and axis tokens) for the outer
axes, cell frames, title and legend. The `hue` palette is currently the built-in
scale palette rather than a theme token; a theme-aware `palette(...)` override
is a follow-up, so the FDR's `palette` row is not yet exposed.

## Temporal

| Property | Behaviour |
|---|---|
| point position | interpolates in every cell |
| point colour | interpolates |
| diagonal distribution | re-computed per frame |
| hue added | the points split and re-colour |

Transition and animation wiring is deferred to the transition integration; the
composed geometry is transition-ready and this change claims no transition
test.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one group per cell; points as elements |
| `Html` | same |
| `Native` / `Wasm` | point splatting per cell |
| `Tui` | only a small matrix is practical |
| `React` / `Vue` / `Angular` | one `<svg>` per cell |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The grid has one cell per pair of variables.
  Evidence: `pair_plot_tests::the_default_build_takes_every_continuous_column`
  asserts nine cells for three variables.
- [x] The diagonal shows the distribution the configuration selects.
  Evidence: `pair_plot_tests::the_diagonal_kind_changes_the_output` and the
  density/histogram cells composed in `diagonal.rs`.
- [x] The `hue` grouping is consistent across the scatter cells, the diagonal
  and the legend. Evidence:
  `pair_plot_tests::hue_groups_the_marks_and_the_diagonal` and
  `pair_plot_tests::a_missing_hue_stays_aligned_with_its_observation`. The
  hexbin and binned-contour forms encode observation density through a
  continuous colour scale instead of the hue palette.
- [x] Dense, sparse and streaming inputs give the same matrix.
  Evidence: `parity_tests::dense_sparse_and_streaming_inputs_draw_the_same_grid`
  and `parity_tests::every_off_diagonal_kind_is_representation_parity`.

## Open questions

- Should `SVPairPlot` and `SVSplomPlot` be one recipe with a type-dependent
      diagonal, rather than two? *Open — leaning yes; the difference is
      largely the diagonal and the dodge/jitter.*
