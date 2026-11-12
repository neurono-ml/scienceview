# Spec Delta

## Purpose

Statistical transforms turn raw observations into the derived values a chart
draws — bins, counts, estimates, densities, cumulative probabilities,
percentiles and fitted curves — with the same result for dense, sparse and
streaming input.

## ADDED Requirements

### Requirement: The stat set is closed and complete

The system SHALL provide binning, counting, aggregation, kernel density, empirical CDF, percentiles and fitting, and recipes SHALL compose exactly one of them.

#### Scenario: a histogram composes the bin stat
- **WHEN** a histogram is built
- **THEN** the bin stat produces edges and counts

#### Scenario: a regression plot composes the fit stat
- **WHEN** a regression plot is built
- **THEN** the fit stat produces coefficients and a curve

### Requirement: Statistics respect the representation

Every stat SHALL resolve each input column's representation exactly once per operation, SHALL accept dense, sparse and streaming views, SHALL never densify a sparse view and SHALL never block on a stream.

#### Scenario: a sparse view is binned without expansion
- **WHEN** a sparse view is binned
- **THEN** the bins are computed from indices and values without expansion

#### Scenario: a sparse aggregate is folded in stored time
- **WHEN** an ungrouped sparse column of a very large logical length is aggregated
- **THEN** the stored entries and the implicit-zero multiplicity are folded without iterating the logical length

#### Scenario: a streaming view is consumed as its current window
- **WHEN** a streaming view is binned
- **THEN** the bins cover the current window and the call returns promptly

### Requirement: Window semantics are documented and enforced

A stat whose mathematics needs the whole dataset SHALL operate on the window and SHALL report that the result is windowed. Every input column SHALL contribute to that flag, including the independent, group and weight columns and the custom-estimator path.

#### Scenario: a streaming density is marked windowed
- **WHEN** a density is estimated over a stream
- **THEN** the result is marked as windowed

#### Scenario: a mixed input is windowed
- **WHEN** a fit's value column is dense and its independent column is streaming
- **THEN** the fitted curve is marked as windowed

#### Scenario: a dense view is exact
- **WHEN** a density is estimated over a dense view
- **THEN** the result is exact over that view

### Requirement: Statistical options are typed and validated

Every stat SHALL take a typed option builder, and an invalid configuration SHALL fail with an error before an observation is used or a derived value is produced.

#### Scenario: an invalid bin width is rejected
- **WHEN** a bin width of zero is requested
- **THEN** the operation fails with a typed configuration error

#### Scenario: a quantile outside the unit interval is rejected
- **WHEN** a percentile quantile of `1.5` is requested
- **THEN** the operation fails with a typed configuration error

### Requirement: Statistics are deterministic

A statistic whose definition is order-independent SHALL produce the same result independent of the order of the observations, within floating-point rounding.

#### Scenario: an aggregate is order-independent
- **WHEN** the same values are aggregated in two different orders
- **THEN** both results are equal

### Requirement: Numerical stability is enforced

A fit SHALL condition its independent column — centring and scaling it before solving — so that a large zero offset does not flatten the recovered curve, and it SHALL expand the conditioned solution back into the caller's polynomial coefficients.

#### Scenario: a linear fit at a billion-scale offset recovers the line
- **WHEN** a line is fitted with independent values around `1.7e9`
- **THEN** the predictions follow the generating line within tolerance

#### Scenario: a quadratic fit at a million-scale offset recovers the curve
- **WHEN** a degree-2 polynomial is fitted with independent values around `1e6`
- **THEN** the predictions follow the generating curve within tolerance

### Requirement: Non-finite observations are rejected with their position

Each stat SHALL reject a non-finite observation, weight, group code or independent value with an error naming the offending logical position, on every kernel and in the custom-estimator path.

#### Scenario: a NaN observation reports its position
- **WHEN** a value column holds a `NaN` at logical position 3
- **THEN** the operation fails with a non-finite error carrying position 3

### Requirement: Static-only exceptions are explicit and documented

A kernel that genuinely cannot accept a representation SHALL reject it up front with an `UnsupportedRepresentation` error naming an alternative path, and the change SHALL state why and name that path. The only such exception in this capability is the paired fit over sparse storage.

#### Scenario: a sparse paired fit names the alternative path
- **WHEN** a fit is applied to sparse columns
- **THEN** it fails with `UnsupportedRepresentation` naming dense or streaming binding, or pre-aggregation, as the alternative

### Requirement: Heavy statistics run on a caller-controlled compute path

Kernel density and fitting SHALL expose a non-blocking compute path and SHALL report whether a variant is heavy; they SHALL NOT start a runtime of their own and SHALL NOT block the thread that applies them.

#### Scenario: a heavy stat runs on a scoped compute thread
- **WHEN** a density or a fit is spawned on a caller-provided thread scope
- **THEN** the scoped result equals the result of applying the stat directly

### Requirement: Every stat application is observable

Each application SHALL record one span with the stat kind, the value column's representation, the logical point count, the duration and the outcome.

#### Scenario: a stat application records its span fields
- **WHEN** a bin stat is applied to a sparse view
- **THEN** one span records `stat.kind`, `representation`, `points`, `duration` and `outcome`
