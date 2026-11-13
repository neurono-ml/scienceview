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
- [x] Name the public module tree
  `sv_recipes -> sv_relational -> sv_scatter_plot` with `#[path]` attributes,
  keeping the folders and re-exporting only the plot types from the family.
- [x] Dispatch `render()` through the configured kind: the SVG document for
  `Svg`, the native frame for `Native`, a typed `BackendUnavailable` when the
  feature is compiled out and `UnsupportedBackendCapability` for a kind with no
  recipe serializer; each scene serializer rejects a mismatched kind.

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
  and the point count the current operation resolved from the columns, not the
  build-time probe.

## 4. Example and generated figure

- [x] Add `examples/scatter_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/scatter_plot.svg` with
  `cargo run --example scatter_plot --features render-svg` and verify the
  `figures` gate reports no drift.
- [x] Make the example call the public `to_svg()` method, declare
  `required-features = ["render-svg"]` on its target, and add the
  `the_public_svg_method_matches_the_checked_in_figure` drift test that compares
  the public method's output with the checked-in figure.

## 5. Book chapter (book branch)

- [x] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [x] Mirror the figure into `docs/src/assets/scatter_plot.svg` and embed it in the
  chapter.
- [x] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/recipes/relational-scatter-plot.md` status to `shipped`,
  linking the example and the chapter, and align its axis-label defaults and
  `colour` contract with the code.
- [x] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [x] Run the gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets` (default and `--all-features`),
  `cargo test --workspace --all-targets` (default and `--all-features`),
  `cargo test --workspace --doc`, the examples, the figures check,
  `mdbook build docs` and `openspec validate scatter-plot --strict`.
- [x] Open the ADR issue and reference it from both PRs with a closing keyword.
- [x] Trigger the independent acceptance review against
  `docs/FDRs/recipes/relational-scatter-plot.md` before opening the code PR.
- [x] Address the review blockers: backend dispatch, resolved point count in
  the span, prefixed public module tree, figure drift test, documentation
  corrections and this spec/design correction.
