# Tasks

## Binding surface

- [x] Write the failing tests for builder parity with the Rust API.
- [x] Implement the builders and their options.

## Data sources and streaming

- [x] Write the failing tests for dense, sparse and streaming input from Python.
- [x] Implement the data source bindings.

## Packaging

- [x] Add the packaging metadata and the wheel build.
- [x] Verify installation in a clean environment.

## Acceptance

- [x] Add the example, generated figure and book chapter (book branch).
- [x] Run the four gates and open the ADR issue.


## Review fixes

- [x] Bind the recipe set one adapter at a time (`figure`, `scatter`, `line`, `area`, `histogram`) and report the bound set through `plot_kinds()`; a kind without an adapter is not constructible.
- [x] Fix the pre-existing `splom_plot` test import so the SVG feature alone compiles its tests, which the binding's default `render-svg` surfaces in the workspace gates.
