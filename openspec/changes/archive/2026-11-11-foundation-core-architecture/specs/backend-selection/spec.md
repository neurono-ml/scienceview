# Spec Delta

## Purpose

Defines how a rendering backend is chosen for every figure and artist, so that
the rendering form is always an explicit, deliberate choice and never an
implicit fallback.

## ADDED Requirements

### Requirement: Explicit backend selection is mandatory

Every figure and artist builder SHALL require a backend selection before the
figure or artist can be built, and the selection SHALL be one of the supported
backend kinds.

#### Scenario: Builder without a backend selection
- **WHEN** a user builds a figure without selecting a backend
- **THEN** the build fails with an error stating that a backend selection is required

#### Scenario: Builder with a supported backend selection
- **WHEN** a user selects a supported backend kind and builds the figure
- **THEN** the built figure carries the selected backend kind

### Requirement: No automatic backend selection

The backend selection SHALL NOT offer an automatic or default variant, and
enabling the Cargo feature of a backend SHALL NOT cause that backend to be
selected.

#### Scenario: Feature enabled but not selected
- **WHEN** a user enables the Cargo feature of a backend but selects a different backend
- **THEN** the figure is rendered by the selected backend only

#### Scenario: No default value
- **WHEN** a user inspects the available backend kinds
- **THEN** no automatic or default variant is offered among them

### Requirement: Backend kind is a runtime value

The figure SHALL remain non-generic with respect to the backend, apart from an
optional scalar type parameter used when the figure's configuration carries
scalar values.

#### Scenario: Same figure type across backends
- **WHEN** a user holds a figure built for one backend
- **THEN** the same type can be built and used for another backend without changing its type parameters

#### Scenario: Switching the backend at runtime
- **WHEN** a user changes the backend kind of an existing figure
- **THEN** subsequent operations are performed by the new backend

### Requirement: Operations record the configured backend kind

Every public operation of a figure or artist SHALL record the configured
backend kind in its observability span.

#### Scenario: Observing an operation
- **WHEN** an operation is executed on a figure configured for a given backend kind
- **THEN** the observability span of that operation carries the configured backend kind

### Requirement: Dispatch happens per operation, not at build time

The concrete backend implementation SHALL be selected when an operation is
performed, and building a figure SHALL NOT perform rendering work.

#### Scenario: Building performs no rendering
- **WHEN** a user builds a figure
- **THEN** no rendering is performed and no backend output is produced

#### Scenario: Dispatch at operation time
- **WHEN** a user performs a rendering operation on a built figure
- **THEN** the concrete backend implementation is resolved and invoked at that moment

### Requirement: The backend kind is carried even when no heavy work is needed

The backend kind SHALL be carried and recorded for every figure, including
figures whose rendering only performs text layout.

#### Scenario: Text-only figure
- **WHEN** a user builds a figure that only lays out text
- **THEN** the configured backend kind is still carried and recorded in the operation span
