# Tasks

## 1. Module, builder and composition

- [ ] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [ ] Create `src/recipes/multivariate/parallel_coordinates_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [ ] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVParallelCoordinatesPlot` carrying its configured kind.
- [ ] Wire the composition `data -> [no stat] -> values -> [SVMove::Stack? no — one line per observation] -> [SVScale per axis] -> SVMark::Line` through the core kernels.

## 2. Data contract and parity

- [ ] Write the failing parity test: dense, sparse and streaming input of the
  same data produce the same marks within the documented tolerance.
- [ ] Implement the column resolution and the explicit error for a missing or
  mistyped column.
- [ ] Verify a sparse input is never densified and a streaming input never
  blocks; assert the window behaviour where the recipe depends on it.

## 3. Concurrency and observability

- [ ] Write the failing concurrency test for rendering the same figure from
  several threads.
- [ ] Assert the operation span carries the backend kind, the representation
  and the point count.

## 4. Example and generated figure

- [ ] Add `examples/parallel_coordinates_plot.rs` demonstrating the documented configuration.
- [ ] Generate `docs/figures/parallel_coordinates_plot.svg` with `cargo run --example parallel_coordinates_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [ ] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [ ] Mirror the figure into `docs/src/assets/parallel_coordinates_plot.svg` and embed it in the
  chapter.
- [ ] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [ ] Change `docs/FDRs/multivariate-parallel-coordinates-plot.md` status to `shipped`, linking the
  example and the chapter.
- [ ] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [ ] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [ ] Open the ADR issue and reference it from both PRs with a closing keyword.
- [ ] Trigger the independent acceptance review against
  `docs/FDRs/multivariate-parallel-coordinates-plot.md` before opening the code PR.
