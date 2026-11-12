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


## Review follow-ups

- [x] Keep the aggregate uncertainty type as `SVAggregateErrorKind` instead of the FDR's `SVErrorKind`, so the name cannot be confused with the public `SVError` enumeration; the option remains `SVAggregateOptions::error`.
- [x] Expose the user-supplied fit model as the public `SVFitEstimator` trait with `sv_fit_with_estimator`, keeping the closed `SVStat` enumeration unchanged.
