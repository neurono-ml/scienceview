# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, the supplied order moves the cells, and the dendrograms reproduce
  the supplied merge distances.
- [x] Create `src/recipes/matrix/clustered_heatmap_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the supplied-clustering builder: required `backend(...)`,
  default `execution_mode(...)`, required `row_order(...)` and
  `column_order(...)`, optional `row_linkage(...)` / `column_linkage(...)`,
  and `SVClusteredHeatmapPlot` carrying its configured kind.
- [x] Wire the composition `data -> [SVStat::Aggregate (matrix)] -> matrix +
  supplied orders and trees -> [validate] -> [SVScale::Continuous over a colour
  map] -> SVMark::Image (cells) + supplied dendrogram paths`.
- [x] Add the typed `SVError::InvalidClusterTree` variant naming the axis and
  merge position.

## 2. Data contract and validation

- [x] Write the failing tests: a sparse value column is read in place, a
  streaming source is rejected up front, and a dense matrix and a long table
  paint the same cells.
- [x] Implement the column resolution and the explicit error for a missing or
  mistyped column.
- [x] Validate the supplied order and tree per axis: leaves exactly once,
  indices in range and earlier, finite non-decreasing distances, consistent
  leaf counts, permutation order and matching length.
- [x] Verify a sparse input is never densified and a streaming input is
  rejected with `UnsupportedRepresentation` plus the snapshot alternative.

## 3. Concurrency and observability

- [x] Write the failing concurrency test for rendering the same figure from
  several threads.
- [x] Assert the operation span carries the backend kind, the representation
  and the drawn cell count.

## 4. Example and generated figure

- [x] Add `examples/clustered_heatmap_plot.rs` with a small deterministic
  matrix and a hand-written, commented merge tree.
- [x] Generate `docs/figures/clustered_heatmap_plot.svg` with
  `cargo run --example clustered_heatmap_plot --features render-svg` and verify
  the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [x] Add the chapter `ch54-clustered-heatmap-plot.md` under
  `docs/src/development-explained/`.
- [x] Mirror the figure into `docs/src/assets/clustered_heatmap_plot.svg` and
  embed it in the chapter with an SVG diagram.
- [x] Register the chapter in `docs/src/SUMMARY.md`, update
  `maintaining.md` and `docs/src/examples.md`, and verify `mdbook build`.

## 6. FDR and changelog

- [x] Rewrite `docs/FDRs/recipes/matrix-clustered-heatmap-plot.md` to the
  supplied-clustering contract and set the status to `shipped`, ticking only
  tested acceptance items.
- [x] Add the `CHANGELOG.md` entry.

## 7. Gates and review

- [x] Run the gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets`, doctests and the figures drift check,
  plus the `--all-features` variants.
- [x] Open the ADR issue and reference it from the three PRs with a closing
  keyword.
- [ ] Trigger the independent acceptance review against
  `docs/FDRs/recipes/matrix-clustered-heatmap-plot.md` before opening the code
  PR.
