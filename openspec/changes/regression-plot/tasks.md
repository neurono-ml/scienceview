# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [x] Create `src/recipes/relational/regression_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVRegressionPlot` carrying its configured kind.
- [x] Wire the composition `data -> [SVStat::Fit] -> (coefficients, predicted curve) -> [no move] -> [SVScale] -> SVMark::Dot (points) + SVMark::Line (fit) + SVMark::Ribbon (interval)` through the core kernels.

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

- [x] Add `examples/regression_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/regression_plot.svg` with `cargo run --example regression_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [x] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [x] Mirror the figure into `docs/src/assets/regression_plot.svg` and embed it in the
  chapter.
- [x] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/relational-regression-plot.md` status to `shipped`, linking the
  example and the chapter.
- [x] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [x] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [x] Open the ADR issue and reference it from both PRs with a closing keyword.
- [x] Trigger the independent acceptance review against
  `docs/FDRs/relational-regression-plot.md` before opening the code PR.

## 8. Review fixes

- [x] Record that `weight` and the separate `ci` selector are deferred to a
  core-stat change because the closed fit kernel consumes neither.
- [x] Record the identical-support sparse pairing rule that keeps the kernel's
  no-densification guarantee.
- [x] Align the spec delta's data contract with the deferred `weight` option,
  so the normative requirements no longer promise a column the closed fit
  kernel cannot consume.

## 9. Review fixes

- [x] Reject a sparse view whose support leaves implicit zeros with
  `UnsupportedRepresentation`, naming the dense and streaming alternatives;
  accept a sparse view only when every logical position is stored. Add
  dense-vs-sparse parity tests with explicit zero observations and the
  rejection path.
- [x] Tick the temporal requirements only when a transition surface exists, or
  defer them in the FDR.
- [x] Replace the claimed-but-hardcoded theme consumption with `theme(...)`
  wiring or an explicit deferral in the FDR.
- [ ] Split `regression_plot_tests.rs` into a `regression_plot_tests/` folder
  module.
- [ ] Route the example through the public `render()`/`save()`, gate it on
  `render-svg`, regenerate the figure and compare it to the checked-in copy.
- [ ] Document the recipe-level scene contract and test the typed error for an
  unavailable backend kind.
- [ ] Strengthen the tests: model-specific coefficients and a band compared to
  the kernel's interval values.
- [x] Rename the module path to `sv_recipes::sv_relational::sv_regression_plot`
  and update every path in code, examples, README and the book.
- [ ] Update the book imports and claims; keep the composition diagram valid;
  `mdbook build docs` green.
