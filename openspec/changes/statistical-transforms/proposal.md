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
- Condition the fit design so large zero offsets do not flatten the recovered
  curve, and reject non-finite observations with their logical position.
- Record one span per stat application (P8).

## Capabilities

### New Capabilities
- `statistical-transforms`: statistical transforms.

### Modified Capabilities

None.

## Deviations from the FDR

- **The compute path is caller-controlled, not implicitly pooled.** The FDR
  says density and fitting run "on the compute pool, never on the render
  thread". This change narrows that to what the core can honestly promise: a
  heavy stat exposes `SVStat::is_heavy()` and `SVStat::spawn_on(scope, input)`,
  and the caller — the figure or application that owns the frame loop — decides
  to offload it. The core starts no runtime and never blocks the thread that
  applies the stat. Rationale: the core is dependency-light by contract, and a
  scheduler owned by the render loop is the only place that knows whether
  offloading pays for the frame in flight. The corresponding requirement is
  `Heavy statistics run on a caller-controlled compute path`.
- **The paired fit is a static-only exception for sparse storage.** Two
  independently sparse columns place their implicit zeros at different
  positions, so a regression through that origin mass answers a different
  question than the one asked. The fit rejects sparse storage up front with
  `SVError::UnsupportedRepresentation`. The alternative path named in the error
  is to bind dense or streaming columns, or to pre-aggregate the sparse view
  before fitting; the streaming path still reports `windowed`.

## Impact

- New paths: `crates/scienceview_core/src/stat/`.
- Depends on the data boundary and on scales; consumed by most recipes.
- The parity harness is adopted by every kernel.
