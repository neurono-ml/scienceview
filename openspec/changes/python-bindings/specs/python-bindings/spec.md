# Spec Delta

## Purpose

The library is usable from Python with the same modelling, the same explicit backend selection and the same data sources as the Rust API.

## ADDED Requirements

### Requirement: The bound recipe set is exposed

The Python surface SHALL expose each recipe it binds through keyword arguments
that mirror that recipe's Rust builder, and SHALL report the bound set through
`plot_kinds()`. A recipe without an adapter SHALL NOT be constructible and
SHALL NOT be silently approximated.

#### Scenario: Building a scatter plot
- **WHEN** a Python user builds a scatter plot
- **THEN** the accepted keywords are exactly the ones `options_for("scatter")`
  declares, and each one reaches the Rust builder

#### Scenario: Asking for an unbound recipe
- **WHEN** a Python user calls a constructor whose recipe has no adapter
- **THEN** the kind is absent from `plot_kinds()` and cannot be built

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

