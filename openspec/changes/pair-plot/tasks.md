# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [x] Create `src/recipes/multivariate/pair_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVPairPlot` carrying its configured kind.
- [x] Wire the composition `data -> [no stat] -> values -> [SVMove::Dodge (+ Jitter) within each cell] -> [SVScale per axis] -> SVMark::Dot, one cell per pair` through the core kernels.

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

- [x] Add `examples/pair_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/pair_plot.svg` with `cargo run --example pair_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [x] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [x] Mirror the figure into `docs/src/assets/pair_plot.svg` and embed it in the
  chapter.
- [x] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/multivariate-pair-plot.md` status to `shipped`, linking the
  example and the chapter.
- [x] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [x] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [x] Open the ADR issue and reference it from both PRs with a closing keyword.
- [x] Trigger the independent acceptance review against
  `docs/FDRs/multivariate-pair-plot.md` before opening the code PR.

## Deviations recorded during implementation

- The FDR lists `dodge` with a default of `true`; a continuous pair grid has no
  categorical slot to dodge into, so the recipe exposes the displacement move as
  an explicit `jitter(...)` option (accepting `Jitter` and `Dodge`) and defaults
  to no displacement. This is reviewed as a follow-up against FDR-144 rather
  than silently claiming the boolean.
- Transition and animation wiring is deferred to the transition integration, as
  in the other recipes of this wave; the composed geometry is transition-ready
  and no transition test is claimed.
- The independent review amended the FDR's configuration and data-contract
  tables to the implemented surface: the continuous-variable scope, the
  explicit `jitter(...)` displacement instead of the `dodge` boolean, the
  deferred `palette` override, the deferred nominal variables and their
  count-plot diagonal, and the implemented `off_diagonal_kind`, `alpha`,
  `missing_value_policy` and `title` options.
- The off-diagonal hexbin and binned-contour cells are chart-owned geometry over
  the closed kernels, not delegated recipe compositions: `SVHexbinPlot` and
  `SVDensityContourPlot` solve their own figure and cannot be handed a single
  pair cell while preserving the shared per-variable domain. ADR #182 records
  the trade-off; the binned contour is deliberately not the KDE-based
  `SVDensityContourPlot`.
- The review fixed a positional-alignment defect in the hue grouping: a missing
  hue value now keeps its slot in the group-index vector, so the observations
  after it keep their category and the SVG matches the equivalent source with
  the row removed. Regression:
  `pair_plot_tests::a_missing_hue_stays_aligned_with_its_observation`.
