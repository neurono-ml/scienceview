# Design

## Context

`SVDensityPlot` is one of the scientific-core recipes specified in
`docs/FDRs/distribution-density-plot.md`. This change implements exactly that record on top of
the core kernels already landed by the earlier waves, and touches nothing else.


## Decisions

### D1 — Composition

The chart is exactly the composition
`data -> [SVStat::KernelDensity] -> (grid, density) -> [no move] -> [SVScale] -> SVMark::Line | SVMark::Area`. It introduces no new mark, stat, move or scale variant; if
implementation reveals a missing variant, that is a change to the core
capability first (patterns.md P5).

### D2 — Data contract

The chart consumes `value`, `series`, `weight`. Dense,
sparse and streaming representations are accepted; the recipe dispatches on the
representation once per operation and never densifies or blocks (patterns.md
P6).

### D3 — Temporal behaviour

density curve: interpolates point by point over the fixed grid. fill: interpolates with the curve. bandwidth change: the curve morphs into the new estimate.

### D4 — Registration

The chart is registered by appending one
line to `src/recipes/distribution/mod.rs`. No central registry exists, because a
central registry would serialise every recipe change against every other one
(patterns.md P4).

## Open Questions

- Should the bandwidth be chosen automatically by a better rule (for example, a plug-in selector), and if so is that a different kernel? *Open.*
- Should the density kernel support a weighted variant natively, or through the `weight` column? *Current answer: through the column — the kernel takes weights rather than growing a second variant.*


## Implementation amendments

### A1 — Weight is not a recipe option yet

The FDR lists a `weight` column, but the kernel-density kernel applies an
unweighted estimate only. Weighting would be a new core kernel variant with its
own spec delta (patterns.md P5), so the recipe ships without the column and the
FDR records the amendment. Series grouping covers the multi-curve case.

### A2 — Grouped sparse columns are rejected

Grouping a sparse value column by a dense series column would require walking
the value column's whole logical lattice to attribute every implicit zero,
which is a densification. The recipe rejects that combination with
`UnsupportedRepresentation` and names the alternative (dense or streaming value
column, or an ungrouped estimate). An ungrouped sparse column is passed to the
kernel as-is.

### A3 — Normalisation semantics

`Density` keeps the kernel's probability density, `Probability` multiplies by
the grid spacing so the samples sum to one, and `Count` additionally multiplies
by the observation count so the samples sum to the group size.

### A4 — `bw_adjust` composes with the derived bandwidth

When the bandwidth is not fixed, the recipe runs the kernel once to obtain the
Silverman bandwidth and re-applies it fixed and scaled when `bw_adjust != 1`.
It never reimplements the rule; the second pass is a kernel call.
