# FDR-111 — `SVHistogramPlot`

| Field | Value |
|---|---|
| Id | FDR-111 |
| Name | `SVHistogramPlot` |
| Family | Distributional |
| Status | shipped |
| Layer | Recipe |
| Core | [core-mark](../core/core-mark.md), [core-stat](../core/core-stat.md), [core-move](../core/core-move.md), [core-scale](../core/core-scale.md), [core-transition](../core/core-transition.md), [core-animation](../core/core-animation.md) |
| Public chapter | [Chapter 22 — Histograms](../../src/development-explained/ch22-histogram-plot.md) |
| Example program | [`examples/histogram_plot.rs`](../../../examples/histogram_plot.rs) |

## Purpose

`SVHistogramPlot` bins a continuous variable and draws the count, probability
or density in each bin. It is the reference recipe for the stat layer: the
chart is a bar mark plus a binning stat, and almost every other distributional
recipe is a variation of that arrangement.

## Sources

| Library | Function | Link |
|---|---|---|
| seaborn | `histplot` | <https://seaborn.pydata.org/api.html#seaborn.histplot> |
| seaborn | `objects.Hist` | <https://seaborn.pydata.org/api.html#seaborn.objects.Hist> |
| bokeh | `Quad` | <https://docs.bokeh.org/en/latest/docs/reference/models/glyphs.html#bokeh.models.Quad> |
| plotly | `go.Histogram` | <https://plotly.com/python/reference/#plotly.graph_objects.Histogram> |

## Recipe

```
data -> [SVStat::Bin] -> (edges, counts) -> [SVMove::Stack | Dodge]
     -> [SVScale x, y, colour] -> SVMark::Bar
```

## Data contract

| Column | Role | Type | Required |
|---|---|---|---|
| `value` | the variable to bin | continuous | yes |
| `series` | grouping; one set of bins each | nominal | no |
| `weight` | observation weight | continuous | no |

Accepted representations: dense, sparse, streaming. For a streaming source the
bins are computed over the rolling window. The in-place update of the drawn
bars is deferred (see Temporal and Acceptance).

Weighted binning is deferred: the shipped `SVStat::Bin` kernel does not consume
observation weights, so a configured `weight` is rejected at build time with a
configuration error that names the kernel gap. The recipe never reimplements
weighted binning; it belongs to a change of the `statistical-transforms`
capability.

### Match key

`Category` on the bin index within a series. Bins are matched by index, not by
their edge value, so that a bin keeps its identity when the domain shifts.

## Visual channels

| Channel | Encodes | Scale |
|---|---|---|
| x | bin edges | `Continuous` |
| y | count, probability or density | `Continuous` |
| fill colour | `series` | `Nominal` |

## Configuration

| Option | Type | Default | Source |
|---|---|---|---|
| `value` | `SVColumnRef` | — | all three |
| `series` | `Option<SVColumnRef>` | `None` | all three |
| `bin_count` | `Option<usize>` | derived from the axis width | seaborn, plotly |
| `bin_width` | `Option<f64>` | `None` | seaborn, plotly |
| `bin_edges` | `Option<Vec<f64>>` | `None` | seaborn |
| `normalise` | `SVNormalisation` | `Count` | seaborn, plotly `histfunc` |
| `cumulative` | `bool` | `false` | seaborn `cumulative` |
| `common_normalisation` | `bool` | `false` | seaborn |
| `common_bins` | `bool` | `true` | seaborn |
| `shrink` | `f32` | `1.0` | seaborn `shrink` — the gap between bars |
| `element` | `SVBarElement` | `Bar` | seaborn (`bars`, `step`, `poly`) |
| `orientation` | `SVBarOrientation` | `Vertical` | bokeh, plotly |

`element` selects how the bins are drawn: as separate bars with a gap, as a
stepped outline, or as a filled polygon. The step and polygon forms are the
`SVStepPlot` and `SVAreaPlot` geometries over bin counts, so they share their
kernels rather than duplicating them.

## Theme

Consumes `mark.palette`, `mark.size` for the bar gap, `text.*`, axis tokens.
The histogram is the canonical consumer of the discrete part of the series
palette.

## Temporal

| Property | Behaviour |
|---|---|
| bar height | interpolates |
| bin edge | interpolates when the domain shifts; a bin that changes identity is an exit plus an enter |
| new bin | enters with the enter rule and grows to its height |
| normalisation change | the heights morph into the new normalisation |

A streaming histogram is the archetypal live chart: bins grow in height as
observations arrive, and the bin boundaries stay put.

Transition wiring is deferred in this change. The recipe produces
transition-ready geometry — bins keep their identity by index and the heights
live in data space — and `update` recomposes the current window, but no
enter/update/exit animation path runs yet. That integration belongs to the
transition/streaming capability; the FDR records the target behaviour above and
the acceptance checklist does not tick it.

## Backend notes

| Backend | Notes |
|---|---|
| `Svg` | one `<rect>` per bin |
| `Html` | same |
| `Native` / `Wasm` | filled rectangles, snapped to the cell grid in the terminal |
| `Tui` | one cell column per bin, with block characters for partial heights |
| `React` / `Vue` / `Angular` | one `<rect>` per bin |
| `Tauri` / `Dioxus` | same as native |

## Acceptance

- [x] The bin edges follow the requested count, width or explicit edges. The
      companion tests cover the count, width and explicit-edge modes.
- [x] `normalise` yields counts, probabilities or densities that sum or
      integrate as documented. Probability sums to one; density integrates to
      one over the bin widths.
- [x] `cumulative` yields a monotone non-decreasing series.
- [x] Dense, sparse and streaming inputs give the same histogram for the same
      data, within the documented tolerance. The parity tests compare the
      serialized SVG byte for byte.
- [ ] A streaming histogram updates in place, without redrawing the whole
      figure. *Deferred:* `update` recomposes the current window; a
      backend-incremental bin update belongs to the transition/streaming
      integration.
- [x] The number of bins adapts to the space available. With no boundary option
      the recipe derives the count from its plot extent at 14 logical pixels per
      bin, clamped to `4..=40`; a layout-driven composition supplies that extent.

## Open questions

- Should the bin count be derived from the *pixel* width of the axis, or from
      the data only? *Current answer: derived from the available width, with
      the data range as the other input — the composition owns the width, the
      stat owns the range. The recipe's plot extent stands in for the width a
      layout-driven composition will supply.*
- Should `shrink` be a mark option rather than a recipe option? *Current
      answer: a recipe option, since it changes the bar width, and the bar
      mark carries a width that the recipe computes.*
