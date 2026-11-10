# Spec Delta

## Purpose

Defines how data enters the library and how it is represented for computation,
so that any supported data source works with any artist without densifying
sparse data or blocking the render thread on a stream.

## ADDED Requirements

### Requirement: Sources implement a public ingestion contract

Data sources SHALL be pluggable through a public source contract, so that a
third party can provide a source without modifying the core.

#### Scenario: Third-party source
- **WHEN** a user implements the public source contract for an external data system
- **THEN** the resulting source can be passed to any artist that accepts that data kind

#### Scenario: Declared schema
- **WHEN** a source is attached to an artist
- **THEN** the source exposes the names and roles of its columns

### Requirement: Representation is resolved once per operation

A source SHALL expose its data as one of exactly three representations — dense,
sparse or streaming — and the receiving operation SHALL dispatch on that
representation exactly once.

#### Scenario: Dispatch is single
- **WHEN** an operation reads data from a source
- **THEN** the representation is resolved once and the operation proceeds on that representation

### Requirement: Sparse data is never densified

An operation SHALL NOT convert a sparse representation into a dense one as part
of reading the data.

#### Scenario: Sparse input stays sparse
- **WHEN** an operation reads a sparse representation
- **THEN** the underlying storage is not expanded to a dense buffer

### Requirement: Streaming data never blocks the render thread

Reading from a streaming representation SHALL NOT block waiting for new data,
and an operation on a streaming representation SHALL return using the data
available at that moment.

#### Scenario: Stream with no new data
- **WHEN** an operation reads a streaming representation while no new data has arrived
- **THEN** the operation returns with the currently available data instead of waiting

### Requirement: Source adaptors cover the documented data systems

The library SHALL provide adaptors that expose borrowed slices, `ndarray`
arrays, sparse index-and-value storage, standard-library channels, arbitrary
streams, Polars frames and DataFusion record batches through the public source
contract.

#### Scenario: Adaptor produces a view
- **WHEN** a user attaches one of the documented adaptors to an artist
- **THEN** the adaptor presents the data as one of the three representations, without copying it into an owned array

### Requirement: Representation parity across artists

An artist that mathematically tolerates more than one representation SHALL
accept dense, sparse and streaming inputs on every public operation.

#### Scenario: All three representations accepted
- **WHEN** a user calls the same public operation with dense, sparse and streaming inputs
- **THEN** each call succeeds and produces the same rendered result within the documented tolerance

### Requirement: Multi-series operations accept every representation combination

An operation that receives more than one series SHALL accept every combination
of dense, sparse and streaming series, after validating that the series have
matching lengths.

#### Scenario: Every combination of series representations
- **WHEN** a user calls a multi-series operation with series drawn from dense, sparse and streaming sources, in any combination
- **THEN** the operation succeeds and produces the same rendered result within the documented tolerance

#### Scenario: Mismatched series lengths
- **WHEN** a user calls a multi-series operation with series of different lengths
- **THEN** the operation fails with an error stating the mismatched lengths, before any series is read

### Requirement: Static-only exceptions are explicit

When the mathematics of an artist genuinely requires full-data geometry, the
artist SHALL reject a streaming view with an explicit unsupported-representation
error naming a documented alternative path.

#### Scenario: Rejected streaming view
- **WHEN** a user calls the operation of a static-only artist with a streaming view
- **THEN** the call fails with an unsupported-representation error that names the alternative path to use

### Requirement: Sources declare their reactivity

A source SHALL declare how changes reach it, choosing among static, observed,
event-driven and manual reactivity.

#### Scenario: Declared reactivity is visible
- **WHEN** a user inspects a source
- **THEN** the source reports its reactivity mode

#### Scenario: Manual reactivity
- **WHEN** a source declares manual reactivity
- **THEN** the data of that source changes only when the owner explicitly pushes an update

### Requirement: Asynchronous producers are adapted without a core runtime

A source fed by an asynchronous producer SHALL present a synchronous view and
SHALL NOT require the core to depend on an asynchronous runtime.

#### Scenario: Asynchronous producer behind a channel
- **WHEN** a user attaches a source fed by a channel of record batches
- **THEN** the source presents a streaming view synchronously and the core requires no async runtime

#### Scenario: Multiple lengths in one source
- **WHEN** a source's columns have different lengths
- **THEN** attaching that source to an artist fails with an error stating the mismatched lengths

### Requirement: Data sources are shared by reference

The same source SHALL be attachable to multiple artists and figures, so that a
selection or filter applied to the source is observed by all of them.

#### Scenario: Shared source across figures
- **WHEN** a user attaches one source to two figures and changes the source
- **THEN** both figures observe the change
