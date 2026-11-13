# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [x] Create `src/recipes/distribution/density_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVDensityPlot` carrying its configured kind.
- [x] Wire the composition `data -> [SVStat::KernelDensity] -> (grid, density) -> [no move] -> [SVScale] -> SVMark::Line | SVMark::Area` through the core kernels.

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

- [x] Add `examples/density_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/density_plot.svg` with `cargo run --example density_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [x] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [x] Mirror the figure into `docs/src/assets/density_plot.svg` and embed it in the
  chapter.
- [x] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/distribution-density-plot.md` status to `shipped`, linking the
  example and the chapter.
- [x] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [x] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [x] Open the ADR issue and reference it from both PRs with a closing keyword.
- [x] Trigger the independent acceptance review against
  `docs/FDRs/distribution-density-plot.md` before opening the code PR. The
  review verified the spec, conventions, gates and the cross-merge union, and
  recorded the weight and transition deferrals in the spec delta.

## Follow-ups (not this change)

- [ ] Add the `weight` column once a weighted kernel-density variant exists in
  the statistical-transform capability. The shipped kernel applies an
  unweighted estimate, so the recipe deliberately omits the column instead of
  reimplementing the kernel (patterns.md P5); the FDR amendment records it.
- [ ] Wire transition animation once the transition integration reaches the
  distribution recipes. The scene already emits transition-ready geometry (one
  curve per series over a fixed grid).
