# Maintaining this book

This part is a didactic, order-of-PRs textbook explaining the *why* behind
every `scienceview` change, for a reader with no Rust or numerical-computing
background.

- A new feature, figure, artist, kernel, or method adds a **new** chapter
  (`chNN-<slug>.md`), registered in `SUMMARY.md` under this part.
- A bug fix or refactor that changes existing behavior **edits** the existing
  chapter covering that behavior; fixes amend, they do not duplicate.

Every chapter is grounded in the real code and specs: the companion
`*_tests.rs` modules are the executable source of truth walked through as
examples.

## Chapters

| Chapter | File | Subject |
|---|---|---|
| 1 | `ch01-product-foundation.md` | The product foundation: the two audiences, the trait/enum split, the four-stage interaction pipeline, and the open decisions |
| 2 | `ch02-workspace-bootstrap.md` | The Cargo workspace: the crate map, the feature map, the pinned toolchain and the four CI gates |
| 3 | `ch03-theme-tokens.md` | Design tokens from zero: the stylesheet source, build-time and runtime compilation, palettes and colour maps, motion tokens and reduced motion |
| 4 | `ch04-data-source-layer.md` | The data boundary: `SVDataSource`, the three representations, sparse indices, the bounded window, dispatch once and the parity harness |
| 5 | `ch05-figure-builder-core.md` | The figure substrate: the builder typestate, the required explicit backend, per-operation dispatch, one error type and one operation span |
| 6 | `ch06-scale-mapping.md` | The scale kernel: the four kinds, explicit and derived domains across the three representations, the four transforms and their inverses, stable nominal colours, theme-bound continuous colour and tick derivation from the granted space |
| 7 | `ch07-layout-solver.md` | The layout solver: the length model, constraints, the measure and solve passes, the four responsiveness modes, the viewport and grid-independence, and the property tests |
| 8 | `ch08-mark-rendering.md` | Marks: the closed nine-variant vocabulary, the representation parity of the geometry kernel, backend-agnostic primitives, theme defaults and overrides, build-time validation and the interpolable-property table |
| 9 | `ch09-chart-decorations.md` | Decorations: the measure/solve protocol, space-adaptive ticks and label collision avoidance, the legend entry lifecycle and bounded layout, the colour bar as a scale view, titles, text boxes and themed primitives |
| 10 | `ch10-statistical-transforms.md` | Statistical transforms: the closed seven-variant `SVStat`, non-densifying sparse reads and streaming windows, binning, counting, aggregation with Welford and weighted means, percentiles without expansion, kernel density and bandwidth rules, the empirical CDF, the five fit models with confidence bands and the estimator trait, determinism and the scoped compute pool |
| 11 | `ch11-positional-moves.md` | Moves: the closed six-variant vocabulary, the enforced composition order and its typed errors, diverging stacks, group-count dodges, keyed deterministic jitter and swarm, normalisation by contribution and representation parity |
| 12 | `ch12-svg-backend.md` | The SVG backend: the document builder, deterministic attribute order and number format, escaping, self-contained PNG rasters, grouped paths, accessibility, registry dispatch and the examples |
| 13 | `ch13-mark-transitions.md` | Transitions: typed match keys and duplicate detection, the enter/update/exit sets, interpolation driven by the mark kernel's property table, theme motion tokens and reduced motion, the clock-free progress model, the backend capability contract, parity and concurrency |
| 14 | `ch14-animation-timeline.md` | Animation: the model-owned clock, keyframe deltas and the transition seam, fixed and streaming sources over one bounded window, sampling and easing, playback state, looping and direction, interruptible re-targeting, the action vocabulary and the reproducibility, parity and concurrency tests |
| 15 | `ch15-native-backend.md` | The native backend: two layers, straight-alpha blending, distance and sampled anti-aliasing, the device pixel ratio, dirty regions, the model-clock frame loop, resize and scale change, and the capability refusal of export |
| 16 | `ch16-scatter-plot.md` | The first relational recipe: the channel surface, dense/sparse/streaming parity, the jitter and dodge displacement moves, per-point size and colour on top of the dot kernel, the overplotting policy, the backend-agnostic scene and its SVG/native dispatch, concurrency and the operation span |
| 17 | `ch17-line-plot.md` | The second relation recipe: long-format columns, three representations, stable ordering, gaps and bridging, aggregation with an interval, step and spline vertices, multi-series colour, markers, observability and SVG rendering |
| 18 | `ch18-area-plot.md` | The first relational recipe: the area chart, the baseline modes, stacking through `SVMove::Stack`, the ribbon versus area decision, the missing-value policy, representation parity and the generated figure |
| 19 | `ch19-step-plot.md` | The relational step recipe: `SVStepPosition` pre/post/mid geometry, ordered and deduplicated samples, missing values, aggregation, multi-series colouring, backend-neutral composition and the real SVG document |
| 20 | `ch20-band-plot.md` | The band plot recipe: the filled region between two boundaries, explicit and derived modes over `SVStat::Aggregate`, series, missing-value policy, dashed borders and the centre line, parity and concurrency |
| 21 | `ch21-regression-plot.md` | Regression: the multi-mark recipe, the kernel-owned fit and confidence band, pairing across dense, sparse and streaming input, series grouping, extrapolation policy and the acceptance tests |
| 22 | `ch22-histogram-plot.md` | The distributional histogram recipe: binning through `SVStat::Bin`, count/probability/density and cumulative heights, bin count/width/edge modes, series with common bins and common normalisation, stacking and dodging through `SVMove`, missing-value policy, representation parity including sparse implicit zeros, the bar/step/polygon elements, SVG and native dispatch, observability and the deferred weight and in-place streaming paths |
| 23 | `ch23-density-plot.md` | Density plots: `SVStat::KernelDensity` with three kernel shapes, Silverman's bandwidth and `bandwidth_adjustment`, the fixed evaluation grid and `cut`, the `normalise` modes, series grouping, missing-value policy, sparse and streaming parity, SVG/native dispatch and the generated figure |
| 24 | `ch24-density-contour-plot.md` | Density contours: the binned joint density over the bin kernel, marching squares and loop stitching, nested filled bands, the data boundary, backends and the acceptance tests |
| 25 | `ch25-ecdf-plot.md` | The ECDF recipe: the empirical cumulative distribution through `SVStat::EmpiricalCdf`, exact horizontal-then-vertical step geometry, normalisation and the derived survival curve, orientation, series grouping, missing values, non-densifying sparse reads, streaming windows and SVG/native serialization |
| 26 | `ch26-rug-plot.md` | The distributional rug recipe: the one-dimensional marginal tick, the line-kernel segment composition, the value and depth scales, orientation and the axis margin, series colours, missing observations, sparse stored-entry semantics, the bounded compound path and the acceptance tests |
| 27 | `ch27-box-plot.md` | Box plots: the percentile-owned five-number summary, the whisker rules, outliers as dots, notches, series dodging, dense/sparse/streaming parity and the SVG/native dispatch |
| 28 | `ch28-violin-plot.md` | Violin plots: the kernel-density silhouette, bandwidth and adjustment, the Area/Count/Width scales, inner box and jittered sample points, split series, dense/sparse/streaming parity and the SVG/native dispatch |
| 30 | `ch30-strip-plot.md` | Strip plots: the one-dimensional dot per observation, the keyed `SVMove::Jitter` displacement, orientation, series colours, marker size and alpha, the missing-value policy, sparse stored-entry semantics, SVG/native dispatch and the parity, concurrency and observability tests |
| 31 | `ch31-swarm-plot.md` | Swarm plots: the collision-free `SVMove::Swarm` layout, the data-space marker extent that makes the kernel's test a pixel test, both orientations, series colours, the missing-value policy, sparse stored-entry semantics, parity, concurrency, the span and the SVG/native dispatch |
| 32 | `ch32-bar-plot.md` | Bar plots: one aggregate per category through `SVStat::Aggregate`, the estimator and confidence surface, weights, dodge and stack through `SVMove`, ordering, missing-value policy, dense/sparse/streaming parity, SVG/native dispatch and the two explicitly deferred lines |
| 33 | `ch33-count-plot.md` | Count plots: counting through `SVStat::Count`, the shared category axis across series, ordering rules, normalisation, stacking and dodging through `SVMove`, category labels, missing-value policy, sparse implicit zeros, streaming updates and the SVG/native dispatch |
| 34 | `ch34-point-plot.md` | Point plots: one aggregate estimate per category and series through `SVStat::Aggregate`, the standard-deviation, standard-error and percentile intervals, capped error bars, dodging through `SVMove::Dodge`, the optional joining line, ordering, missing-value policy, dense/sparse/streaming parity and the SVG/native dispatch |
| 35 | `ch35-heatmap-plot.md` | Heatmaps: a rectangular matrix drawn as coloured cells through `SVStat::Aggregate` pivot, the continuous colour scale and its `SVColourBar`, annotations, row/column labels, the missing-cell policy, square cells and axis orientation, dense/sparse/streaming parity and the SVG/native dispatch |
| 36 | `ch36-contour-plot.md` | Contours: a supplied scalar field read from a matrix or interpolated from a point cloud through `SVStat::Bin`, level count/range/size selection, marching squares and stitched closed regions, the `Lines`/`Filled` styles, the missing-value policy, sparse implicit zeros, streaming windows, labels, and the SVG/native dispatch |

Chapter 3 covers theming, chapter 4 covers the data boundary, chapter 5
covers the figure substrate, chapter 6 covers the scale kernel, chapter 7
covers the layout solver, chapter 8 covers the mark kernel, chapter 9
covers the decoration surface, chapter 10 covers the statistical transform
kernel, chapter 11 covers the positional-move kernel, chapter 12 covers the
first rendering backend, chapter 13 covers the transition kernel, chapter 14
covers the animation timeline, chapter 15 covers the native backend and
chapter 21 covers the regression recipe, each written by its own change. Later chapters are planned, one per capability of the
chapter 20 covers the band plot recipe, each written by its own change. Later chapters are planned, one per capability of the
covers the animation timeline, chapter 15 covers the native backend and chapter
19 covers the first relational recipe, each written by its own change. Later
chapters are planned, one per capability of the
`foundation-core-architecture` change — `backend-selection`, `data-view-boundary`, `model-events`, `event-payloads`,
covers the animation timeline, chapter 15 covers the native backend, and
chapters 16 to 22 cover the shipped recipes (scatter, line, area, step, band,
regression and histogram), chapter 27 covers box plots and chapter 32 covers bar
plots, each written by its own change. Later chapters are
planned, one per capability of the
`foundation-core-architecture` change —
`backend-selection`, `data-view-boundary`, `model-events`, `event-payloads`,
`backend-capabilities`, `theming`, `layout-engine`, `ui-kit`, `scripting` and
`dashboard` — and are written as the corresponding implementation changes
land, so that each one can walk through real code and real companion tests.
The plan lives in
`openspec/changes/foundation-core-architecture/documentation-plan.md`.

Chapter 29 covers the boxen plot recipe, which nests letter values from the
percentile kernel into `SVBoxenPlot`: it explains the depth rules, the nested
widths and the sparse and streaming parity tests.

Chapter 37 covers the density heatmap recipe, which asks the bin kernel for the
two edge vectors, folds the observations through the aggregate kernel and
paints one rectangle per cell over a continuous colour scale; it explains the
normalisation choices, the empty-bin policy and the sparse and streaming
parity tests.
Chapter 35 covers the heatmap recipe, which turns a long table or a direct
matrix into coloured cells through `SVStat::Aggregate` and draws the colour bar
from the same continuous scale the cells use.

## Diagrams

Diagrams are always Mermaid or SVG, colourful and harmonized with the mdBook
theme; ASCII-art diagrams are forbidden. A diagram is authored as Mermaid
source and rendered to SVG, and the SVG is checked in under
`docs/src/development-explained/assets/` so the book renders without a
JavaScript dependency.
