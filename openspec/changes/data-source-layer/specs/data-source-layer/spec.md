# Spec Delta

## Purpose

Data enters the library through an extensible source trait and is presented to
computation as exactly one of three representations: dense, sparse or
streaming.

## ADDED Requirements

### Requirement: Ingestion is an open trait

Any producer SHALL be able to feed a figure by implementing the data source trait, without modifying the core.

#### Scenario: ingestion is an open trait
- **WHEN** a third-party crate implements the trait
- **THEN** its data can be used by any artist

#### Scenario: ingestion is an open trait
- **WHEN** a built-in source is used
- **THEN** it is implemented through the same trait

### Requirement: Computation sees exactly three representations

Every kernel SHALL receive a view that is dense, sparse or streaming, and SHALL dispatch on it exactly once per operation.

#### Scenario: computation sees exactly three representations
- **WHEN** a dense source is consumed
- **THEN** the kernel takes the dense branch once

#### Scenario: computation sees exactly three representations
- **WHEN** a sparse source is consumed
- **THEN** the kernel takes the sparse branch and the data is not expanded

#### Scenario: computation sees exactly three representations
- **WHEN** a streaming source is consumed
- **THEN** the kernel takes the streaming branch and does not block

### Requirement: Streaming is bounded and non-blocking

A streaming source SHALL expose a bounded rolling window and SHALL NOT block the render thread.

#### Scenario: streaming is bounded and non-blocking
- **WHEN** data arrives faster than it is rendered
- **THEN** the window drops the oldest entries and the render returns promptly

#### Scenario: streaming is bounded and non-blocking
- **WHEN** no new data arrives
- **THEN** the operation returns the current window

### Requirement: Representations are equivalent

For the same data, dense, sparse and streaming inputs SHALL produce results equal within the documented tolerance.

#### Scenario: representations are equivalent
- **WHEN** the same dataset is presented three ways
- **THEN** the outputs agree within tolerance

#### Scenario: representations are equivalent
- **WHEN** a sparse input is presented as dense
- **THEN** the outputs agree exactly

