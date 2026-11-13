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

- Deferred (decision 2026-11-13): weighted binning stays out of scope because
  the shipped `SVStat::Bin` kernel does not consume observation weights; a
  configured `weight` is rejected with an error naming the kernel gap. The
  weights support belongs to the future `statistical-weights` change (weighted
  `SVStat::Bin`); this change makes no acceptance claim for it.
- Deferred (decision 2026-11-13): in-place streaming update stays out of scope;
  window recomposition is the shipped behaviour, and growing the bars without a
  redraw belongs to a future recipe-streaming integration change. No acceptance
  claim is made.
- Deferred (decision 2026-11-13): recipe-level animation wiring is out of scope;
  recipes ship transition-ready geometry and the wiring belongs to a future
  recipe-animation integration change. No acceptance claim is made.

## Implementation notes

- 2026-11-13: the three deferrals above are recorded as explicit decisions with
  their reason and owner change; no box is ticked because none of the three
  capabilities shipped here.
