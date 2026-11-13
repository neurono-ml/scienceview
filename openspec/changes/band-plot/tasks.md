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
- [x] Trigger the independent acceptance review against
  `docs/FDRs/relational-band-plot.md` before opening the code PR. *Pending the
  parent agent: this implementer validated the FDR acceptance locally, but an
  independent reviewer must still sign it off.*

## 8. Review fixes

- [x] Validate the x scale kind and the column existence/roles at `build`
  with typed errors, and add the failing tests first.
- [x] Replace the densifying column reads with representation-aware positions
  (dense positions, sparse stored indices, streaming window) and cover a huge
  logical length with few stored entries in both modes.
- [x] Implement the composed native serializer (`to_native_frame`) and make
  every serializer reject a mismatched configured kind with a typed error.
- [x] Untick the transition acceptance item and defer the temporal behaviour to
  the transition integration instead of claiming an untested API.
- [x] Replace the assertion-free percentile test with exact interval
  assertions; cover titles, viewport, `execution_mode`, `temporal_unit`,
  derived missing values, column errors and implicit-zero sparse parity.
- [x] Count only drawn points in the `points` span field and remove the dead
  `as_str` API.
- [x] Expose the recipe as `sv_recipes::sv_relational::sv_band_plot` through
  `#[path]` and qualify the missing-value policy per recipe.
- [x] Call the public `to_svg()` from the example, regenerate the figure and
  pin the documentation branch to the new code tip.
- [x] Ground the FDR, the chapter and the changelog in the fixed code and
  tests.

## 9. Implementation notes (recorded after the code landed)

- `derive` is a mode selected by `derive()` together with the `values` column,
  not a caller-supplied `SVStat`: only `SVStat::Aggregate` yields the
  `(value, interval)` pair the ribbon consumes, so the recipe builds that stat
  from `error` and `confidence`. The narrowing is recorded in `design.md`.
- An explicit `SVBandMissingValuePolicy` was added to the surface: `Omit`
  (default) splits the band at a non-finite sample; `Error` rejects the render.
  The derived mode applies the same policy before the aggregate kernel.
- The render operation returns a backend-agnostic `SVBandRenderOutput`. The
  `render-svg` feature provides `to_svg()` and the `render-native` feature
  provides `to_native_frame()`; the example calls the public `to_svg()` rather
  than reimplementing the serializer.
- Sparse columns contribute only stored indices through representation-aware
  positions; the `points` span field counts drawn samples.
- The public path is `scienceview::sv_recipes::sv_relational::sv_band_plot`,
  mapped onto the physical folder with `#[path]`.
