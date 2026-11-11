# Spec Delta

## Purpose

The library is usable from Python with the same modelling, the same explicit backend selection and the same data sources as the Rust API.

## ADDED Requirements

### Requirement: Builders are exposed

The Python surface SHALL expose every figure builder with the same options as the Rust builder.

#### Scenario: Building a scatter plot
- **WHEN** a Python user builds a scatter plot
- **THEN** the available options match the Rust builder

### Requirement: The backend stays explicit

The Python surface SHALL require an explicit backend selection and SHALL NOT provide an automatic default.

#### Scenario: Missing backend
- **WHEN** a figure is built without a backend
- **THEN** construction fails with an explicit error

### Requirement: Data sources are equivalent

The Python surface SHALL accept the same representations, including streaming updates.

#### Scenario: Streaming in Python
- **WHEN** a Python producer pushes data
- **THEN** the figure updates in place

