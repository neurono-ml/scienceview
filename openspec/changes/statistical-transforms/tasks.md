# Tasks

## Binning and counting

- [x] Write failing tests for bin edges, widths, normalisation and cumulative modes.
- [x] Implement bin and count, using the parity harness.

## Aggregation and percentiles

- [x] Write failing tests for each aggregate method and the error kinds.
- [x] Implement aggregate and percentile.

## Density and CDF

- [x] Write failing tests for bandwidth rules, grid size and window semantics.
- [x] Implement kernel density and empirical CDF off the render thread.

## Fit

- [x] Write failing tests for each fit model and the confidence interval.
- [x] Implement the fit kernel and the estimator trait.


## Review follow-ups (round 1)

- [x] Keep the aggregate uncertainty type as `SVAggregateErrorKind` instead of the FDR's `SVErrorKind`, so the name cannot be confused with the public `SVError` enumeration; the option remains `SVAggregateOptions::error`.
- [x] Expose the user-supplied fit model as the public `SVFitEstimator` trait with `sv_fit_with_estimator`, keeping the closed `SVStat` enumeration unchanged.

## Review follow-ups (round 2)

- [x] Condition the fit design before solving and add large-offset regression tests at `x ≈ 1e6` and `x ≈ 1.7e9`, including a degree-2 case.
- [x] Resolve each aggregate, fit and estimator column's representation once, and fold the sparse aggregate in `O(stored)`.
- [x] Consider every input column in `is_windowed`, and carry the flag on the custom-estimator curve.
- [x] Add position-bearing non-finite tests for every kernel path.
- [x] Record one span per `SVStat::apply` and assert its fields.
- [x] Narrow the heavy-kernel requirement to the caller-controlled compute path and record the deviation in the proposal and design.
- [x] Keep the FDRs honest: `SVAggregateErrorKind`, `SVCountOrder`, the bin defaults, the static-only fit exception and the acceptance criteria.

## Correction note

An earlier PR description and the `docs(stat)` commit stated a test count and a
"shipped" status that did not match the branch at the time of review. History is
not rewritten; this note records the correction. The authoritative count is the
one produced by `cargo test -p scienceview_core --lib sv_stat` on the final
commit, and the FDR status is `in-progress` until the change merges.
