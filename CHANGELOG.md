# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The section between two releases describes the changes of the version being
released.

## [Unreleased]

### Added

- The distributional recipe `SVRugPlot` (`scienceview::sv_recipes::sv_distribution`):
  one small tick per observation, drawn in the margin of the axis the values
  occupy. The configuration surface follows FDR-115 — `value`, `series`,
  `orientation`, `height`, `alpha`, `line_width`, `title` and `value_label` —
  and the builder requires an explicit backend selection. Every tick is the
  two-point `SVMark::Line` segment geometry of the closed mark vocabulary; the
  recipe places each stored observation once through a continuous value scale
  and a bounded depth scale that spans the margin, so the ticks never enter the
  plot area. Dense, sparse and streaming input agree byte for byte, a sparse
  column marks only its stored support without being densified, and a large
  sample is grouped into one compound path per paint so the document stays
  bounded. Non-finite observations draw no tick. The new example
  `examples/rug_plot.rs` generates `docs/figures/rug_plot.svg`.
- The relational recipe `SVRegressionPlot` (`scienceview::sv_recipes::sv_relational`):
  a scatter overlaid with a fitted curve and, optionally, a confidence band.
  The configuration surface follows FDR-106 — `x`, `y`, `series`, `model`,
  `order`, `confidence`, `show_points`, `show_fit`, `marker`, `line_width` —
  and the builder requires an explicit backend selection. The fit is never
  reimplemented: `SVStat::Fit` supplies the coefficients, the grid, the
  predictions and the confidence bounds for the linear, polynomial,
  logarithmic, exponential and power models. The recipe pairs the predictor and
  the response once per operation, groups observations by `series` and fits one
  model per group, drops non-finite pairs before calling the kernel, derives
  the two domains from the points and the band, and composes `SVMark::Dot`,
  `SVMark::Line` and `SVMark::Ribbon` over one axes and one legend. Dense,
  sparse and streaming input agree within the documented tolerance; a sparse
  pair is read through its stored support without materialising implicit zeros,
  and a mismatched sparse support is rejected with the kernel's documented
  alternative. `SVExtrapolationPolicy` clips the curve to an explicit visible
  domain instead of inventing predictions beyond the fitted range. The new
  example `examples/regression_plot.rs` generates
  `docs/figures/regression_plot.svg`.
- The band plot recipe `SVBandPlot` in
  `scienceview::sv_recipes::sv_relational::sv_band_plot`: the filled region
  between two boundary lines, composed exactly as
  `data -> [SVStat::Aggregate + error] -> (lower, upper) -> [SVScale] ->
  SVMark::Ribbon`. The builder requires an explicit backend, defaults
  `execution_mode` to `Automatic`, and accepts either explicit `lower`/`upper`
  columns or a derived mode that aggregates a value column per `x` group with
  `SVAggregateErrorKind` (standard deviation by default, or a centred
  percentile interval selected with `confidence`). One optional `series`
  column draws one band per series. Fill opacity, an optional dashed or solid
  border, an optional centre line, titles and an explicit
  `SVBandMissingValuePolicy` (omit and split the band at a gap, or reject the
  render) are part of the surface. The x scale and the columns are validated
  at `build`, so a nominal/boolean axis or a missing/wrong-role column fails
  there rather than at `render`. Dense, sparse and streaming input resolve once
  per operation; a sparse column contributes only its stored entries (a huge
  logical length with a few stored values draws a few points) and a streaming
  window renders without blocking and reports `is_approximate`. The composed
  geometry serializes through `to_svg()` and, with `render-native`, through
  `to_native_frame()`; any other configured kind fails with a typed error
  instead of being ignored. Transition and animation behaviour is deferred to
  the transition integration. The example `examples/band_plot.rs` calls the
  public `to_svg()` and writes `docs/figures/band_plot.svg`.
- The `SVStepPlot` relational recipe: a value held constant between discrete
  events, drawn as explicit step vertices through the existing line mark. It
  composes the documented pipeline — source columns, optional
  `SVStat::Aggregate`, scales and `SVMark::Line` — and exposes `SVStepPosition`
  (`Before`, `After`, `Center`), `SVStepDuplicatePolicy` (`LastWins`,
  `FirstWins`), `x`/`y`/`series` columns, a continuous or temporal `x` scale,
  line width, dash, markers, aggregate
  and a title. Dense, sparse and streaming columns are read through the single
  `SVDataSource` boundary without densifying sparse storage or blocking on a
  stream; the operation span carries the configured backend kind, the
  representation and the number of observations the scene actually draws. The
  chart renders through the real SVG backend; a configured kind without a
  serializer returns the typed `BackendUnavailable` error, and
  `examples/step_plot.rs` produces `docs/figures/step_plot.svg`.

- The relational recipe `SVAreaPlot` in `scienceview::sv_recipes::sv_relational`: an
  area chart that fills the region between an ordered boundary and a baseline.
  The builder requires an explicit backend and defaults to
  `SVExecutionMode::Automatic`; it exposes `x`, `y`, an optional `series`,
  `stack`, `baseline` (`Zero`, `Minimum`, `Maximum` or `Value`), `opacity`,
  `line`, `interpolate` and a missing-value policy, plus axis titles, scale kind
  and viewport. Non-stacked areas close against the configured baseline with
  `SVMark::Area`; stacked areas use `SVMove::Stack` and draw the region between
  the cumulative baseline and the cumulative top with `SVMark::Ribbon`. A
  non-finite sample is omitted and splits the area, or rejected under the strict
  policy. Dense, sparse and streaming columns are accepted, each resolved once
  per operation, and the operation span records the backend kind, the
  representation and the point count. `rendering()` returns the ordered
  backend-agnostic scene; `render` dispatches it to the serializer of the
  configured kind — the SVG document or the native raster frame — and fails
  with a typed capability error for a kind the recipe does not serialize yet.
  Sparse columns are never densified: only stored entries and the boundaries of
  implicit-zero runs become vertices.
- The `SVLinePlot` recipe (`src/recipes/relational/line_plot/`, public path
  `scienceview::sv_recipes::sv_relational::sv_line_plot`): one line per
  `(series, group)` over an ordered `x` axis, composed from the existing data,
  stat, scale, mark and decoration kernels. It reads named columns from any
  `SVDataSource` (with typed `ColumnNotFound` errors), accepts dense, sparse
  and streaming channels, sorts each series by `x` with a stable order, splits
  the line at missing values unless `bridge_missing(true)` connects them, and
  supports per-series colour and dash encoding, line width, markers, step or
  Catmull-Rom interpolation, optional aggregation with an estimator and an
  uncertainty interval, a configurable legend placement and continuous or
  temporal `x` scales. The SVG and native backends serialize one composed
  scene, and `examples/line_plot.rs` generates
  `docs/figures/line_plot_recipe.svg`.
- The first relational recipe, `SVScatterPlot` (`scienceview::sv_recipes::sv_relational::sv_scatter_plot`):
  the joint distribution of two continuous columns with optional `series`,
  `colour` and `size` channels, a constant `alpha` for overplotting, a marker
  shape, edge colour and width, the jitter and dodge displacement moves, and
  axes, grid, title and legend. It composes the existing data, scale, move,
  mark and decoration kernels — no kernel variant is added — resolves dense,
  sparse and streaming columns once per operation without densifying or
  blocking, and records the backend kind, representation and the point count
  the current operation actually resolved in one operation span. `compose()`
  returns a backend-agnostic `SVScatterScene`; `render()` dispatches through the
  configured kind — the SVG document for `Svg`, the native frame for `Native`,
  a typed capability error for a kind with no recipe serializer and
  `BackendUnavailable` for a compiled-out backend — and each scene serializer
  rejects a mismatched kind. A large scatter groups its circles into one
  compound path per paint. The example `examples/scatter_plot.rs` calls the
  public `to_svg()` method and generates `docs/figures/scatter_plot.svg`.
- The native rendering backend `scienceview_native`: a from-scratch CPU
  renderer that implements `SVRenderBackend` for `SVBackendKind::Native` and
  registers itself in the `SVBackendRegistry`. It is split in two layers. A
  pure rasterizer (`SVRasterCanvas`) owns a straight-alpha RGBA buffer and
  draws flattened paths with non-zero winding, stroked segments with butt,
  round and square caps, round, miter and bevel joins (a miter past the limit
  falls back to a bevel), dashes and anti-aliasing, rectangles with corner radii, circles,
  text laid out as boxes, and borrowed RGBA images with nearest or bilinear
  sampling, all scaled by the viewport's device pixel ratio. A present layer
  (`SVNativeWindow`, compiled only with the `window` feature, which the root
  `render-native` feature enables) blits the buffer through `softbuffer` in a
  `winit` window and re-solves the logical viewport on resize or scale change;
  a streaming producer publishes frames into the open window, which presents
  the newest one in place without reopening. `winit` and `softbuffer` stay out
  of the default build. The frame loop takes
  its clock from the model through `SVFrameClock`, so a paused timeline
  performs no animation work. `render` and `update` return the comparable RGBA
  frame, `show` presents it, and `save` reports the missing export capability
  the native row declares. Dirty regions record what an in-place repaint
  touched, and the headless rasterizer is deterministic across threads.
- The animation timeline in `scienceview_core::sv_animation`: `SVAnimation`
  owns the playback clock and the keyframe window, and `SVAnimationBuilder`
  requires an explicit `SVKeyframeSource` (`Fixed` or `Streaming`) with the
  documented `SVExecutionMode::Automatic` default. A keyframe is an
  `SVKeyframeDelta` over the mark property vocabulary, and
  `SVKeyframeDelta::interpolate` is the seam the transition kernel consumes.
  Time is supplied by the caller through `advance(Duration)` — there is no wall
  clock, timer or async runtime — so playback is reproducible. A fixed source
  validates non-decreasing timestamps at build time (an equal timestamp is a
  zero-length segment that swaps instantly); a streaming source
  accepts arrivals into a bounded window that drops the oldest frame and counts
  accepted, duplicate, out-of-order and evicted keyframes. A keyframe arriving
  mid-transition re-targets from the interpolated position without a jump, and
  playback exposes idle, playing, paused and finished state with loop,
  direction, speed and the `SVAnimationAction` vocabulary. Every operation
  emits one `scienceview.animation` span.
- The transition kernel in `scienceview_core::sv_transition`: `SVTransition`
  matches marks between two states by a stable key — `Category`, `Index`,
  `Identity` or a seeded `Derived` hash, resolved into a typed
  `SVMatchKeyValue` from a dense, sparse or streaming `SVColumnView` with one
  dispatch and duplicate-key detection — and computes the enter, update and exit
  sets. Interpolation follows the mark kernel's interpolable-property table:
  position, size, colour, opacity and font size tween while shape, dash pattern
  and font family hold their previous value and swap at the end. Enter and exit
  rules (`None`, `FadeIn`/`ScaleIn`/`SlideIn`, `FadeOut`/`ScaleOut`/`SlideOut`)
  govern appearances and disappearances. Durations and easings resolve from the
  theme's motion tokens behind the `theme` feature, with documented built-in
  defaults otherwise, and a reduced-motion theme collapses every frame to its
  final state. The kernel owns no clock: evaluation is a pure function of a
  caller-supplied progress in `0..=1`, so it is deterministic and safe to run
  from several threads, and a backend without in-place update resolves the
  transition to its final state.
- The positional-move kernel in `scienceview_core::sv_move`: `SVMove` is the
  closed vocabulary of stack, dodge, jitter, shift, swarm and normalize, applied
  in data space between the statistic and the scale. `SVMoveSequence::new`
  enforces the fixed composition order — one displacement move, then shift,
  then normalize — and rejects any other ordering with typed
  `SVError::MoveOrderViolation` and `SVError::ConflictingMoveStage` before data
  is read. Stacking is diverging (positives up, negatives down, zeros kept at
  zero), dodge widths derive from the per-category group count, jitter and swarm
  are deterministic from the mark key through an in-crate SplitMix64 generator
  with no extra dependency, the swarm layout is collision-free and bounded, and
  normalize divides by the contribution total so a stacked group reaches one
  (or one hundred) within `1e-12`. Every move accepts dense, sparse and
  streaming views, dispatches once, never densifies sparse storage and uses the
  window total over a stream.
- The SVG rendering backend `scienceview_svg`: a zero-dependency crate that
  implements `SVRenderBackend` for `SVBackendKind::Svg` and registers itself in
  the `SVBackendRegistry`, so `SVFigure::render()` works end to end. It
  serializes every mark shape (path, rectangle, circle, text and an RGBA raster
  embedded as a deterministic PNG data URI) and every decoration primitive
  (line, rectangle and rotated text) into a self-contained document with fixed
  attribute order and a fixed numeric format, making two renders byte
  identical. Accessibility metadata (`role="img"`, `aria-label`, `<title>`,
  `<desc>`), `viewBox`, `width` and `height` are always written and every text
  and attribute value is escaped. Consecutive equal-paint circles are grouped
  into one `<path>`, keeping a ten-thousand-point scatter openable. `render`
  and `save` are implemented; `update` and `show` return the typed
  `UnsupportedBackendCapability` error declared by the export-only capability
  row. The `line_plot_svg` and `scatter_svg` examples generate the checked-in
  documentation figures through the backend, and the figures CI job verifies
  they never drift.
- The statistical transform kernel in `scienceview_core::sv_stat`: `SVStat` is
  the closed set of binning, counting, aggregation, kernel density, empirical
  CDF, percentiles and fitting, and every operation resolves the
  `SVColumnView` representation exactly once. Sparse columns contribute their
  implicit zeros as a multiplicity and are never densified; streaming columns
  are read from the bounded window they already carry, and the window-dependent
  kernels report `windowed` instead of pretending to be exact. Binning supports
  explicit edges, widths and counts with count, probability and density
  normalisation; aggregation reports a value plus a standard-deviation, standard
  error or centred-percentile band, uses Welford variance and accepts weights for
  weighted means and sums; percentiles and the empirical CDF sort stored values
  without expanding implicit zeros; kernel density implements Gaussian,
  Epanechnikov and uniform kernels with Silverman's bandwidth rule; and fitting
  covers linear, polynomial, exponential, logarithmic and power models with a
  confidence band, plus the `SVFitEstimator` trait for user-supplied models.
  Non-finite observations are errors with their position, order-independent
  statistics ignore input order, and the heavy kernels expose `is_heavy` and
  `spawn_on` so they can run on a scoped compute thread. The kernel is
  re-exported from the root facade as `scienceview::sv_stat`.
- The decoration kernel in `scienceview_core::sv_decoration`: axes on all four
  sides with ticks derived from the space the layout granted (`SVTickSpace` and
  `SVTickDerivation`) and a deterministic label collision pass, grid lines that
  reuse the placed tick coordinates, a legend whose entries follow the series
  enter/exit lifecycle with a bounded overflow row, a colour bar tied to the
  very `SVContinuousColourScale` the cells use, and titles, subtitles and
  captions. Every decoration measures itself in the measure pass, places itself
  in the solve pass and emits backend-agnostic line, rectangle and text
  primitives; text is measured as a box until a backend rasterizes glyphs. The
  decoration style reads the existing `theme` feature's tokens and mirrors the
  same values in documented built-in defaults when the feature is off.
- The scale kernel in `scienceview_core::sv_scale`: `SVScale` is the closed
  model with exactly four kinds — continuous, nominal, temporal and boolean —
  and each scale owns its data domain and its display range together, so an
  axis and the marks on it cannot disagree while the domain animates. Domains
  are explicit when configured and derived from a dense, sparse or streaming
  `SVColumnView` otherwise, with the representation resolved once and the
  sparse storage never densified.
- `SVScaleTransform` with `Linear`, `Logarithmic`, `SymmetricLogarithmic` and
  `SquareRoot`, each exposing a forward map and its inverse; an undefined value
  is a configuration error instead of a `NaN`.
- Colour scales: `SVScalePalette` assigns a stable palette entry per category
  so a series keeps its colour across updates, while `SVContinuousColourScale`
  interpolates a colour map. Both work without any theme dependency through
  the default palette and colour map; with the new core `theme` feature they
  resolve theme tokens and re-resolve them per call, so a theme recompilation
  restyles continuous encodings without rebuilding the figure.
- The tick derivation hook: `SVTickSpace` turns the axis length and minimum
  label spacing the layout engine grants into a tick budget, and
  `SVScale::ticks` derives values, positions and labels per kind, including
  integer-exponent logarithmic ticks and a readable temporal unit ladder.
  `SVTickDerivation` is the trait a decoration consumes.
- The scale is the first real kernel to adopt the `SVRepresentationParityHarness`
  in `sv_data`, proving dense, sparse and streaming domain derivation agree.
- The layout solver in `scienceview_core` (`sv_layout`): a total length model
  (`SVLength` with absolute, fraction, intrinsic, aspect-ratio and fill
  variants), minimum and maximum constraints, a layout tree solved in two
  passes (measure, then solve) against a backend-supplied `SVViewport`, the
  four explicit responsiveness modes (fixed, fit to parent, aspect-ratio
  constrained, content derived), cached invalidation and revisioning on
  viewport change, spacing seeded by theme tokens with built-in defaults, and
  deterministic clamping — the minimum wins — when constraints contradict.
- The data source layer: `SVDataSource` is the open ingestion contract and
  `SVColumnView` is the closed computation boundary with exactly three
  representations — dense, sparse and streaming. Adapters cover borrowed
  slices, `ndarray` views (`data-ndarray`), `sprs` sparse vectors
  (`data-sprs`), standard-library channels (`streaming`) and Tokio channels
  (`streaming-tokio`); sparse data is never densified. `SVRollingWindow` bounds
  every streaming source and never blocks the render thread, and
  `SVRepresentationParityHarness` proves dense, sparse and streaming outputs
  agree within the documented tolerance (dense against sparse exactly).
- Figure substrate in `scienceview_core`: `SVFigure`, the typestate
  `SVFigureBuilder` that cannot build without a data source and a backend,
  `SVBackendKind` (one variant per backend, deliberately no automatic variant),
  `SVExecutionMode` (defaulting to `Automatic`) and the `SVBackendRegistry`
  through which backend crates — including third-party crates — plug in.
- `SVRenderBackend` as the public rendering contract: `build` transfers the
  configured kind into the figure without dispatching, and every operation
  (`render`, `update`, `save`, `show`) resolves the concrete backend at call
  time. Unsupported capability combinations fail at build time.
- `SVError` as the single public error enumeration, including
  `UnsupportedRepresentation` and backend-unavailable errors.
- One operation span per public operation, carrying `backend.kind`,
  `representation`, `points` and `duration`, with the outcome recorded on
  completion.
- `scienceview_theme`, the design-token crate: one authored stylesheet
  (`stylesheet/scienceview.scss`) compiled at build time into typed colours,
  typography, spacing, radii, shadows and motion tokens, with categorical,
  continuous and data-derived palettes, colour maps, reduced-motion collapse
  and opt-in runtime recompilation that restyles a figure without rebuilding
  it. Exposed through the `theme` feature.
- The repository now builds as a Cargo workspace: `scienceview_core` plus one
  crate per rendering backend (`scienceview_svg`, `scienceview_html`,
  `scienceview_tui`, `scienceview_native`, `scienceview_wasm`,
  `scienceview_react`, `scienceview_vue`, `scienceview_angular`,
  `scienceview_tauri`, `scienceview_dioxus`). The root `scienceview` package is
  a facade that re-exports the core and the enabled backends, `default = []`
  and every backend feature maps 1:1 to its optional path dependency.
- `rust-toolchain.toml` pinning Rust 1.85 with `rustfmt` and `clippy`, and a
  workspace lint set that denies warnings.
- Workspace smoke tests covering core reachability, the default feature set,
  the toolchain pin, the lint configuration, the README badges and this
  changelog entry.
- Feature Description Records for the chart surface (`docs/FDRs/`) — one
  record per core kernel and per scientific-core chart recipe, plus the
  catalogue of deferred chart types and the engineering contract that
  governs their implementation.
- A scaffolded change grid of fifty-two OpenSpec changes implementing the
  chart surface, the kernels it composes and the backends and integrations it
  needs, dispatched as a dependency-ordered DAG of waves.

- Product requirements document (`docs/PRD.md`) establishing the product source
  of truth: the vision, the two audiences the library serves, the
  architectural principles, the capability catalogue, the roadmap and the open
  decisions.
- Ten normative capability specifications covering backend selection, the data
  view boundary, model events, event payloads, backend capabilities, theming,
  the layout engine, the interactive control kit, scripting and dashboards.
- `AGENTS.md` and `openspec/config.yaml` now point at `docs/FDRs/patterns.md`
  as the normative contract for changes that implement the chart surface.
- The mark kernel in `scienceview_core::sv_mark`: the closed `SVMark`
  vocabulary — dot, line, area, bar, ribbon, text, image, wedge and error bar —
  with the option surface of every variant, build-time validation, theme-token
  defaults with explicit overrides, and backend-agnostic geometry primitives
  (`SVMarkShape`, `SVMarkPrimitive`, `SVMarkGeometry`) resolved in the layout's
  coordinate system without rasterizing. Dense, sparse and streaming columns
  are read through the `SVColumnView` boundary — sparse storage is never
  densified and a live window never blocks the render thread — and the
  interpolable-property table is exported for the transition kernel.

### Changed

- The data boundary is exposed through the public modules `sv_data` and
  `sv_window` (the on-disk folders stay `data/` and `window/`), its failures are
  folded into `SVError` so the crate keeps one public error type, and `SVFigure`
  carries the data layer's `SVColumnView` instead of a duplicate representation
  enumeration.
- CI now runs the workspace quality gates — formatting, clippy with warnings as
  errors and the workspace test suite — plus a figures job that reruns every
  example and fails on any drift under `docs/figures/`.
- The README now points at the product requirements document and describes the
  facade-over-workspace layout.

## [0.1.0] - Unreleased

### Added

- Initial repository: builder-style figure description, the trait-driven
  feature-gated rendering backends and the documentation figures generated by
  the tool itself.

[Unreleased]: https://github.com/neurono-ml/scienceview/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/neurono-ml/scienceview/releases/tag/v0.1.0
