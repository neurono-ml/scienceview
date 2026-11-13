# Tasks

## 1. Module, builder and composition

- [x] Write the failing tests first: the chart builds with its documented
  defaults, and each option changes the output.
- [x] Create `src/recipes/relational/band_plot/{mod.rs, builder.rs,
  core_implementation.rs}` following patterns.md P3; `mod.rs` is a pure
  dispatcher.
- [x] Implement the builder state machine, the required `backend(...)` and the
  default `execution_mode(...)`, with `SVBandPlot` carrying its configured kind.
- [x] Wire the composition `data -> [SVStat::Aggregate + error] -> (lower, upper) -> [no move] -> [SVScale] -> SVMark::Ribbon` through the core kernels.

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

- [x] Add `examples/band_plot.rs` demonstrating the documented configuration.
- [x] Generate `docs/figures/band_plot.svg` with `cargo run --example band_plot` and
  verify the `figures` gate reports no drift.

## 5. Book chapter (book branch)

- [x] Create the documentation branch from `docs/documentations` and add the
  chapter under `docs/src/`.
- [x] Mirror the figure into `docs/src/assets/band_plot.svg` and embed it in the
  chapter.
- [x] Register the chapter in `docs/src/SUMMARY.md` and verify `mdbook build`.

## 6. FDR and changelog

- [x] Change `docs/FDRs/relational-band-plot.md` status to `shipped`, linking the
  example and the chapter.
- [x] Add the `CHANGELOG.md` entry.

## 7. Acceptance

- [x] Run the four gates locally: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets`, `cargo test --workspace`, and the
  figures check.
- [x] Open the ADR issue and reference it from both PRs with a closing keyword.
- [ ] Trigger the independent acceptance review against
  `docs/FDRs/relational-band-plot.md` before opening the code PR. *Pending the
  parent agent: this implementer validated the FDR acceptance locally, but an
  independent reviewer must still sign it off.*

## 8. Implementation notes (recorded after the code landed)

- `derive` is a mode selected by `derive()` together with the `values` column,
  not a caller-supplied `SVStat`: only `SVStat::Aggregate` yields the
  `(value, interval)` pair the ribbon consumes, so the recipe builds that stat
  from `error` and `confidence`. The narrowing is recorded in `design.md`.
- An explicit `SVMissingValuePolicy` was added to the surface: `Omit` (default)
  splits the band at a non-finite sample; `Error` rejects the render.
- The render operation returns a backend-agnostic `SVBandRenderOutput`; the
  SVG serializer is the `render-svg` feature and the example serializes the
  same geometry through `SVSvgDocument`.
