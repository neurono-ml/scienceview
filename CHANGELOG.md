# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The section between two releases describes the changes of the version being
released.

## [Unreleased]

### Added

- The multivariate recipe `SVJointPlot` (`scienceview::sv_recipes::sv_multivariate`):
  two variables shown together with the marginal distribution of each — a
  central bivariate panel (scatter, hexbin, density contour or regression)
  with a histogram, density curve or rug on the top and the right, all sharing
  the central scales. The chart is the composition of FDR-145 —
  `data -> [no stat] -> values -> central cell -> marginal cells ->
  [shared SVScale per axis] -> three panels sharing a layout` — and its
  configuration surface is `x`, `y`, `series`, `kind`, `marginal_kind`,
  `marker`, `alpha`, `regression_model`, `missing_value_policy`, the titles,
  with `backend(...)` required and `execution_mode` defaulting to `Automatic`.
  The central panel reuses the scatter mark, the hexbin recipe, the density
  contour recipe and the fit kernel; the marginals reuse `SVStat::Bin`,
  `SVStat::KernelDensity` and the bar and line marks; the three panel
  rectangles are solved by `sv_layout`. Dense, sparse and streaming inputs
  agree, a sparse column is never densified and a streaming window renders
  without blocking. The scene serializes through `to_svg()` and, with
  `render-native`, through `to_native_frame()`; any other configured kind
  fails with a typed error. Transition and animation wiring is deferred to the
  transition integration; the geometry is transition-ready. The example
  `examples/joint_plot.rs` writes `docs/figures/joint_plot.svg`.
- The multivariate recipe `SVPairPlot` (`scienceview::sv_recipes::sv_multivariate`):
  a pair grid over a variable list, with one cell per pair, shared per-variable
  domains, optional `hue` grouping, a configurable diagonal (density or
  histogram) and a configurable off-diagonal shape (scatter, hexbin or a binned
  density contour). The chart follows FDR-144 and composes only the closed
  kernels: dot, bar and line marks, `SVStat::Bin` and `SVStat::KernelDensity`,
  the scale kernel and the layout engine's grid. The hexbin lattice and the
  binned-contour pass are chart-owned geometry, recorded as a trade-off in
  ADR #182 (they mirror the geometry the hexbin and KDE contour recipes own).
  Dense, sparse and streaming
  input produce the same grid; the missing-value policy is explicit. The
  example `examples/pair_plot.rs` writes `docs/figures/pair_plot.svg`.
  Transition and animation wiring is deferred; the geometry is
  transition-ready.
- The multivariate recipe `SVSplomPlot` (`scienceview::sv_recipes::sv_multivariate`):
  a scatterplot matrix over two or more continuous variables — one scatter cell
  per variable pair and a univariate distribution on the diagonal (histogram by
  default, kernel density or empty on request), with the observations shared
  across every cell. The chart is the composition of FDR-143 —
  `data -> [SVStat for the diagonal only] -> values -> [SVScale per axis] ->
  SVMark::Dot per cell` — and its configuration surface is `variables`,
  `colour` (nominal grouping), `diagonal`, `marker`, `size`, `opacity`,
  `scale_mode` (per-variable `Independent` or common `Shared`), `cell_aspect`,
  `show_upper`, `show_lower`, `show_diagonal`, `missing_value_policy`, `title`
  and the backend, with `backend(...)` required and `execution_mode` defaulting
  to `Automatic`. The grid is solved through `sv_layout`; dense, sparse and
  streaming variables render the same matrix, a sparse column is never
  densified, and a streaming window never blocks. The scene serializes through
  `to_svg()` and, with `render-native`, through `to_native_frame()`; any other
  kind fails with a typed error. Transition and animation wiring is deferred to
  the transition integration; the geometry is transition-ready. The example
  `examples/splom_plot.rs` writes `docs/figures/splom_plot.svg`.
- The multivariate recipe `SVParallelCategoriesPlot`
  (`scienceview::sv_recipes::sv_multivariate`): a set of categorical axes whose
  observations become aggregated ribbons between adjacent axes. The chart is
  the composition of FDR-142 —
  `data -> [SVStat::Count] -> category bands per axis -> [SVScale per axis
  (nominal bands)] -> SVMark::Ribbon` — and its configuration surface is
  `variables`, `variable_labels`, `category_labels`, `colour`, `colour_mode`,
  `opacity`, `band_gap`, `category_order`, `label_angle`,
  `missing_value_policy` and `title`, with `backend(...)` required and
  `execution_mode` defaulting to `Automatic`. Every axis is counted through
  `SVStat::Count`, and the observations are aggregated per adjacent-axis
  category pair through the same kernel; the ribbon slice thickness encodes the
  count. Ribbons are coloured by count or by category. Dense, sparse and
  streaming inputs produce byte-identical figures, a sparse column is never
  densified, and a streaming window renders without blocking. The scene
  serializes through `to_svg()` and, with `render-native`, through
  `to_native_frame()`; any other configured kind fails with a typed error.
  Transition and animation wiring is deferred to the transition integration;
  the geometry is transition-ready. The example
  `examples/parallel_categories_plot.rs` writes
  `docs/figures/parallel_categories_plot.svg`.
- The multivariate recipe `SVParallelCoordinatesPlot`
  (`scienceview::sv_recipes::sv_multivariate`): many observations drawn as
  polylines through one vertical axis per continuous variable. The chart is the
  composition of FDR-141 —
  `data -> [no stat] -> values -> [one line per observation] -> [SVScale per axis] -> SVMark::Line` —
  and its configuration surface is `variables`, `colour`, `line_width`,
  `opacity`, `dimensions` (`SVDimension` range, visibility and axis tint),
  `range`, `label_angle`, `axis_label`, `normalization`, `missing`, the title
  and `backend(...)`, with `execution_mode` defaulting to `Automatic`. Every
  visible variable receives its own continuous `SVScale` and axis order comes
  from `variables` or `dimensions`. Each observation is an `SVMark::Line`
  polyline; the optional colour column is a nominal grouping, and missing values
  follow a gap, bridge, drop-observation or error policy. Dense, sparse and
  streaming inputs produce the same polylines; a sparse column is read through
  `value_at` and never densified, and a streaming window never blocks. The
  composed scene serializes through `to_svg()` and, with `render-native`,
  through `to_native_frame()`; any other configured kind fails with a typed
  error. Transition and animation wiring is deferred to the transition
  integration; the geometry is transition-ready. The example
  `examples/parallel_coordinates_plot.rs` writes
  `docs/figures/parallel_coordinates_plot.svg`.
- The matrix recipe `SVHexbinPlot` (`scienceview::sv_recipes::sv_matrix`):
  two continuous variables binned into a pointy-top hexagonal lattice and each
  occupied cell drawn as one filled path, coloured by its count. The chart is
  the composition of FDR-135 — `data -> [hexagonal lattice] -> (cells, counts)
  -> [SVScale continuous over a colour map] -> one path per hexagon` — and its
  configuration surface is `x`, `y`, `series`, `value`, `aggregation`,
  `grid_size`, `extent`, `min_count`, `orientation`, `colour_map`,
  `colour_scale`, `normalise`, `missing_value_policy`, the titles and the
  viewport, with `backend(...)` required and `execution_mode` defaulting to
  `Automatic`. Count aggregation accumulates the observations directly; mean
  aggregation is delegated to `SVStat::Aggregate` and never reimplemented. A
  sparse coordinate column is read through `value_at` and never densified; a
  streaming window renders without blocking. The composed geometry carries a
  colour bar and axes and serializes through `to_svg()` and, with
  `render-native`, through `to_native_frame()`; any other configured kind fails
  with a typed error instead of being ignored. Transition and animation wiring
  is deferred to the transition integration; the geometry is transition-ready.
  The example `examples/hexbin_plot.rs` writes `docs/figures/hexbin_plot.svg`.
- The matrix recipe `SVImagePlot` (`scienceview::sv_recipes::sv_matrix`):
  a raster drawn on its data extent, either a numeric field mapped through a
  continuous colour map (greyscale by default) or RGBA pixels drawn as stored.
  The chart is the composition of FDR-134 —
  `data -> [no stat] -> raster -> [SVScale::Continuous over a colour map] -> SVMark::Image` —
  and its configuration surface is `raster` (`SVImageSource`),
  `colour_map`, `interpolation` (nearest or bilinear), `minimum_intensity` and
  `maximum_intensity` (the FDR's `z_min`/`z_max`), `opacity`, `origin`, the
  raster dimensions, the data `extent`, `aspect_ratio` (equal or stretch), the
  `missing_policy`, `colour_bar`, the titles and the viewport, with
  `backend(...)` required and `execution_mode` defaulting to `Automatic`.
  An intensity raster accepts dense, sparse and streaming columns: a sparse
  raster is read from its stored entries and never densified, and a streaming
  column renders its newest window without blocking. Missing pixels are
  transparent or take the domain's lower colour. The composed scene serializes
  through `to_svg()` (a PNG data URI) and, with `render-native`, through
  `to_native_frame()`; any other configured kind fails with a typed error, and
  the terminal backend is rejected at build time because it cannot express a
  raster. Transition and animation wiring — including the pixel cross-fade the
  FDR names — is deferred to the transition integration; the geometry is
  transition-ready. The example `examples/image_plot.rs` computes its field
  analytically and writes the deterministic figure `docs/figures/image_plot.svg`.
- The matrix recipe `SVDensityHeatmapPlot` (`scienceview::sv_recipes::sv_matrix`):
  the two-dimensional density of two continuous variables drawn as a raster of
  coloured cells. The chart is the composition of FDR-133 —
  `data -> [SVStat::Bin (2D)] -> (grid, counts) -> [SVScale::Continuous over a colour map] -> SVMark::Image` —
  and its configuration surface is `x`, `y`, `series`, `weight`, `bin_count_x`,
  `bin_count_y`, `normalise`, `colour_map`, `colour_bar`, `opacity`,
  `empty_bin_policy`, the titles and the viewport, with `backend(...)` required
  and `execution_mode` defaulting to `Automatic`. Both edge vectors come from
  `SVStat::Bin` per axis and the per-cell fold from `SVStat::Aggregate` (`Sum`)
  over derived row-major cell codes; the recipe adds no kernel variant.
  `SVNormalisation` normalises the cells (`Count`, `Probability`, `Density`) and
  `SVEmptyBinPolicy` paints an empty bin with the map's lowest value or omits
  it. A sparse column is walked through its stored entries plus the implicit-zero
  run and is never densified; a streaming column is read from its current window
  and never blocks. The composed geometry serializes through `to_svg()` and,
  with `render-native`, through `to_native_frame()`; any other configured kind
  fails with a typed error. Transition and animation wiring is deferred to the
  transition integration; the geometry is transition-ready. The example
  `examples/density_heatmap_plot.rs` writes
  `docs/figures/density_heatmap_plot.svg`.

- The matrix recipe `SVContourPlot` (`scienceview::sv_recipes::sv_matrix`):
  contour lines and filled bands over a supplied two-dimensional scalar field.
  The chart is the composition of FDR-132 —
  `data -> [SVStat::Bin (grid) + SVStat::Aggregate (cell mean)] -> scalar field
  -> [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line |
  SVMark::Area` —
  and its configuration surface is `value`, `matrix(rows, columns)`, `x`, `y`,
  `x_range`, `y_range`, `grid_count`, `levels`, `level_range`, `level_size`,
  `style`, `colour_map`, `colour_scale`, `line_width`, `labels`,
  `missing_policy`, the titles and the viewport, with `backend(...)` required and
  `execution_mode` defaulting to `Automatic`. A matrix column is read directly
  onto a lattice and a point cloud is binned on both axes through `SVStat::Bin`
  and averaged per cell by `SVStat::Aggregate`; the marching-squares extraction
  lives in the recipe's `geometry` submodule
  because the closed stat vocabulary has no contour variant. Sparse input is
  never densified and a streaming input is read from its current window. The
  composed geometry serializes through `to_svg()` and, with `render-native`,
  through `to_native_frame()`; any other configured kind fails with a typed
  error instead of being ignored. Transition and animation wiring is deferred;
  the geometry is transition-ready. The example `examples/contour_plot.rs`
  writes `docs/figures/contour_plot.svg`.
- The matrix recipe `SVHeatmapPlot` (`scienceview::sv_recipes::sv_matrix`):
  a two-dimensional matrix drawn as a grid of cells whose colour encodes the
  value, with the colour bar as the legend of the continuous colour scale. The
  chart is the composition of FDR-131 —
  `data -> [SVStat::Aggregate (pivot into a matrix)] -> (rows, columns, values) -> [SVScale::Continuous over a colour map] -> SVMark::Image | SVMark::Rect` —
  and its configuration surface is `matrix` or `data_source` with `row`,
  `column` and `value`, `row_labels`, `column_labels`, `colour_map`, `centre`,
  `robust`, `annot`, `annot_format`, `line_width`, `square`, `colour_bar`,
  `missing`, `orientation`, the titles and the viewport, with `backend(...)`
  required and `execution_mode` defaulting to `Automatic`. Dense, sparse and
  streaming values are resolved once per operation; a sparse value column
  contributes only its stored cells and is never densified, and a streaming
  column contributes its current window. Duplicate row/column pairs are pivoted
  through `SVStat::Aggregate`. The composed geometry serializes through
  `to_svg()` and, with `render-native`, through `to_native_frame()`; any other
  configured kind fails with a typed error instead of being ignored. Transition
  and animation wiring is deferred to the transition integration; the geometry
  is transition-ready. The example `examples/heatmap_plot.rs` writes
  `docs/figures/heatmap_plot.svg`.
- The distributional recipe `SVPointPlot` (`scienceview::sv_recipes::sv_distribution`):
  the point estimate of every category with its uncertainty, optionally joined
  by a line. The chart is the composition of FDR-123 —
  `data -> [SVStat::Aggregate + interval] -> (estimate, lower, upper) -> [SVMove::Dodge] -> [SVScale] -> SVMark::Dot + ErrorBar + optional Line` —
  and its configuration surface is `category`, `value`, `series`, `estimator`,
  `error`, `confidence_interval`, `ordering`, `missing_value_policy`, `orient`,
  `marker`, `caps`, `cap_width`, `join`, `line_width`, `dash`, `colour`, the
  titles and the viewport, with `backend(...)` required and `execution_mode`
  defaulting to `Automatic`. The estimator and the interval come from
  `SVStat::Aggregate` and are never reimplemented; `SVMove::Dodge` places
  several series inside a category slot. A sparse column contributes only its
  stored entries and a streaming column its current window; both are resolved
  once per operation. The composed geometry serializes through `to_svg()` and,
  with `render-native`, through `to_native_frame()`; any other configured kind
  fails with a typed error instead of being ignored. Transition and animation
  wiring is deferred to the transition integration; the geometry is
  transition-ready. The example `examples/point_plot.rs` writes
  `docs/figures/point_plot.svg`.
- The distributional recipe `SVCountPlot` (`scienceview::sv_recipes::sv_distribution`):
  the number of observations in every category, drawn as a bar. The chart is
  the composition of FDR-122 —
  `data -> [SVStat::Count] -> (count per category) -> [SVMove::Stack | Dodge] -> [SVScale] -> SVMark::Bar` —
  and its configuration surface is `category`, `series`, `category_labels`,
  `series_labels`, `orient`, `width`, `normalise`, `order`, `colour`, `multiple`
  and `missing_value_policy`, with `backend(...)` required and `execution_mode`
  defaulting to `Automatic`. Counting is never reimplemented:
  `SVStat::Count` counts each series' category column and applies the requested
  `SVCountOrder`, while the recipe only aligns the returned category sets on one
  shared axis. `SVMove::Stack` and `SVMove::Dodge` arrange several series inside
  a category slot; `normalise` divides the counted values by the grand total.
  A sparse category column keeps its implicit zeros as category-zero
  observations and is never densified; a streaming window renders without
  blocking and `update(...)` recomposes the current window. The composed scene
  serializes through `to_svg()` and, with `render-native`, through
  `to_native_frame()`; any other configured kind fails with a typed error
  instead of being ignored. Weighted counting is rejected with a typed
  configuration error naming the `SVStat::Count` kernel gap. Transition and
  animation wiring is deferred to the transition integration; the geometry is
  transition-ready. The example `examples/count_plot.rs` writes
  `docs/figures/count_plot.svg`.
- The distributional recipe `SVBarPlot` (`scienceview::sv_recipes::sv_distribution`):
  the aggregate value of every category as a bar, optionally with an error
  interval. The composition is FDR-121's exactly — `SVStat::Aggregate` produces
  one value (and interval) per category and series, `SVMove::Dodge` or
  `SVMove::Stack` arranges the series, the scale kernel places them and
  `SVMark::Bar`, `SVMark::ErrorBar` and `SVMark::Text` draw them; no kernel
  variant is added. The builder exposes `category`, `value`, `series`,
  `weight`, `estimator`, `error`, `confidence_interval`, `arrangement`,
  `ordering`, `missing_value_policy`, `orient`, `width`, `colour`, `label`,
  `label_format`, the titles and the viewport, and requires an explicit
  `backend(...)`. A sparse column contributes only its stored entries and a
  streaming column its current window; both are resolved once per operation.
  The figure is generated by `examples/bar_plot.rs` through the public
  `to_svg()` serializer and mirrored into the book (chapter 32). The fill
  pattern stays deferred until the core gains an `SVFillPattern` type.
- The distributional recipe `SVSwarmPlot` (`scienceview::sv_recipes::sv_distribution`):
  one dot per observation of a continuous variable grouped by category, displaced
  along the categorical axis so that no two dots of a category overlap; the width
  of the swarm reads the density of the sample. The configuration surface follows
  FDR-120 — `value`, `category`, `series`, `marker`, `size`, `orient`,
  `swarm_width`, `alpha` and `missing_value_policy` — and the builder requires an
  explicit backend selection. The layout is never reimplemented: the recipe
  converts the solved marker diameter and the category slot into the data-space
  `marker_extent` and `width` that `SVMove::Swarm` takes, so the kernel's
  deterministic, collision-free, width-bounded layout and its typed errors — a
  width below two marker extents, or a cluster that cannot be placed inside the
  slot — reach the caller unchanged. Both orientations share one layout pass and
  only reassign the axes; the series column colours the dots and adds a legend.
  Dense, sparse and streaming columns resolve once per operation; a sparse input
  contributes exactly its stored entries (a million-logical column with three
  entries draws three dots) and a streaming window renders without blocking.
  Non-finite observations are dropped or rejected by the missing-value policy.
  The composed geometry serializes through `to_svg()` and, with `render-native`,
  through `to_native_frame()`; any other configured kind fails with a typed
  error instead of being ignored. Transition and animation wiring is deferred to
  the transition integration; the geometry is transition-ready. The example
  `examples/swarm_plot.rs` writes `docs/figures/swarm_plot.svg`.
- The distributional recipe `SVStripPlot` (`scienceview::sv_recipes::sv_distribution`):
  every observation of a variable drawn as one dot over its category, displaced
  along the categorical axis so overlapping points stay visible. The
  configuration surface follows FDR-119 — `value`, `category`, `series`,
  `jitter`, `jitter_mode`, `seed`, `marker`, `alpha`, `size`, `orient` and
  `missing_value_policy` — and the builder requires an explicit backend
  selection. The displacement is `SVMove::Jitter`, keyed by the observation's
  row and category, so a seeded jitter is reproducible and a point keeps its
  offset while its value changes. Dense, sparse and streaming columns resolve
  once per operation; a sparse column contributes only its stored entries and
  is never densified, and a streaming window renders without blocking. The dots
  are the closed `SVMark::Dot` shape vocabulary, coloured by the optional
  `series` column. The composed geometry serializes through `to_svg()` and,
  with `render-native`, through `to_native_frame()`; any other configured kind
  fails with a typed error. Transition and animation wiring is deferred to the
  transition integration; the geometry is transition-ready. The example
  `examples/strip_plot.rs` writes `docs/figures/strip_plot.svg`.
- The distributional recipe `SVBoxenPlot` (`scienceview::sv_recipes::sv_distribution`):
  letter-value plots, a nest of nested quantile boxes that summarises a large
  distribution with far fewer observations than a violin needs. The
  configuration surface follows FDR-118 — `value`, `category`, `series`,
  `depth`, `tail_rule`, `show_median`, `show_outliers`, `outlier_marker`,
  `width` and `orient` — and the builder requires an explicit backend
  selection. Every letter value comes from `SVStat::Percentile`; the recipe
  never reimplements the estimator. `SVBoxenTailRule` selects the
  tail-stopping rule (`Tukey`, Henderson's `floor(log2(n)) - 3`, is the
  default; `Proportion` and `Full` are also available) and `depth` overrides
  the derived nesting depth, which produces `2 * depth - 1` boxes. The central
  box spans the interquartile range and every outer box narrows by half, so the
  nest reads as a sequence of letter values. The boxes are `SVMark::Ribbon`
  primitives, the median is `SVMark::Line`, the optional outliers are
  `SVMark::Dot`, and `SVMove::Dodge` lays one nest per series inside a
  category. Both orientations are supported. Dense, sparse and streaming
  columns resolve once per operation; a sparse input contributes only its
  stored entries and a streaming window renders without blocking and reports
  `is_approximate`. The composed geometry serializes through `to_svg()` and,
  with `render-native`, through `to_native_frame()`; any other configured kind
  fails with a typed error instead of being ignored. Transition and animation
  wiring is deferred to the transition integration; the geometry is
  transition-ready. The example `examples/boxen_plot.rs` writes
  `docs/figures/boxen_plot.svg`.

- The distributional recipe `SVBoxPlot` (`scienceview::sv_recipes::sv_distribution`):
  the five-number summary of every category, drawn as a box with whiskers,
  median line and — optionally — the observations outside the whiskers. The
  configuration surface follows FDR-116 — `value`, `category`, `series`,
  `quartiles`, `whisker`, `whisker_range`, `show_outliers`, `outlier_marker`,
  `notch`, `show_mean`, `width` and `orient` — and the builder requires an
  explicit backend selection. The quartiles, median and percentile whiskers are
  never reimplemented: `SVStat::Percentile` supplies them with its documented
  linear interpolation between neighbouring ranks, while `SVWhiskerRule`
  (`Tukey`, `MinMax`, `Percentile`) only selects which observations become the
  whisker ends and which become outliers. `SVMove::Dodge` lays one box per
  series inside a category, and a category slot stays one unit wide even when
  the chart has a single category. The box body is composed from `SVMark::Ribbon`
  (three stacked segments when the notch is enabled), the median, whiskers and
  caps from `SVMark::Line`, and the outliers and mean from `SVMark::Dot`; no
  mark, stat, move or scale variant is added by the recipe. Both orientations
  are supported. Dense, sparse and streaming columns resolve once per
  operation; a sparse input contributes only its stored entries (a
  million-logical column with five entries summarises five observations) and a
  streaming window renders without blocking and reports `is_approximate`. The
  composed geometry serializes through `to_svg()` and, with `render-native`,
  through `to_native_frame()`; any other configured kind fails with a typed
  error instead of being ignored. Transition and animation wiring is deferred
  to the transition integration; the geometry is transition-ready. The example
  `examples/box_plot.rs` writes `docs/figures/box_plot.svg`.

- The distributional recipe `SVRugPlot` (`scienceview::sv_recipes::sv_distribution`):
  one small tick per observation, drawn in the margin of the axis the values
  occupy. The configuration surface follows FDR-115 — `value`, `series`,
  `orientation`, `height`, `alpha`, `line_width`, `title` and `value_label` —
  and the builder requires an explicit backend selection. Every tick is the
  two-point `SVMark::Line` segment geometry of the closed mark vocabulary; the
  recipe places each stored observation once through a continuous value scale
  and a bounded depth scale that spans the margin, so the ticks never enter the
  plot area — the tick length is capped at the margin depth even at
  `height = 1.0`. Dense, sparse and streaming input of the same observations
  agree byte for byte, a sparse column marks only its stored support without
  being densified and without letting implicit zeros widen the axis, and a
  large sample is grouped into one compound path per paint so the document
  stays bounded. Non-finite observations draw no tick, and a column with no
  drawable observation renders an empty scene. The new example
  `examples/rug_plot.rs` generates `docs/figures/rug_plot.svg`.
- The empirical cumulative distribution recipe `SVECdfPlot` in
  `scienceview::sv_recipes::sv_distribution::sv_ecdf_plot`: the proportion of
  observations at or below each value, composed exactly as
  `data -> [SVStat::EmpiricalCdf] -> (values, cumulative probability) ->
  [SVScale] -> SVMark::Line (step geometry)`. The curve is never reimplemented:
  the stat kernel derives the breakpoints and probabilities, and the recipe
  expands them into explicit horizontal-then-vertical vertices reaching the
  total at the maximum. The builder requires an explicit backend, defaults
  `execution_mode` to `Automatic`, and exposes `value`, optional `series`,
  `normalise`, `orient` (`SVBarOrientation`), `complementary`, `line_width`,
  `viewport` and `title`. `normalise(false)` reaches the observation count;
  `complementary(true)` draws the survival curve from the same kernel result;
  horizontal orientation swaps the two channels. Dense, sparse and streaming
  input resolve once per operation: an ungrouped sparse column reaches the
  kernel unchanged (never densified) and a streaming window renders without
  blocking. Non-finite observations and series codes are omitted. The
  composition serializes through `to_svg()` and, with `render-native`, through
  `to_native_frame()`; any other configured kind fails with a typed error.
  Transition and animation wiring is deferred. The example
  `examples/ecdf_plot.rs` writes `docs/figures/ecdf_plot.svg`.
- The distributional recipe `SVDensityContourPlot`
  (`scienceview::sv_recipes::sv_distribution`): the joint distribution of two
  continuous variables drawn as filled contour bands or contour lines. The
  configuration surface follows FDR-113 — `x`, `y`, `series`, `weight`,
  `levels`, `level_range`, `bandwidth`, `bandwidth_adjustment`, `grid_count`, `style`,
  `colour_map` and `colour_scale` — and the builder requires an explicit backend
  selection. The density is never reimplemented: `SVStat::Bin` derives the two
  edge vectors and the recipe folds the observation pairs into the joint grid,
  normalised so it integrates to one; sparse views are walked through their
  stored entries and streaming views through their current window. A
  marching-squares step turns the scalar field into backend-agnostic paths, and
  the filled style paints each threshold region as one closed compound path
  from the lowest level up, so the bands nest without gaps. SVG and native
  serialize the same composition; every other kind reports a typed
  unsupported-capability error. The example is `examples/density_contour_plot.rs`
  and the generated figure is `docs/figures/density_contour_plot.svg`.
- The distribution recipe `SVDensityPlot`
  (`scienceview::sv_recipes::sv_distribution::sv_density_plot`): a smoothed
  estimate of one continuous variable, composed exactly as
  `data -> [SVStat::KernelDensity] -> (grid, density) -> [no move] -> [SVScale]
  -> SVMark::Line | SVMark::Area`. The configuration surface follows FDR-112 —
  `value`, `series`, `bandwidth`, `bandwidth_adjustment`, `kernel`, `grid_count`, `cut`,
  `fill`, `normalise`, `line_width` — and the builder requires an explicit
  backend selection. The estimate is never reimplemented:
  `SVStat::KernelDensity` supplies the grid, the density and the Silverman
  bandwidth for the Gaussian, Epanechnikov and uniform kernels. The recipe
  scales the bandwidth linearly with `bandwidth_adjustment`, groups observations by
  `series` into one curve per group with a palette colour, drops non-finite
  observations by policy, and places `SVMark::Line` plus an optional
  `SVMark::Area` fill over one axes pair. Dense, sparse and streaming input
  agree within tolerance; a sparse value column is read through its stored
  support without materialising implicit zeros, and grouping a sparse column is
  rejected with the documented alternative rather than densified. The new
  example `examples/density_plot.rs` generates `docs/figures/density_plot.svg`.
- The distributional recipe `SVHistogramPlot` in
  `scienceview::sv_recipes::sv_distribution::sv_histogram_plot`: a continuous
  variable binned and drawn as bars, composed exactly as
  `data -> [SVStat::Bin] -> (edges, counts) -> [SVMove::Stack | Dodge] ->
  [SVScale x, y, colour] -> SVMark::Bar`. The builder requires an explicit
  backend, defaults `execution_mode` to `Automatic`, and exposes the FDR-111
  surface: `value`, `series`, `bin_count`, `bin_width`, `bin_edges`,
  `normalise`, `cumulative`, `common_normalisation`, `common_bins`, `shrink`,
  `element`, `orientation`, `multiple` and a missing-value policy. Binning is
  never reimplemented: `SVStat::Bin` supplies the edges and the count,
  probability or density heights, and `SVMove` supplies stacking and dodging.
  Dense, sparse and streaming input resolve once per operation; a sparse column
  keeps its stored support and its implicit zeros, and a streaming window
  renders without blocking. The composed geometry serializes through `to_svg()`
  and, with `render-native`, through `to_native_frame()`; any other configured
  kind fails with a typed error instead of being ignored. Weighted binning is
  not implemented because the shipped bin kernel does not consume weights, and
  a configured weight fails with a configuration error naming that gap. The
  example `examples/histogram_plot.rs` calls the public `to_svg()` and writes
  `docs/figures/histogram_plot.svg`.
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

- The distributional recipe `SVViolinPlot` (`scienceview::sv_recipes::sv_distribution`):
  the symmetric density silhouette of every category, revealing multimodality,
  skew and bimodality that a box plot hides. The configuration surface follows
  FDR-117 — `value`, `category`, `series`, `bandwidth`, `bandwidth_adjustment`,
  `width`, `scale` (`Area`, `Count`, `Width`), `show_box`, `show_points`,
  `split` and `orient` — and the builder requires an explicit backend
  selection. The estimate comes from `SVStat::KernelDensity` with the derived
  bandwidth and its adjustment; `SVStat::Percentile` supplies the optional
  inner box. `SVMove::Dodge` lays one violin per series inside a category, or
  `split(true)` gives two series the two halves of one slot; `SVMove::Jitter`
  places the optional sample points deterministically. The silhouette is
  composed from `SVMark::Area` (one closed path per mirrored half), the inner
  box and median from `SVMark::Line`, and the sample points from `SVMark::Dot`;
  no mark, stat, move or scale variant is added by the recipe. Both
  orientations are supported. Dense, sparse and streaming columns resolve once
  per operation; a sparse input contributes only its stored entries and a
  streaming window renders without blocking and reports `is_approximate`. The
  composed geometry serializes through `to_svg()` and, with `render-native`,
  through `to_native_frame()`; any other configured kind fails with a typed
  error instead of being ignored. Transition and animation wiring is deferred
  to the transition integration; the geometry is transition-ready. The example
  `examples/violin_plot.rs` writes `docs/figures/violin_plot.svg`.

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
