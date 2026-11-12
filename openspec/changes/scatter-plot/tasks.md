# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [x] Create `src/recipes/relational/scatter_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVScatterPlot` carrying its configured kind.
- [x] Wire the composition `data -> [no stat] -> values -> [SVMove::Dodge | Jitter] -> [SVScale x, y, colour] -> SVMark::Dot` through the core kernels.

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

- [x] Add `examples/scatter_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/scatter_plot.svg` with `cargo run --example scatter_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [ ] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [ ] Mirror the figure into `docs/src/assets/scatter_plot.svg` and embed it in the
  chapter.
- [ ] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/relational-scatter-plot.md` status to `shipped`, linking the
  example and the chapter.
- [x] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [ ] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [ ] Open the ADR issue and reference it from both PRs with a closing keyword.
- [ ] Trigger the independent acceptance review against
  `docs/FDRs/relational-scatter-plot.md` before opening the code PR.
