# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [x] Create `src/recipes/relational/step_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVStepPlot` carrying its configured kind.
- [x] Wire the composition `data -> [SVStat::Aggregate] -> values -> [no move] -> [SVScale] -> SVMark::Line` through the core kernels.

## 2. Data contract and parity

- [x] Write the failing parity test: dense, sparse and streaming input of the
  same data produce the same marks within the documented tolerance.
- [x] Implement the column resolution and the explicit error for a missing or
  mistyped column.
- [x] Verify a sparse input is never densified and a streaming input never
  blocks; assert the window behaviour where the recipe depends on it.

## 3. Concurrency and observability

- [x] Write the failing concurrency test for rendering the same figure from
  several threads.
- [x] Assert the operation span carries the backend kind, the representation
  and the point count.

## 4. Example and generated figure

- [x] Add `examples/step_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/step_plot.svg` with `cargo run --example step_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [x] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [x] Mirror the figure into `docs/src/assets/step_plot.svg` and embed it in the
  chapter.
- [x] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/recipes/relational-step-plot.md` status to `shipped`, linking the
  example and the chapter.
- [x] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [x] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [x] Open the ADR issue and reference it from both PRs with a closing keyword.
- [x] Trigger the independent acceptance review against
  `docs/FDRs/recipes/relational-step-plot.md` before opening the code PR.

## 8. Review fixes

- [x] Document the SVG-only serializer scope, the folded `where` option and
  the recipe-owned vertex expansion in `design.md` and the spec delta.
- [x] Align the step geometry and acceptance evidence with the shipped tests,
  example and chapter.

## 9. Acceptance re-review

- [x] Make the update test append a real sample through a live source and
  assert that the advanced window changes the output while preserving the
  prior samples; rename the claim to what the recipe does.
- [x] Record the drawn point count in every operation span, not the logical
  column length, and cover a 100k-logical sparse column with two drawn points.
- [x] Consume `SVStepPosition::ALL` in the step-position geometry test.
- [x] Untick the incremental-append and animation acceptance items with an
  explicit deferral to the transition integration.
- [x] Dispatch the serializer from the configured backend kind and return
  `SVError::BackendUnavailable` for a kind without a serializer; document the
  recipe-level scene contract.
- [x] Expose `scienceview::sv_recipes::sv_relational::sv_step_plot` through
  `#[path]` and update the example and README import path.
- [x] Refresh the mdBook chapter, its figure mirror and the pinned code
  revision.
