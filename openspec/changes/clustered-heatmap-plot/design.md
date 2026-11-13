# Design

## Context

`SVClusteredHeatmapPlot` is a scientific-core recipe. The earlier plan assumed
`sciencekit` would provide hierarchical clustering, but the published
`sciencekit_cluster` offers only `KMeans`, `MiniBatchKMeans` and `DBSCAN`; none
of them yields a leaf order and a merge tree. Adding a linkage kernel here
would violate the DAG and the closed-kernel rule (patterns P5).

## Decisions

### D1 — The clustering is an input

The recipe consumes the matrix plus `row_order`, `column_order`,
`row_linkage` and `column_linkage`. It never computes a distance, a metric or
a linkage. This keeps `scienceview` a renderer and lets a future `sciencekit`
kernel plug in at the call site without changing the chart.

### D2 — Composition

The chart is the composition
`data -> [SVStat::Aggregate (matrix)] -> matrix + supplied clustering ->
[validate] -> [SVScale::Continuous over a colour map] -> SVMark::Image (cells)
+ supplied dendrogram paths`. It introduces no new mark, stat or scale
variant. The dendrogram segments are `SVMark` path primitives built from the
supplied merge records; the cells reuse the heatmap matrix and colour-scale
kernels.

### D3 — Validation is part of the contract

A supplied tree is validated before composition, per axis: leaves appear
exactly once, child indices are in range and earlier than the merge using
them, distances are finite, non-negative and no shorter than a child, leaf
counts are consistent, and the order is a permutation of the axis dimension.
Failures are `SVError::InvalidClusterTree { axis, position, message }`; an
order of the wrong length is `SVError::MismatchedLengths`. A requested
dendrogram without both trees is an explicit configuration error.

### D4 — Data boundary

Dense and sparse are accepted. The pivot reuses `SVStat::Aggregate`; a sparse
value column contributes only its stored entries, so it is never densified.
Streaming is a static-only exception: the supplied order and tree describe one
whole snapshot, so `build` rejects a stream with
`UnsupportedRepresentation` and the alternative "take a snapshot of the current
window, compute the clustering outside scienceview, then render that snapshot".
There is deliberately no dense/sparse/streaming equivalence requirement.

### D5 — Registration

The chart is registered by appending one `#[path]` module and one `pub use`
block to `src/recipes/matrix/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns P4).

### D6 — Transitions are deferred

The scene carries transition-ready geometry: cell identity survives the
reorder and the paths are recomputed from the supplied tree. Wiring the
transition and animation layers is a later integration, so this change does not
claim an animation acceptance item.

## Open Questions

None. The original clustering-implementation question is resolved by removing
the computation from the recipe.
