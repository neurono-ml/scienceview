# Proposal

## Why

`SVStatInput::with_weights` already existed for aggregates, but the two kernels
that shape the distribution recipes — `SVStat::Bin` and
`SVStat::KernelDensity` — ignored the weight column. PR #219 recorded the gap as
a dated deferral in `histogram-plot` (a configured `weight` was rejected at build
time) and in `density-plot` (the recipe omitted the column entirely), on the
promise that "the `statistical-transforms` capability" would close it.

Weighted observations are a first-class scientific need: survey weights,
frequency tables and importance weights all change the estimate while keeping
the same observations, and users of the Python plotting libraries this project
covers expect `weights=` to work on a histogram and on a KDE.

## What Changes

- `SVStat::Bin` consumes `SVStatInput::weights()`: weighted counts, weighted
  probability/density normalisation, typed rejection of a negative or
  non-finite weight at its logical position, and sparse implicit zeros weighted
  without densification.
- `SVStat::KernelDensity` consumes the same input: a weighted estimate
  normalised by the total weight (so it still integrates to one), the same
  typed weight validation, and sparse implicit zeros weighted by their own
  weight.
- An aggregate that receives a negative weight now reports the same typed
  `SVError::NegativeWeight` instead of a message-only configuration error, so
  every weight-accepting kernel shares one error surface.
- `SVHistogramPlot::weight(...)` stops being a build-time rejection and flows to
  the bin kernel, aligned with the value column through series splitting and
  missing-value omission.
- `SVDensityPlot::weight(...)` is added and flows to the density kernel, with
  the same alignment, including the sparse-value path that zeroes the weight of
  a dropped stored entry instead of densifying.
- FDRs (`core-stat`, the two recipe records), `CHANGELOG.md` and the mdBook gain
  the weighted behaviour; the histogram and density chapters drop their
  "weights deferred" notes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `statistical-transforms`: weighted binning and weighted kernel density, plus
  the shared typed weight error.

## Impact

- `crates/scienceview_core/src/stat/weights.rs` (new), `bin.rs`, `density.rs`,
  `aggregate/apply.rs`, the `SVError` definition and their companion tests.
- `src/recipes/distribution/histogram_plot/preparation/*` and
  `density_plot/preparation/*`, `builder/*` and their tests.
- `docs/FDRs/core/core-stat.md`,
  `docs/FDRs/recipes/distribution-histogram-plot.md`,
  `docs/FDRs/recipes/distribution-density-plot.md`, `CHANGELOG.md`.
- mdBook: new `ch53-weighted-statistics.md`, plus the histogram and density
  chapters.
