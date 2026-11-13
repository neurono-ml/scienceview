# Design

## Context

`SVStatInput` already carries an optional `weights` column, and `SVStat::Aggregate`
already reads it. The two distribution kernels ignored it: `Bin` accumulated a
literal `1.0` per observation and `KernelDensity` divided by the observation
count. The recipes therefore had to either reject the column or omit it.

The repository's representation contract (spec `data-view-boundary`) rules out
the easy implementation — expanding a sparse column into a dense weight buffer —
so the weighted path must stay `O(stored)` for sparse values and sparse weights.

## Decisions

### D1 — One resolved weight column, one shared read path

A new `stat/weights.rs` resolves the weight column exactly once
(`SVObservationValues::from_view`), validates it once, and exposes:

- `sv_validate_weights` — length, finite and non-negative validation;
- `sv_total_weight` — the normalising total, `length` when unweighted;
- `sv_for_each_weighted_observation` — the fold shared by `Bin` and
  `KernelDensity`.

Both kernels fold through the same visitor, so an unweighted call and an
all-ones weighted call take the same arithmetic path.

### D2 — Implicit zeros are one run, weighted by their own weights

For a contiguous weight column the implicit zeros of a sparse value column are
visited once per maximal run, weighted by the run's weight sum. For a sparse
weight column the fold iterates the weight's stored entries and reads the value
at those positions, treating an implicit value zero as `0.0`. A position
without a stored weight carries zero weight, so neither column is materialised.

### D3 — Typed weight errors

`SVError::NegativeWeight { operation, position }` is added next to
`NonFiniteValue`, and the shared validator reports the first offending logical
position. Aggregates adopt it too, replacing their message-only configuration
error. No public name is abbreviated.

### D4 — Normalisation

`Bin` divides weighted counts by the total weight (probability) or by the total
weight times the bin width (density); cumulative sums stay post-normalisation.
`KernelDensity` divides by `total_weight * bandwidth`, which keeps the estimate
a probability density that integrates to one. `Count` normalisation in
`density_plot` scales by the total weight instead of the observation count.

### D5 — Recipe alignment

The histogram already grouped observations per series and dropped non-finite
values. Its weight column is filtered by the same decisions: the recipe tracks
the dropped logical positions and compacts or selects the weights in step, so
each group's weights match its values one-to-one while a sparse weight column
stays sparse. The density recipe does the same; for a sparse value column a
dropped stored entry becomes an implicit zero rather than shortening the
column, so its weight is zeroed in place.

### D6 — Bandwidth stays unweighted-derived

Silverman's rule of thumb still derives the bandwidth from the unweighted
moments. A weighted rule of thumb is a separate statistical decision; the
weight only enters the density sum and its normalisation. This is recorded in
the FDR so the choice is explicit.

## Open Questions

- Should the derived bandwidth use weighted moments? *Open — a follow-up; the
  weighted estimate currently only reweights the kernel sum.*
