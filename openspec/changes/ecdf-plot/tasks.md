# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [x] Create `src/recipes/distribution/ecdf_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVECDFPlot` carrying its configured kind.
- [x] Wire the composition `data -> [SVStat::EmpiricalCdf] -> (values, cumulative probability) -> [no move] -> [SVScale] -> SVMark::Line (step geometry)` through the core kernels.

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

- [x] Add `examples/ecdf_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/ecdf_plot.svg` with `cargo run --example ecdf_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [x] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [x] Mirror the figure into `docs/src/assets/ecdf_plot.svg` and embed it in the
  chapter.
- [x] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/distribution-ecdf-plot.md` status to `shipped`, linking the
  example and the chapter.
- [x] Add the `CHANGELOG.md` entry.

## 6b. Review-fix notes

- [x] The `SVStat::EmpiricalCdf` kernel does not consume observation weights.
  Per patterns P5 the recipe does not reimplement weighting and exposes
  `value`/`series` only; the FDR records the gap as a core change to the stat
  capability. This is a reported deviation from the spec-delta wording
  "consume `value`, `series`, `weight`" and must be resolved by a later core
  change, not by this recipe.
- [x] The complementary (survival) option, left open by FDR-114, is exposed as
  `complementary(bool)` derived from the same kernel probabilities and covered
  by `the_complementary_curve_descends_from_one`.
- [x] Transition and animation wiring is deferred; the recipe claims only
  transition-ready geometry, as recorded by the integration note.
- [x] The step is expanded once in (value, probability) space and the channels
  are swapped for horizontal orientation, so the curve starts at probability
  zero on the minimum observation instead of inventing an observation at value
  zero. Covered by `horizontal_orientation_expands_the_same_step_vertices` and
  `a_complementary_horizontal_curve_starts_at_one_on_the_minimum`.
- [x] An empty or all-missing column reports the recipe's own
  `EmptyInput { operation: "ecdf_plot" }` instead of leaking the kernel's
  operation name. Covered by `an_empty_column_is_an_empty_input` and
  `an_all_missing_column_is_an_empty_input`.
- [x] Degenerate samples are covered: `one_observation_rises_to_one_at_its_only_value`
  and `all_equal_observations_produce_one_riser`.
- [x] The observability tests retry the span capture, so the process-wide
  tracing interest cache rebuilt by sibling test modules cannot make them
  flaky.

## 7. Acceptance

- [x] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [x] Open the ADR issue and reference it from both PRs with a closing keyword.
- [x] Trigger the independent acceptance review against
  `docs/FDRs/distribution-ecdf-plot.md` before opening the code PR.
