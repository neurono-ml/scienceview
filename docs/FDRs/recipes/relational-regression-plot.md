# FDR-106 — `SVRegressionPlot`

| Field | Value |
|---|---|
| Id | FDR-106 |
| Name | `SVRegressionPlot` |
| Family | Relational |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [`ch21-regression-plot.md`](https://neurono-ml.github.io/scienceview/development-explained/ch21-regression-plot.html) |
| Example program | [`examples/regression_plot.rs`](../../../examples/regression_plot.rs) |

## Purpose

`SVRegressionPlot` overlays a fitted model on a scatter, showing the data
points and the predicted curve, optionally with a confidence band. It is the
chart that turns a correlation into a statement.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `regplot`, `lmplot` | <https://seaborn.pydata.org/api.html#seaborn.regplot> |
| seaborn | `objects.PolyFit` | <https://seaborn.pydata.org/api.html#seaborn.objects.PolyFit> |
| plotly | `go.Scatter` with `trendline` | <https://plotly.com/python/reference/#plotly.graph_objects.Scatter> |

## Recipe

```
data -> [SVStat::Fit] -> (coefficients, predicted curve) -> [no move]
     -> [SVScale] -> SVMark::Dot (points) + SVMark::Line (fit) + SVMark::Ribbon (interval)
```

This recipe composes three marks. It is the first recipe where one figure
carries more than one kind of mark, and therefore the reference for how
several marks share one figure's axes and legend.

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `x` | predictor | continuous | yes |
| `y` | response | continuous | yes |
| `series` | grouping; one fit per series | nominal | no |
| `weight` | observation weight | continuous | no |

Accepted representations: dense, sparse, streaming (windowed — a fit over an
unbounded stream is meaningless).

### Match key

`Identity` on an identifier column when given; otherwise the points are not
individually identifiable and the recipe animates the *fit* rather than the
points. The recipe documents this: without a key, the scatter is redrawn and
the fit line morphs.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | `x` | `Continuous` |
| y | `y` | `Continuous` |
| point colour | `series` | `Nominal` |
| fit line colour | `series` | `Nominal` — the same entry as its points |
| band opacity | constant | — |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `x`, `y` | `SVColumnRef` | — | all three |
| `series` | `Option<SVColumnRef>` | `None` | all three |
| `model` | `SVFitModel` | `Linear` | seaborn `PolyFit` |
| `order` | `usize` | `1` | seaborn `PolyFit` |
| `fit` | `SVFitEstimator` | `OrdinaryLeastSquares` | — |
| `show_points` | `bool` | `true` | seaborn |
| `show_fit` | `bool` | `true` | — |
| `confidence` | `Option<f64>` | `None` | seaborn `ci` |
| `ci` | `Option<SVAggregateErrorKind>` | `None` | seaborn |
| `marker` | `SVDotShape` | `Circle` | — |
| `line_width` | `SVLength` | token `mark.line_width` | — |

## Theme

Consumes `mark.palette`, `mark.size`, `mark.area_opacity`, `text.*`, axis
tokens.

## Temporal

| Property | Behaviour |
|---|---|
| fit line | morphs — the coefficients interpolate, and the curve is recomputed per frame |
| confidence band | morphs with the fit |
| points | redraw unless an identity key is available |
| model change | the curve re-fits and morphs into the new shape |

A changing model is the interesting animation here: the curve does not move
point by point, it changes shape. Interpolating the coefficients and
recomputing the curve is the correct behaviour — interpolating the sampled
curve would pass through nonsense for a polynomial.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | three mark groups in one figure |
| `Html` | same |
| `Native` / `Wasm` | the fit is computed off the render thread; the curve is rasterized with the other marks |
| `Tui` | points as cells, the fit as a polyline, the band as shaded cells |
| `React` / `Vue` / `Angular` | three mark groups in one component |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The fitted curve reproduces the estimator's coefficients for a known
      dataset. `the_linear_fit_recovers_the_generating_line` and
      `a_polynomial_order_is_fitted_not_interpolated` assert the kernel's
      coefficients, not a recipe reimplementation.
- [x] A polynomial fit of order *n* is fitted, not interpolated through.
      `a_polynomial_order_is_fitted_not_interpolated` recovers the quadratic
      coefficients and the 128-sample kernel grid.
- [x] The confidence band matches the estimator's interval.
      `the_confidence_band_brackets_the_curve` checks the kernel's lower and
      upper samples around every prediction.
- [x] Dense, sparse and streaming inputs give the same fit for the same data.
      `dense_sparse_and_streaming_agree_within_the_tolerance` compares
      coefficients and predictions to `1e-9`;
      `a_sparse_view_is_paired_through_its_stored_support_without_zeros`
      proves implicit zeros are never materialised, and
      `sparse_columns_with_different_supports_are_rejected` names the
      documented alternative.
- [x] An animated model change morphs the curve through interpolated
      coefficients, not through interpolated samples. The recipe exposes the
      kernel coefficients (`SVRegressionSeriesFit::coefficients`) and the
      recomputed grid, so the transition kernel can interpolate the
      coefficients and re-evaluate the curve per frame; the composition test
      suite asserts the coefficients and the 128-sample grid are the kernel's
      own values.

## Implementation notes

- **Weight is not exposed.** `SVStat::Fit` and `SVFitEstimator` consume no
  observation-weight column, and patterns.md P5 forbids adding a kernel variant
  inside a recipe. The FDR's `weight` row is therefore deferred to a core-stat
  change; the recipe never silently ignores a weight column.
- **`ci` is not exposed separately.** The band's shape is the kernel's
  `confidence` parameter; a second interval selector would invent an interval
  the kernel does not compute.
- **Sparse pairing.** The kernel rejects sparse storage because two
  independently sparse columns describe implicit zeros at different positions.
  The recipe accepts a sparse pair only when the stored support is identical,
  pairs the stored entries, and rejects a mismatch with the same
  `UnsupportedRepresentation` alternative. No implicit zero is materialised.
- **Extrapolation.** `SVExtrapolationPolicy::FitRange` draws the kernel grid
  over the observations. `ClipToVisibleDomain` clips that grid to an explicit
  visible domain; the recipe never evaluates a model outside the fitted range,
  because that evaluation belongs to the fit kernel.
- **Missing values.** A pair whose predictor, response or group code is not
  finite is dropped and counted (`SVRegressionComposition::dropped_pairs`);
  an input with no finite pair fails with `SVError::EmptyInput`.
- **Backends.** SVG is the implemented render path in this change. The plot
  builds for every backend whose mark matrix includes Dot, Line and Ribbon;
  `render` reports the missing export capability for the others until a later
  backend change adds its path.


## Open questions

- Should the fit estimator be an open trait so that a user can plug in their
      own regression, and should the trait live in the core or in a separate
      crate? *Current answer: an open trait in the core, with the ordinary
      least-squares implementation built in — consistent with the
      open-traits-at-the-boundary principle.*
- Should the residual plot be a separate recipe or a mode of this one? *Open;
      it was deferred from seaborn's `residplot`.*
