# Maintaining this book

This part is a didactic, order-of-PRs textbook explaining the *why* behind
every `scienceview` change, for a reader with no Rust or numerical-computing
background.

- A new feature, figure, artist, kernel, or method adds a **new** chapter
  (`chNN-<slug>.md`), registered in `SUMMARY.md` under this part.
- A bug fix or refactor that changes existing behavior **edits** the existing
  chapter covering that behavior; fixes amend, they do not duplicate.

Every chapter is grounded in the real code and specs: the companion
`*_tests.rs` modules are the executable source of truth walked through as
examples.

## Chapters

| Chapter | File | Subject |
|---|---|---|
| 1 | `ch01-product-foundation.md` | The product foundation: the two audiences, the trait/enum split, the four-stage interaction pipeline, and the open decisions |
| 2 | `ch02-workspace-bootstrap.md` | The Cargo workspace: the crate map, the feature map, the pinned toolchain and the four CI gates |
| 3 | `ch03-theme-tokens.md` | Design tokens from zero: the stylesheet source, build-time and runtime compilation, palettes and colour maps, motion tokens and reduced motion |
| 4 | `ch04-data-source-layer.md` | The data boundary: `SVDataSource`, the three representations, sparse indices, the bounded window, dispatch once and the parity harness |
| 5 | `ch05-figure-builder-core.md` | The figure substrate: the builder typestate, the required explicit backend, per-operation dispatch, one error type and one operation span |
| 6 | `ch06-scale-mapping.md` | The scale kernel: the four kinds, explicit and derived domains across the three representations, the four transforms and their inverses, stable nominal colours, theme-bound continuous colour and tick derivation from the granted space |
| 7 | `ch07-layout-solver.md` | The layout solver: the length model, constraints, the measure and solve passes, the four responsiveness modes, the viewport and grid-independence, and the property tests |
| 8 | `ch08-mark-rendering.md` | Marks: the closed nine-variant vocabulary, the representation parity of the geometry kernel, backend-agnostic primitives, theme defaults and overrides, build-time validation and the interpolable-property table |
| 9 | `ch09-chart-decorations.md` | Decorations: the measure/solve protocol, space-adaptive ticks and label collision avoidance, the legend entry lifecycle and bounded layout, the colour bar as a scale view, titles, text boxes and themed primitives |
| 10 | `ch10-statistical-transforms.md` | Statistical transforms: the closed seven-variant `SVStat`, non-densifying sparse reads and streaming windows, binning, counting, aggregation with Welford and weighted means, percentiles without expansion, kernel density and bandwidth rules, the empirical CDF, the five fit models with confidence bands and the estimator trait, determinism and the scoped compute pool |

Chapter 3 covers theming, chapter 4 covers the data boundary, chapter 5
covers the figure substrate, chapter 6 covers the scale kernel, chapter 7
covers the layout solver, chapter 8 covers the mark kernel, chapter 9
covers the decoration surface and chapter 10 covers the statistical
transform kernel, each written by its own change. Later chapters are
planned, one per capability of the `foundation-core-architecture` change —
`backend-selection`, `data-view-boundary`, `model-events`, `event-payloads`,
`backend-capabilities`, `theming`, `layout-engine`, `ui-kit`, `scripting` and
`dashboard` — and are written as the corresponding implementation changes
land, so that each one can walk through real code and real companion tests.
The plan lives in
`openspec/changes/foundation-core-architecture/documentation-plan.md`.

## Diagrams

Diagrams are always Mermaid or SVG, colourful and harmonized with the mdBook
theme; ASCII-art diagrams are forbidden. A diagram is authored as Mermaid
source and rendered to SVG, and the SVG is checked in under
`docs/src/development-explained/assets/` so the book renders without a
JavaScript dependency.
