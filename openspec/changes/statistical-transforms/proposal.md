# Proposal

## Why

Distributional and summary charts are not raw plots: they bin, count, aggregate,
estimate densities, compute cumulative distributions, take percentiles and fit
curves. That is the `SVStat` kernel of `docs/FDRs/core/core-stat.md`, and it is
where representation parity is actually enforced. Without it, nine of the
thirteen distributional recipes and three relational ones cannot exist.

## What Changes

- Introduce the closed `SVStat` enumeration: bin, count, aggregate, kernel density, empirical CDF, percentile and fit.
- Enforce the data-view boundary: dispatch once per operation, never densify, never block.
- Implement the window semantics for the kernels that cannot be unbounded.
- Provide the derived-view type that later layers consume.

## Capabilities

### New Capabilities
- `statistical-transforms`: statistical transforms.

### Modified Capabilities

None.

## Impact

- New paths: `crates/scienceview_core/src/stat/`.
- Depends on the data boundary and on scales; consumed by most recipes.
- The parity harness is adopted by every kernel.
