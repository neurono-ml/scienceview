# FDR-002 — `SVStat`

| Field | Value |
|---|---|
| Id | FDR-002 |
| Name | `SVStat` |
| Family | Core — computation |
| Status | in-progress |
| Layer | Core |
| Public chapter | `docs/src/development-explained/ch10-statistical-transforms.md` |
| Example program | none yet — the executable example is the `sv_stat` module doctest |

## Purpose

`SVStat` is a statistical transform applied to the data *before* it is drawn.
It is the layer that makes a chart say something about the data rather than
merely display it: binning a histogram, estimating a density, computing an
empirical cumulative distribution, fitting a curve.

This is the most consequential kernel in the library, because it is where the
representation-parity rule of `data-view-boundary` is enforced. A statistical
kernel receives an `SVColumnView` — dense, sparse or streaming — and dispatches
on it **exactly once per operation**. A streaming input is never blocked on and
never densified; a sparse input is never expanded.

## Sources

seaborn's `Stat` objects are the model; plotly and bokeh implement the same
transforms inside their chart types.

| `SVStat` | seaborn objects | plotly | bokeh |
|---|---|---|---|
| `Bin` | `Hist` | `Histogram` | `Quad` (histogram) |
| `Count` | `Count` | `Bar` (counts) | `VBar` |
| `Aggregate` | `Agg`, `Est` | `Bar`, `Box` | — |
| `KernelDensity` | `KDE` | `Densitymap` | — |
| `EmpiricalCdf` | `Perc` (percentile) | `Ecdf` | — |
| `Percentile` | `Perc` | `Box` (quartiles) | — |
| `Fit` | `PolyFit` | trendline | — |

- seaborn — <https://seaborn.pydata.org/api.html#seaborn.objects.Hist>
- plotly — <https://plotly.com/python/reference/#plotly.graph_objects.Histogram>

## Recipe

```
data -> [SVStat] -> values -> [SVMove] -> [SVScale] -> SVMark
```

A stat may be absent (a raw scatter) or present (a histogram). Several stats
never compose in one pass: the composition is *one* stat, applied once.

## Data contract

A stat consumes one or more `SVColumnView`s and produces a **derived view**:

| Stat | Input | Output |
|---|---|---|
| `Bin` | 1 continuous column | bin edges + counts per bin |
| `Count` | 1 categorical column | counts per category |
| `Aggregate` | 1 value column + grouping columns | one value per group |
| `KernelDensity` | 1 or 2 continuous columns | density over a grid |
| `EmpiricalCdf` | 1 continuous column | the step function of the empirical CDF |
| `Percentile` | 1 continuous column | requested quantiles |
| `Fit` | 1 continuous column + 1 value column | model coefficients + predicted curve |

### Observation weights

`SVStatInput::with_weights` attaches an optional weight column to any kernel
that accepts one. The weight column follows the same data contract as the
value column: it may be dense, sparse or streaming, it is resolved exactly
once per operation, a sparse weight column is never densified, and a
streaming one is read from its bounded window. A sparse implicit zero is a
zero-weight observation, so it never changes a weighted result.

| Kernel | Weighted | Semantics |
|---|---|---|
| `Bin` | yes | each observation contributes its weight to the bin it falls in; `Count`, `Probability` and `Density` normalise by the total weight |
| `KernelDensity` | yes | the estimate is normalised by the total weight times the bandwidth; Silverman's bandwidth derivation stays unweighted and is documented as such |
| `Aggregate` | yes | `Mean` and `Sum` weight their observations; the remaining methods reject weights explicitly |
| `Count`, `EmpiricalCdf`, `Percentile`, `Fit` | no | the kernel rejects a configured weight column instead of ignoring it |

A negative weight is an error naming the operation and the observation
position (`SVError::NegativeWeight`); a non-finite weight is
`SVError::NonFiniteValue`; a weight column whose length differs from the value
column is `SVError::MismatchedStatisticLengths` at the kernel and
`SVError::MismatchedLengths` when a recipe resolves the columns at build time.
All-ones weights reproduce the unweighted result exactly.

### Representation parity

| Stat | Dense | Sparse | Streaming | Note |
|---|---|---|---|---|
| `Bin` | yes | yes | yes | streaming bins over the rolling window |
| `Count` | yes | yes | yes | |
| `Aggregate` | yes | yes | yes | |
| `KernelDensity` | yes | yes | **windowed** | a density over an unbounded stream is meaningless; the kernel uses the rolling window and documents it |
| `EmpiricalCdf` | yes | yes | **windowed** | same reasoning; the CDF is over the window |
| `Percentile` | yes | yes | **windowed** | quantiles over a rolling window are approximate and are documented as such |
| `Fit` | yes | yes | **windowed** | the fit is recomputed over the window |

A streaming stat never blocks the render thread: it consumes whatever the
window holds at the moment of the operation and returns. The window is
bounded by the source's rollover policy.

## Visual channels

A stat does not draw. It produces values that a later scale maps onto
channels. The output of a stat is always in *data space*, never display
space.

## Configuration

| Kernel | Option | Type | Default |
|---|---|---|---|
| `Bin` | `bin_count` | `Option<usize>` | `None` — defaults to 30 bins when no width or edges are given |
| `Bin` | `bin_width` | `Option<f64>` | `None` |
| `Bin` | `bin_edges` | `Option<Vec<f64>>` | `None` — explicit edges win |
| `Bin` | `range` | `Option<(f64, f64)>` | `None` — derived from the observations |
| `Bin` | `normalise` | `SVNormalisation` | `Count` (`Count`, `Probability`, `Density`) |
| `Bin` | `cumulative` | `bool` | `false` |
| `Count` | `order` | `SVCountOrder` | `DataOrder` (`DataOrder`, `Ascending`, `Descending`) |
| `Aggregate` | `method` | `SVAggregateMethod` | `Mean` (`Mean`, `Median`, `Sum`, `Minimum`, `Maximum`, `StandardDeviation`, `Count`) |
| `Aggregate` | `error` | `Option<SVAggregateErrorKind>` | `None` (`StandardDeviation`, `StandardError`, `Percentile`) |
| `KernelDensity` | `bandwidth` | `Option<f64>` | `None` — derived by rule of thumb |
| `KernelDensity` | `kernel` | `SVKernelShape` | `Gaussian` |
| `KernelDensity` | `grid_count` | `usize` | `256` |
| `KernelDensity` | `cut` | `f64` | `3.0` bandwidths beyond the data range |
| `EmpiricalCdf` | `normalise` | `bool` | `true` |
| `Percentile` | `quantiles` | `Vec<f64>` | `[0.25, 0.5, 0.75]` |
| `Fit` | `model` | `SVFitModel` | `Linear` (`Linear`, `Polynomial`, `Exponential`, `Logarithmic`, `Power`) |
| `Fit` | `order` | `usize` | `1` (polynomial degree) |
| `Fit` | `confidence` | `Option<f64>` | `None` — confidence band |

## Theme

Stats consume no tokens. They produce values; the recipe decides how those
values are coloured.

## Temporal

| Property | Behaviour |
|---|---|
| bin boundaries | fixed across a transition unless the axis range changes; a moving boundary is a re-bin, not an interpolation |
| counts | interpolate numerically between keyframes, matched by bin index |
| density values | interpolate numerically |
| fit coefficients | interpolate numerically; the predicted curve is recomputed per frame |
| new categories | enter with the transition's enter rule |

A streaming stat is *not* animated between frames: it is recomputed. The
animation layer treats a streaming source as a keyframe source with an
implicit, continuously updated target.

## Backend notes

Stats are backend-independent by construction: they run before any backend is
consulted. The backend only ever sees the derived values.

The heavy kernels (`KernelDensity`, `Fit`) expose a scoped compute path:
`SVStat::spawn_on(scope, input)` hands the kernel to a caller-provided thread
scope while the borrowed columns stay alive, and `SVStat::is_heavy()` reports
whether a variant is one of them. Offloading is the caller's decision; the
kernel never starts a runtime of its own.

## Acceptance

- [x] Each stat dispatches on `SVColumnView` exactly once per operation.
- [x] A sparse input is never densified; the stat reports the same result as
      the equivalent dense input within the documented tolerance.
- [x] A streaming input never blocks the render thread and never exceeds the
      window.
- [x] The `Bin`, `Aggregate` and `Count` kernels produce identical output
      for dense, sparse and streaming inputs of the same data.
- [x] The window-dependent kernels (`KernelDensity`, `EmpiricalCdf`,
      `Percentile`, `Fit`) accept a streaming view, operate on its bounded
      window and report `windowed`; none of them blocks on the stream.
- [x] `Fit` rejects only sparse storage, and only as a documented static-only
      exception: two independently sparse columns describe implicit zeros at
      different positions. The error names the alternative path (bind dense or
      streaming columns, or pre-aggregate the sparse view).
- [x] The fit conditions its independent column before solving, so a line at
      `x ≈ 1.7e9` is recovered rather than flattened; regression tests cover
      `x ≈ 1e6` and `x ≈ 1.7e9`, including a degree-2 case.
- [x] Every `SVStat::apply` records one span with `stat.kind`, the value
      column's `representation`, `points`, `duration` and `outcome`.
- [x] `Bin` and `KernelDensity` accept an observation weight column of any
      representation, resolve it once, never densify it, and reproduce the
      unweighted result exactly when every weight is one.
- [x] A negative, non-finite or wrong-length weight column is a typed error
      naming the operation and, where it applies, the observation position.
- [x] Weighted binning and weighted kernel density are covered by
      representation-parity and concurrency tests.
- [x] Every kernel has companion tests in a `*_tests.rs` module beside its
      implementation.

## Open questions

- Should `Fit` support robust regression (Huber, quantile) as a first-class
  model, or as a user-supplied function? *Answered: a user-supplied function
  behind `SVFitEstimator`, called through `sv_fit_with_estimator`, so the core
  stays dependency-light and the closed stat set is unchanged.*
- Should `Aggregate` support a weighted mean over a weight column in the first
  version? *Answered: yes — `Mean` and `Sum` accept weights through
  `SVStatInput::with_weights`; the other methods reject them explicitly.
  `Bin` and `KernelDensity` accept the same weight column; the remaining
  kernels reject a configured weight rather than ignoring it.*
- What tolerance defines "the same pixels" for a sparse input in the
  `data-view-boundary` sense? *Answered: dense against sparse is exact and
  dense against streaming uses `SVRepresentationTolerance::DEFAULT`.*
