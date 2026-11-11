# Feature Description Records (FDR)

Feature Description Records are the development documentation of `scienceview`.
Each FDR describes **one feature of the library** in enough detail that an
implementer can build it, a reviewer can check it, and a user can look it up
without reading the source.

FDRs live on `main` under `docs/FDRs/`. They are *development* documentation:
the user-facing manual is the mdBook (branch `docs/documentations`, chapters
under `docs/src/`). Every FDR points at the public chapter that materializes
it, and every public chapter points back at the FDR it implements.

## How to read an FDR

Every FDR follows the same schema:

| Section | Content |
|---|---|
| Header | id, name, family, status, the core FDRs it builds on |
| Purpose | what the feature is for, in one paragraph |
| Sources | the Python libraries whose behavior this feature reproduces, with links |
| Recipe | the literal composition of core kernels that produces the feature |
| Data contract | columns, match keys, accepted representations |
| Visual channels | how each visual channel is mapped, and what it may encode |
| Configuration | every knob, its type, default and range |
| Theme | the design tokens the feature consumes |
| Temporal | transitions, animation, backend requirements |
| Backend notes | how each rendering backend realizes the feature |
| Acceptance | the criteria the feature must satisfy before it ships |
| Open questions | unresolved decisions, with the option set |

`status` is one of `planned`, `in-progress`, `shipped`, `deferred`.

## The two layers

`scienceview` models a chart in two layers, and every FDR belongs to exactly
one of them.

```
Layer 1 — CORE (docs/FDRs/core/)
   The kernels every chart is built from. Each core FDR describes a small,
   composable, backend-independent unit of computation or rendering.

     core-mark        SVMark   dot, line, area, bar, ribbon, text, image, wedge, error bar
     core-stat        SVStat   bin, count, aggregate, kde, ecdf, percentile, fit
     core-move        SVMove   stack, dodge, jitter, shift, norm
     core-scale       SVScale  continuous, nominal, temporal, boolean
     core-transition  SVTransition  enter / update / exit, matched by key
     core-animation   SVAnimation   keyframes or stream, interpolation, playback

Layer 2 — RECIPES (docs/FDRs/recipes/)
   The charts a user actually asks for. A recipe is a literal composition of
   core kernels, plus the configuration surface and the acceptance criteria
   that make it a distinct, reviewable feature.
```

The split is deliberate. It is the reason a chart can satisfy the
representation-parity rule of `data-view-boundary` in one place — the
statistical kernels dispatch on `SVColumnView` once per operation — instead of
once per chart type. It is also why the same kernel serves many charts:
`SVStat::bin` backs the histogram, the count plot and the hexbin.

## Core index

| FDR | Kernel | Responsibility |
|---|---|---|
| [core-mark](core/core-mark.md) | `SVMark` | Geometric primitive that turns values into marks |
| [core-stat](core/core-stat.md) | `SVStat` | Statistical transform applied before drawing |
| [core-move](core/core-move.md) | `SVMove` | Positional displacement of overlapping marks |
| [core-scale](core/core-scale.md) | `SVScale` | Mapping of data values onto visual channels |
| [core-transition](core/core-transition.md) | `SVTransition` | Enter / update / exit of marks across states |
| [core-animation](core/core-animation.md) | `SVAnimation` | Keyframes, interpolation and playback |

## Recipe index

### Relational — continuous cartesian

| FDR | Type | Sources |
|---|---|---|
| [relational-scatter-plot](recipes/relational-scatter-plot.md) | `SVScatterPlot` | seaborn `scatterplot`, bokeh `Circle`/`Scatter`, plotly `Scatter` |
| [relational-line-plot](recipes/relational-line-plot.md) | `SVLinePlot` | seaborn `lineplot`, bokeh `Line`/`MultiLine`, plotly `Scatter` (lines) |
| [relational-area-plot](recipes/relational-area-plot.md) | `SVAreaPlot` | seaborn `Area`, bokeh `VArea`/`HArea`, plotly `Scatter` (fill) |
| [relational-step-plot](recipes/relational-step-plot.md) | `SVStepPlot` | bokeh `Step`, plotly `Scatter` (`line_shape`) |
| [relational-band-plot](recipes/relational-band-plot.md) | `SVBandPlot` | seaborn `Band`, plotly `Scatter` (fill between) |
| [relational-regression-plot](recipes/relational-regression-plot.md) | `SVRegressionPlot` | seaborn `regplot`/`lmplot`, plotly `Scatter` (trendline) |

### Distributional

| FDR | Type | Sources |
|---|---|---|
| [distribution-histogram-plot](recipes/distribution-histogram-plot.md) | `SVHistogramPlot` | seaborn `histplot`, bokeh `Quad`, plotly `Histogram` |
| [distribution-density-plot](recipes/distribution-density-plot.md) | `SVDensityPlot` | seaborn `kdeplot`, plotly `Densitymap` |
| [distribution-density-contour-plot](recipes/distribution-density-contour-plot.md) | `SVDensityContourPlot` | seaborn `kdeplot` (bivariate), plotly `Contour` |
| [distribution-ecdf-plot](recipes/distribution-ecdf-plot.md) | `SVECDFPlot` | seaborn `ecdfplot`, plotly `Ecdf` |
| [distribution-rug-plot](recipes/distribution-rug-plot.md) | `SVRugPlot` | seaborn `rugplot`, bokeh `Segment` |
| [distribution-box-plot](recipes/distribution-box-plot.md) | `SVBoxPlot` | seaborn `boxplot`, bokeh `Box`, plotly `Box` |
| [distribution-violin-plot](recipes/distribution-violin-plot.md) | `SVViolinPlot` | seaborn `violinplot`, plotly `Violin` |
| [distribution-boxen-plot](recipes/distribution-boxen-plot.md) | `SVBoxenPlot` | seaborn `boxenplot` |
| [distribution-strip-plot](recipes/distribution-strip-plot.md) | `SVStripPlot` | seaborn `stripplot`, plotly `strip` |
| [distribution-swarm-plot](recipes/distribution-swarm-plot.md) | `SVSwarmPlot` | seaborn `swarmplot` |
| [distribution-bar-plot](recipes/distribution-bar-plot.md) | `SVBarPlot` | seaborn `barplot`, bokeh `VBar`/`HBar`, plotly `Bar` |
| [distribution-count-plot](recipes/distribution-count-plot.md) | `SVCountPlot` | seaborn `countplot`, bokeh `VBar`, plotly `Bar` |
| [distribution-point-plot](recipes/distribution-point-plot.md) | `SVPointPlot` | seaborn `pointplot`, plotly `Scatter` (error bars) |

### Matrix and field

| FDR | Type | Sources |
|---|---|---|
| [matrix-heatmap-plot](recipes/matrix-heatmap-plot.md) | `SVHeatmapPlot` | seaborn `heatmap`, bokeh `ImageRGBA`, plotly `Heatmap` |
| [matrix-contour-plot](recipes/matrix-contour-plot.md) | `SVContourPlot` | plotly `Contour` |
| [matrix-density-heatmap-plot](recipes/matrix-density-heatmap-plot.md) | `SVDensityHeatmapPlot` | plotly `Densitymap` (2D) |
| [matrix-image-plot](recipes/matrix-image-plot.md) | `SVImagePlot` | plotly `Image`, bokeh `Image`/`ImageRGBA` |
| [matrix-hexbin-plot](recipes/matrix-hexbin-plot.md) | `SVHexbinPlot` | bokeh `HexTile` |
| [matrix-clustered-heatmap-plot](recipes/matrix-clustered-heatmap-plot.md) | `SVClusteredHeatmapPlot` | seaborn `clustermap` |

### Multivariate

| FDR | Type | Sources |
|---|---|---|
| [multivariate-parallel-coordinates-plot](recipes/multivariate-parallel-coordinates-plot.md) | `SVParallelCoordinatesPlot` | plotly `Parcoords` |
| [multivariate-parallel-categories-plot](recipes/multivariate-parallel-categories-plot.md) | `SVParallelCategoriesPlot` | plotly `Parcats` |
| [multivariate-splom-plot](recipes/multivariate-splom-plot.md) | `SVSplomPlot` | plotly `Splom` |
| [multivariate-pair-plot](recipes/multivariate-pair-plot.md) | `SVPairPlot` | seaborn `pairplot`/`PairGrid` |
| [multivariate-joint-plot](recipes/multivariate-joint-plot.md) | `SVJointPlot` | seaborn `jointplot`/`JointGrid` |

### Deferred families

Chart types outside the scientific core are catalogued, with their sources and
the reason for the deferral, in [deferred.md](deferred.md).

## Coverage policy

The library covers the **scientific core**: the chart families a scientist
reaches for when exploring data — relational, distributional, matrix/field and
multivariate. Every core recipe has a full FDR. Everything else (hierarchical
partitions, flow diagrams, 3D surfaces, polar and geographic projections,
financial and dashboard-specialized types) is catalogued in `deferred.md`
rather than specified, so that the catalogue of the three reference libraries
is complete and traceable without committing the implementation to a scope
that has not been validated.

Promotion from `deferred` to a full FDR is a change in its own right, and
requires the same evidence as any other feature: a proposed use case, a
kernel composition that does not force a change to the core, and an
acceptance criteria set.

## Relationship with the manual

```
docs/FDRs/            (main)         development documentation
docs/src/             (docs/documentations)  user-facing manual
```

An FDR names the chapter and the example program that materialize it. A
chapter names the FDR it implements. A recipe is not shipped until both sides
exist: the code, the example under `examples/`, the tool-generated figure
under `docs/src/assets/`, the chapter, and the FDR.

## Conventions used in these records

- Public Rust items are prefixed: `SV` + PascalCase for types and traits
  (`SVScatterPlot`), `sv_` + snake_case for free functions and modules. Methods
  inside `impl` blocks carry no prefix.
- Names are spelled out; the only abbreviation permitted is the project
  prefix `sv`/`SV`.
- Every figure and artist builder exposes `backend(SVBackendKind)` with no
  automatic variant, and `execution_mode(SVExecutionMode)`.
- Data is always borrowed: `ArrayView`, sparse views, borrowed streaming
  chunks. Nothing is copied to satisfy the public API.
- Dense, sparse and streaming inputs are equivalent wherever the mathematics
  allows it; a static-only exception must state why and name the alternative
  path.
