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
| 5 | `ch05-figure-builder-core.md` | The figure substrate: the builder typestate, the required explicit backend, per-operation dispatch, one error type and one operation span |

Chapter 3 covers theming, chapter 4 covers the data boundary and chapter 5
covers the figure substrate, each written by its own change. Later chapters are
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
