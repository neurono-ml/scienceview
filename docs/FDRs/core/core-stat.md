# FDR-002 — `SVStat`

| Field | Value |
|---|---|
| Id | FDR-002 |
| Name | `SVStat` |
| Family | Core — computation |
| Status | planned |
| Layer | Core |
| Public chapter | _pending_ |
| Example program | _pending_ |

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
| `Bin` | `bin_count` | `usize` | derived from the available width |
| `Bin` | `bin_width` | `Option<f64>` | `None` |
| `Bin` | `bin_edges` | `Option<Vec<f64>>` | `None` — explicit edges win |
| `Bin` | `range` | `Option<(f64, f64)>` | `None` |
| `Bin` | `normalise` | `SVNormalisation` | `None` (`Count`, `Probability`, `Density`) |
| `Bin` | `cumulative` | `bool` | `false` |
| `Count` | `sort` | `SVCategoryOrder` | `DataOrder` |
| `Aggregate` | `method` | `SVAggregateMethod` | `Mean` (`Mean`, `Median`, `Sum`, `Min`, `Max`, `StdDev`, `Count`) |
| `Aggregate` | `error` | `Option<SVErrorKind>` | `None` (`StdDev`, `StdError`, `Percentile`) |
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

The heavy kernels (`KernelDensity`, `Fit`) run on the compute pool, never on
the render thread, per the asynchrony principle of the foundation.

## Acceptance

- [ ] Each stat dispatches on `SVColumnView` exactly once per operation.
- [ ] A sparse input is never densified; the stat reports the same result as
      the equivalent dense input within the documented tolerance.
- [ ] A streaming input never blocks the render thread and never exceeds the
      window.
- [ ] The `Bin`, `Aggregate` and `Count` kernels produce identical output
      for dense, sparse and streaming inputs of the same data.
- [ ] The window-dependent kernels (`KernelDensity`, `EmpiricalCdf`,
      `Percentile`, `Fit`) document their window semantics and reject a
      streaming view with `UnsupportedRepresentation` only where the window is
      genuinely insufficient.
- [ ] Every kernel has companion tests in a `*_tests.rs` module beside its
      implementation.

## Open questions

- Should `Fit` support robust regression (Huber, quantile) as a first-class
  model, or as a user-supplied function? *Current answer: a user-supplied
  function, behind a trait, so the core stays dependency-light.*
- Should `Aggregate` support a weighted mean over a weight column in the first
  version? *Open.*
- What tolerance defines "the same pixels" for a sparse input in the
  `data-view-boundary` sense? *Open — it must be fixed before the first
  implementation.*
