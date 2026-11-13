# Tasks

## 1. Weighted kernel: binning

- [x] Write the failing tests first: weighted counts, all-ones equality,
  weighted probability and density normalisation, sparse implicit zeros,
  negative/non-finite/mismatched weight rejection.
- [x] Resolve the weight column once in `SVStat::Bin` through a shared
  `stat/weights.rs` fold; never densify a sparse value or weight column.
- [x] Reject a negative weight with `SVError::NegativeWeight { operation,
  position }` and a non-finite weight with `SVError::NonFiniteValue`.

## 2. Weighted kernel: kernel density

- [x] Write the failing tests first: all-ones equality, weighted estimate
  integrates to one, sparse implicit zeros weighted, negative/non-finite/zero
  total/wrong length rejection.
- [x] Normalise the weighted estimate by the total weight times the bandwidth,
  keeping Silverman's bandwidth derivation unweighted (recorded in the FDR).
- [x] Adopt the typed weight error in aggregates so every weight-accepting
  kernel shares one error surface.

## 3. Parity and concurrency

- [x] Extend the representation-parity tests: weighted binning over dense,
  sparse and streaming value and weight columns; weighted density over dense,
  sparse and streaming.
- [x] Add a concurrency test asserting weighted binning and density are
  thread-independent and equal to the calling thread's result.

## 4. Histogram recipe

- [x] Write the failing tests first: a weight column scales the heights, a
  wrong-length weight column is rejected, a negative weight reports its
  position, and weighted dense/sparse/streaming inputs agree.
- [x] Remove the build-time weight rejection and carry the column through
  `SVHistogramConfiguration` into `SVStat::Bin`, aligned with the value
  column through series splitting and missing-value omission.
- [x] Update `docs/FDRs/recipes/distribution-histogram-plot.md` and the
  `histogram-plot` deferral note.

## 5. Density recipe

- [x] Write the failing tests first: all-ones equality, a weight shifts the
  curve, the weighted density integrates to one, sparse equivalence, grouped
  alignment, negative/wrong-length rejection.
- [x] Add `SVDensityPlot::weight(...)`, resolve the column once per series and
  hand it to `SVStat::KernelDensity`; zero the weight of a dropped sparse
  stored entry instead of densifying.
- [x] Update `docs/FDRs/recipes/distribution-density-plot.md` and the
  `density-plot` deferral note.

## 6. FDR, changelog and book

- [x] Update `docs/FDRs/core/core-stat.md`: status, acceptance evidence and the
  weighted variants of `Bin` and `KernelDensity`.
- [x] Add the `CHANGELOG.md` entry under `## [Unreleased] / ### Added`.
- [x] Add `docs/src/development-explained/ch53-weighted-statistics.md`, register
  it in `SUMMARY.md`, update `maintaining.md`, and include an SVG diagram.
- [x] Update the histogram and density chapters where they say weights are
  deferred.

## 7. Gates

- [x] `cargo fmt --all --check`.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` and
  `--all-features`.
- [x] `cargo test --workspace --all-targets` and `--all-features`, plus doctests.
- [x] Regenerate every example and confirm no drift under `docs/figures`.
- [x] `openspec validate statistical-weights --strict` and `mdbook build docs`.

## 8. Delivery

- [x] Open the ADR issue and reference it from all three PRs with a closing
  keyword.
- [x] Push the code, openspec and docs branches and open the three PRs.
- [x] Watch CI and fix any red check.
