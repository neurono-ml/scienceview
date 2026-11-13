# Spec Delta

## Purpose

Statistical transforms turn raw observations into the derived values a chart
draws — bins, counts, estimates, densities, cumulative probabilities,
percentiles and fitted curves — with the same result for dense, sparse and
streaming input.

## ADDED Requirements

### Requirement: An observation weight reweights binning and kernel density

`SVStat::Bin` and `SVStat::KernelDensity` SHALL accept an optional observation
weight column through `SVStatInput::with_weights`, SHALL resolve it once per
operation, and SHALL produce the weighted transform of the value column.

#### Scenario: weighted bin counts
- **WHEN** a bin stat is applied with weights `[1, 3, 2]` to observations in
  three bins
- **THEN** the counts are `[1, 3, 2]`

#### Scenario: all-ones weights reproduce the unweighted result
- **WHEN** a bin or density stat is applied with an all-ones weight column
- **THEN** the result equals the unweighted result exactly

#### Scenario: weighted probability normalisation
- **WHEN** a probability-normalised bin stat is applied with a total weight of
  four
- **THEN** each count is divided by four, not by the observation count

#### Scenario: a weighted density integrates to one
- **WHEN** a kernel density is applied with positive weights over a fine grid
- **THEN** the trapezoidal integral of the estimate is one within tolerance

#### Scenario: sparse implicit zeros carry their own weights
- **WHEN** a sparse value column is weighted by a dense or sparse weight column
- **THEN** every implicit zero contributes the weight stored at its logical
  position, and neither column is expanded

### Requirement: A weight column is validated once with its position

An invalid weight column SHALL be rejected before any observation is used, with
an error naming the operation and the logical position of the first offending
weight.

#### Scenario: a negative weight is rejected
- **WHEN** a weight column holds `-3` at logical position 1
- **THEN** the operation fails with `SVError::NegativeWeight { operation,
  position: 1 }`

#### Scenario: a non-finite weight is rejected
- **WHEN** a weight column holds `NaN` or an infinity at logical position 1
- **THEN** the operation fails with a non-finite error carrying position 1

#### Scenario: mismatched lengths are rejected
- **WHEN** the weight column is shorter than the value column
- **THEN** the operation fails with a mismatched-lengths error naming both
  lengths, before reading a weight

## MODIFIED Requirements

### Requirement: Statistics respect the representation

Every stat SHALL resolve each input column's representation exactly once per
operation, SHALL accept dense, sparse and streaming views for the value, group
and weight columns, SHALL never densify a sparse view and SHALL never block on
a stream.

#### Scenario: a sparse view is binned without expansion
- **WHEN** a sparse view is binned
- **THEN** the bins are computed from indices and values without expansion

#### Scenario: a weighted sparse view is binned without expansion
- **WHEN** a sparse value column is binned with a sparse or dense weight column
- **THEN** the weighted counts are computed from the stored entries and the
  implicit-zero runs without expanding either column

#### Scenario: a sparse aggregate is folded in stored time
- **WHEN** an ungrouped sparse column of a very large logical length is aggregated
- **THEN** the stored entries and the implicit-zero multiplicity are folded without iterating the logical length

#### Scenario: a streaming view is consumed as its current window
- **WHEN** a streaming view is binned
- **THEN** the bins cover the current window and the call returns promptly

### Requirement: Non-finite observations are rejected with their position

Each stat SHALL reject a non-finite observation, weight, group code or independent value with an error naming the offending logical position, on every kernel and in the custom-estimator path. A negative observation weight SHALL be rejected the same way, with a dedicated typed error.

#### Scenario: a NaN observation reports its position
- **WHEN** a value column holds a `NaN` at logical position 3
- **THEN** the operation fails with a non-finite error carrying position 3

#### Scenario: a negative weight reports its position
- **WHEN** a value column holds a negative weight at logical position 2
- **THEN** the operation fails with a negative-weight error carrying position 2

### Requirement: Window semantics are documented and enforced

A stat whose mathematics needs the whole dataset SHALL operate on the window and SHALL report that the result is windowed. Every input column SHALL contribute to that flag, including the independent, group and weight columns and the custom-estimator path.

#### Scenario: a streaming density is marked windowed
- **WHEN** a density is estimated over a stream
- **THEN** the result is marked as windowed

#### Scenario: a streaming weight column is windowed
- **WHEN** the value column is dense but the weight column is streaming
- **THEN** the result is marked as windowed

#### Scenario: a mixed input is windowed
- **WHEN** a fit's value column is dense and its independent column is streaming
- **THEN** the fitted curve is marked as windowed

#### Scenario: a dense view is exact
- **WHEN** a density is estimated over a dense view
- **THEN** the result is exact over that view

### Requirement: Statistical options are typed and validated

Every stat SHALL take a typed option builder, and an invalid configuration SHALL fail with an error before an observation is used or a derived value is produced. A weight column whose values are invalid SHALL fail with the typed weight errors recorded above, and a weighted density whose total weight is not positive SHALL be rejected.

#### Scenario: an invalid bin width is rejected
- **WHEN** a bin width of zero is requested
- **THEN** the operation fails with a typed configuration error

#### Scenario: a quantile outside the unit interval is rejected
- **WHEN** a percentile quantile of `1.5` is requested
- **THEN** the operation fails with a typed configuration error

#### Scenario: a zero total weight is rejected
- **WHEN** every weight of a density is zero
- **THEN** the operation fails with a typed configuration error instead of
  dividing by zero

#### Scenario: a kernel that cannot use weights rejects the column
- **WHEN** a weight column is configured on `Count`, `Percentile`,
  `EmpiricalCdf`, `Fit`, or an `Aggregate` method other than `Mean` and `Sum`
- **THEN** the operation fails with a typed configuration error naming the
  kernel instead of silently ignoring the weight
