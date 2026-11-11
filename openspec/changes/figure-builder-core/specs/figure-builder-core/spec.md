# Spec Delta

## Purpose

Every figure and artist is constructed through a builder that requires an
explicit rendering backend, carries an execution mode, and dispatches to the
backend at operation time.

## ADDED Requirements

### Requirement: The backend is explicit and required

Every figure and artist builder SHALL require an explicit backend selection, and the system SHALL NOT provide an automatic backend variant.

#### Scenario: the backend is explicit and required
- **WHEN** a builder is finalized without a backend
- **THEN** the program does not compile or `build` returns an explicit configuration error

#### Scenario: the backend is explicit and required
- **WHEN** a backend is selected
- **THEN** the built figure carries that kind for its whole lifetime

### Requirement: Dispatch happens per operation

The backend kind SHALL be resolved to a concrete backend on every operation (`render`, `update`, `save`, `show`), never during `build`.

#### Scenario: dispatch happens per operation
- **WHEN** a figure is built
- **THEN** no backend is resolved yet

#### Scenario: dispatch happens per operation
- **WHEN** `render` is called
- **THEN** the kind is dispatched to the concrete backend for that call

### Requirement: The execution mode defaults to automatic

Every builder SHALL accept an execution mode, defaulting to `Automatic`, and an explicit mode SHALL be honoured when the backend supports it.

#### Scenario: the execution mode defaults to automatic
- **WHEN** no execution mode is given
- **THEN** the builder uses `Automatic`

#### Scenario: the execution mode defaults to automatic
- **WHEN** an explicit mode is given
- **THEN** the built figure uses it

### Requirement: Unsupported combinations fail at build time

When the selected backend does not support a requested capability, `build` SHALL fail with an explicit error rather than degrade silently at runtime.

#### Scenario: unsupported combinations fail at build time
- **WHEN** a streaming figure is built for an export-only backend
- **THEN** `build` returns an error naming the unsupported capability

#### Scenario: unsupported combinations fail at build time
- **WHEN** a supported combination is built
- **THEN** `build` succeeds

### Requirement: Operations are observable

Every public operation SHALL emit a span recording the configured backend kind, the data representation and the number of drawn marks.

#### Scenario: operations are observable
- **WHEN** `render` runs
- **THEN** one span is emitted with the three fields

#### Scenario: operations are observable
- **WHEN** the backend fails
- **THEN** the span records the failure and the error is returned

