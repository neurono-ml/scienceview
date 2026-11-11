# Spec Delta

## Purpose

Statistical transforms turn raw observations into the derived values a chart
draws — bins, counts, estimates, densities, cumulative probabilities,
percentiles and fitted curves — with the same result for dense, sparse and
streaming input.

## ADDED Requirements

### Requirement: The stat set is closed and complete

The system SHALL provide binning, counting, aggregation, kernel density, empirical CDF, percentiles and fitting, and recipes SHALL compose exactly one of them.

#### Scenario: the stat set is closed and complete
- **WHEN** a histogram is built
- **THEN** the bin stat produces edges and counts

#### Scenario: the stat set is closed and complete
- **WHEN** a regression plot is built
- **THEN** the fit stat produces coefficients and a curve

### Requirement: Statistics respect the representation

Every stat SHALL accept dense, sparse and streaming views, dispatch once, never densify a sparse view and never block on a stream.

#### Scenario: statistics respect the representation
- **WHEN** a sparse view is binned
- **THEN** the bins are computed from indices and values without expansion

#### Scenario: statistics respect the representation
- **WHEN** a streaming view is binned
- **THEN** the bins cover the current window and the call returns promptly

### Requirement: Window semantics are documented and enforced

A stat whose mathematics needs the whole dataset SHALL operate on the window and SHALL report that the result is windowed.

#### Scenario: window semantics are documented and enforced
- **WHEN** a density is estimated over a stream
- **THEN** the result is marked as windowed

#### Scenario: window semantics are documented and enforced
- **WHEN** a density is estimated over a dense view
- **THEN** the result is exact over that view

### Requirement: Heavy statistics do not block rendering

Density estimation and fitting SHALL run on the compute pool and SHALL never block the render thread.

#### Scenario: heavy statistics do not block rendering
- **WHEN** a large fit is requested while a frame is rendered
- **THEN** the frame is not delayed by the fit

