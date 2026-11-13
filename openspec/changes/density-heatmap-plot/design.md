# Design

## Context

`SVDensityHeatmapPlot` is one of the scientific-core recipes specified in
`docs/FDRs/matrix-density-heatmap-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::Bin (2D)] -> (grid, counts) -> [SVScale::Continuous over a colour map] -> SVMark::Image`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `x`, `y`, `series`, `weight`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

cell colour: interpolates. cell entering or leaving: enters or exits. grid growing: the new cells enter.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/matrix/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

### D5 — Default bin counts

`bin_count_x` and `bin_count_y` default to one bin per eight pixels of the
plot area, so the documented defaults resolve to 60 horizontal and 38 vertical
bins. An explicit count always wins; a count below one is rejected at build.

### D6 — Cell normalisation and empty bins

The accumulated cell weight is normalised with `SVNormalisation`: `Count`
keeps the raw weight, `Probability` divides by the series total and `Density`
divides by the series total and the cell area. A cell that received no weight
is painted with the colour map's lowest value (`SVEmptyBinPolicy::LowestValue`,
the default) or emits no primitive (`Transparent`), which satisfies the FDR's
"lowest value, or transparent when configured" acceptance item.

### D7 — Smoothing

The FDR specifies no smoothing option, so none is exposed. Edges come from the
bin kernel and the per-cell fold goes through the aggregate kernel; the recipe
reimplements neither.

## Open Questions

- Should this recipe be a specialisation of `SVHeatmapPlot` — that is, the heatmap recipe with a binning stat — rather than a separate type? *Open: a shared implementation with a different default stat is tempting, but the two have different axis semantics.*

