# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [x] Create `src/recipes/relational/area_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVAreaPlot` carrying its configured kind.
- [x] Wire the composition `data -> [SVStat::Aggregate] -> values -> [SVMove::Stack] -> [SVScale] -> SVMark::Area` through the core kernels.

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

- [x] Add `examples/area_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/area_plot.svg` with `cargo run --example area_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [x] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [x] Mirror the figure into `docs/src/assets/area_plot.svg` and embed it in the
  chapter.
- [x] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/relational-area-plot.md` status to `shipped`, linking the
  example and the chapter.
- [x] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [x] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [x] Open the ADR issue and reference it from both PRs with a closing keyword.
- [x] Trigger the independent acceptance review against
  `docs/FDRs/recipes/relational-area-plot.md` before opening the code PR.

## 8. Review follow-ups

- [x] Compose `SVMark::Ribbon` for stack=true: the area mark closes against a
  scalar baseline and cannot express the per-point cumulative baseline
  `SVMove::Stack` returns.
- [x] Place a temporal ordered axis through a numeric placement scale that
  shares the temporal axis domain and range, because the mark kernel places
  numeric positions while `SVScaleKind::Temporal` answers timestamps.
- [x] Reject a backend with no recipe rendering path at build time; only the
  SVG serializer is wired in this change.
- [x] Honour the missing-value policy while stacking: the shared grid used to
  read a non-finite sample as a zero before the policy ran, so `Error` did not
  reject a stacked render. Covered by
  `a_strict_missing_value_policy_rejects_a_stacked_render`.
- [x] Prove stacking tiles: `the_stacked_areas_tile_against_the_cumulative_baseline`
  asserts that the upper area's baseline is the lower area's top, instead of
  only counting primitives.
- [x] Close the P6 density gap: the preparation resolves each column once into
  a representation-aware accessor and builds only the vertices the geometry
  needs. `a_million_logical_sparse_column_is_not_expanded`,
  `a_stacked_million_logical_sparse_column_is_bounded` and
  `a_million_logical_sparse_column_allocates_proportionally_to_its_stored_slots`
  cover the sparse paths, and the strict policy still rejects a non-finite
  stored value while stacking.
- [x] Split the modules that exceed 200 lines into folder modules (P3):
  `builder/`, `core_implementation/`, `preparation/` and `rendering/`, each
  with a pure `mod.rs` dispatcher.
- [x] Dispatch `render()` on the configured kind: the SVG document and the
  native raster frame replay the same `SVAreaRendering` scene, and a kind with
  no serializer fails at build with `UnsupportedBackendCapability`. `save()`
  exports only SVG because the native backend presents a frame; the
  recipe-level scene contract is recorded in the FDR.
- [x] Remove `update(appended_points)` instead of ignoring its argument, and
  record the in-place update deferral in FDR-103, the design and this spec.
- [x] Rename the public modules to `sv_recipes::sv_relational::sv_area_plot`
  through `#[path]`, and qualify the missing-value policy as
  `SVAreaMissingValuePolicy` so the family re-export cannot collide.
- [x] Keep the follow-up commits scoped to one review blocker each; the first
  implementation history is unchanged because rewriting merged history is not
  a follow-up.
