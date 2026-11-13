# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [x] Create `src/recipes/distribution/histogram_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVHistogramPlot` carrying its configured kind.
- [x] Wire the composition `data -> [SVStat::Bin] -> (edges, counts) -> [SVMove::Stack | Dodge] -> [SVScale x, y, colour] -> SVMark::Bar` through the core kernels.

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

- [x] Add `examples/histogram_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/histogram_plot.svg` with `cargo run --example histogram_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [x] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [x] Mirror the figure into `docs/src/assets/histogram_plot.svg` and embed it in the
  chapter.
- [x] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/recipes/distribution-histogram-plot.md` status to `shipped`, linking the
  example and the chapter.
- [x] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [x] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [x] Open the ADR issue and reference it from both PRs with a closing keyword.
- [x] Trigger the independent acceptance review against
  `docs/FDRs/recipes/distribution-histogram-plot.md` before opening the code PR
  (GitHub Copilot review requested on PR #109).

## 8. Deferred (recorded, not ticked)

- [x] Weighted binning. Delivered by the `statistical-weights` change: the
  `SVStat::Bin` kernel now consumes an observation weight column and the
  recipe's `weight(...)` carries it, replacing the former build-time rejection.
- Deferred (decision 2026-11-13): an in-place streaming update that grows the
  bars without a redraw belongs to the transition/streaming integration. The
  shipped `update` recomposes the current window and returns the new artifact;
  no acceptance claim is made for incremental growth.
- Deferred (decision 2026-11-13): transition wiring. The FDR temporal behaviour
  is a design target; the recipe ships transition-ready geometry and the
  interpolation belongs to a future recipe-animation integration change.
