# Spec Delta

## Purpose

Defines what each rendering backend is able to do, so that a user learns about
an unsupported combination when building a figure rather than when the
interaction silently fails at runtime.

## ADDED Requirements

### Requirement: Backends declare their capabilities

Every backend SHALL declare whether it supports user events, scripting,
streaming updates, in-place update of an existing output, and which export
formats it can produce.

#### Scenario: Declared capabilities are inspectable
- **WHEN** a user inspects the capabilities of a backend
- **THEN** the backend reports its support for events, scripting, streaming, in-place update and export formats

### Requirement: Unsupported combinations fail at build time

When a figure requests a capability that the selected backend does not
support, the build SHALL fail with an explicit error naming the missing
capability.

#### Scenario: Events requested on a static backend
- **WHEN** a user registers a callback on a figure whose backend does not support events
- **THEN** the build fails with an error naming the unsupported capability and the backend

#### Scenario: Streaming requested on a static backend
- **WHEN** a user attaches a streaming source to a figure whose backend does not support streaming
- **THEN** the build fails with an error naming the unsupported capability

### Requirement: Backends support in-place update

A backend that renders interactive figures SHALL be able to update an already
rendered output in place, without the host having to rebuild the figure.

#### Scenario: Update without rebuild
- **WHEN** a user applies a mutation to a rendered figure
- **THEN** the already rendered output reflects the change without the figure being rebuilt

### Requirement: Export formats are explicit

Exporting a figure SHALL require the chosen format to be one the backend
declares as supported.

#### Scenario: Unsupported export format
- **WHEN** a user requests an export format the backend does not support
- **THEN** the export fails with an error naming the unsupported format

### Requirement: Backend output can be embedded in a host

A backend SHALL be able to produce output that a host application can embed,
so that a figure can take part in a larger page or application.

#### Scenario: Embedded output
- **WHEN** a user embeds a figure in a page or application
- **THEN** the produced output is self-contained and does not require the host to supply additional assets
