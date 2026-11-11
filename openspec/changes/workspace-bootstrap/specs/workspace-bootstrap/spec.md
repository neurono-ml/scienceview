# Spec Delta

## Purpose

The repository compiles as a Cargo workspace with a pinned toolchain, so that
every later change has a build unit, a reproducible compiler and an automated
quality gate.

## ADDED Requirements

### Requirement: The repository is a workspace

The repository SHALL build as a Cargo workspace whose members are the core crate and one crate per rendering backend.

#### Scenario: the repository is a workspace
- **WHEN** a developer runs `cargo build --workspace`
- **THEN** every member crate is built, including the backends

#### Scenario: the repository is a workspace
- **WHEN** a backend feature is not enabled
- **THEN** its crate is not compiled into the default build

### Requirement: The toolchain is pinned

The repository SHALL declare the Rust toolchain it supports, and building with an older toolchain SHALL fail with a clear message rather than a compile error deep in a dependency.

#### Scenario: the toolchain is pinned
- **WHEN** a developer uses the pinned toolchain
- **THEN** the workspace builds

#### Scenario: the toolchain is pinned
- **WHEN** a developer uses an older toolchain
- **THEN** the build fails with the declared minimum version

### Requirement: The quality gates run in CI

Every pull request SHALL run formatting, static analysis, tests and the figure drift check, and SHALL fail when any of them fails.

#### Scenario: the quality gates run in ci
- **WHEN** formatting differs from the formatter output
- **THEN** the CI check fails

#### Scenario: the quality gates run in ci
- **WHEN** a clippy warning is introduced
- **THEN** the CI check fails, because warnings are errors

#### Scenario: the quality gates run in ci
- **WHEN** an example produces a figure different from the committed one
- **THEN** the figures job fails

### Requirement: Figures have a home on the code branch

The generated figures SHALL live under `docs/figures/` on the code branch, written by the repository's own examples.

#### Scenario: figures have a home on the code branch
- **WHEN** an example is run
- **THEN** its SVG is written under `docs/figures/`

#### Scenario: figures have a home on the code branch
- **WHEN** a figure is edited by hand
- **THEN** the figures job fails because rerunning the example produces a different file

