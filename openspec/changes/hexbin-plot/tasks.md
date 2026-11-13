# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [x] Create `src/recipes/matrix/hexbin_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVHexbinPlot` carrying its configured kind.
- [x] Wire the composition `data -> [SVStat::Bin (2D, hexagonal lattice)] -> (cells, counts) -> [SVScale::Continuous over a colour map] -> SVMark::HexTile` through the core kernels.

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

- [x] Add `examples/hexbin_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/hexbin_plot.svg` with `cargo run --example hexbin_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [x] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [x] Mirror the figure into `docs/src/assets/hexbin_plot.svg` and embed it in the
  chapter.
- [x] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/matrix-hexbin-plot.md` status to `shipped`, linking the
  example and the chapter.
- [x] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [x] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [x] Open the ADR issue and reference it from both PRs with a closing keyword.
- [x] Trigger the independent acceptance review against
  `docs/FDRs/matrix-hexbin-plot.md` before opening the code PR.

## 8. Review fixes (implementation divergence)

- [x] Record that the closed stat vocabulary has no bivariate hexagonal
  variant: the recipe owns the lattice geometry (like the density-contour
  recipe owns marching squares), while positions come from `sv_scale` and mean
  aggregation is delegated to `SVStat::Aggregate`.
- [x] Extend the documented configuration with the optional `value` column and
  the `aggregation` mode (`Count` or `Mean`), and reject `value` with count
  aggregation instead of silently ignoring it.
- [x] Cover the span backend kind, representation and drawn-cell count.
