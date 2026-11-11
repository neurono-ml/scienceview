# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The section between two releases describes the changes of the version being
released.

## [Unreleased]

### Added

- The layout solver in `scienceview_core` (`sv_layout`): a total length model
  (`SVLength` with absolute, fraction, intrinsic, aspect-ratio and fill
  variants), minimum and maximum constraints, a layout tree solved in two
  passes (measure, then solve) against a backend-supplied `SVViewport`, the
  four explicit responsiveness modes (fixed, fit to parent, aspect-ratio
  constrained, content derived), cached invalidation and revisioning on
  viewport change, spacing seeded by theme tokens with built-in defaults, and
  deterministic clamping — the minimum wins — when constraints contradict.
- The data source layer: `SVDataSource` is the open ingestion contract and
  `SVColumnView` is the closed computation boundary with exactly three
  representations — dense, sparse and streaming. Adapters cover borrowed
  slices, `ndarray` views (`data-ndarray`), `sprs` sparse vectors
  (`data-sprs`), standard-library channels (`streaming`) and Tokio channels
  (`streaming-tokio`); sparse data is never densified. `SVRollingWindow` bounds
  every streaming source and never blocks the render thread, and
  `SVRepresentationParityHarness` proves dense, sparse and streaming outputs
  agree within the documented tolerance (dense against sparse exactly).
- Figure substrate in `scienceview_core`: `SVFigure`, the typestate
  `SVFigureBuilder` that cannot build without a data source and a backend,
  `SVBackendKind` (one variant per backend, deliberately no automatic variant),
  `SVExecutionMode` (defaulting to `Automatic`) and the `SVBackendRegistry`
  through which backend crates — including third-party crates — plug in.
- `SVRenderBackend` as the public rendering contract: `build` transfers the
  configured kind into the figure without dispatching, and every operation
  (`render`, `update`, `save`, `show`) resolves the concrete backend at call
  time. Unsupported capability combinations fail at build time.
- `SVError` as the single public error enumeration, including
  `UnsupportedRepresentation` and backend-unavailable errors.
- One operation span per public operation, carrying `backend.kind`,
  `representation`, `points` and `duration`, with the outcome recorded on
  completion.
- `scienceview_theme`, the design-token crate: one authored stylesheet
  (`stylesheet/scienceview.scss`) compiled at build time into typed colours,
  typography, spacing, radii, shadows and motion tokens, with categorical,
  continuous and data-derived palettes, colour maps, reduced-motion collapse
  and opt-in runtime recompilation that restyles a figure without rebuilding
  it. Exposed through the `theme` feature.
- The repository now builds as a Cargo workspace: `scienceview_core` plus one
  crate per rendering backend (`scienceview_svg`, `scienceview_html`,
  `scienceview_tui`, `scienceview_native`, `scienceview_wasm`,
  `scienceview_react`, `scienceview_vue`, `scienceview_angular`,
  `scienceview_tauri`, `scienceview_dioxus`). The root `scienceview` package is
  a facade that re-exports the core and the enabled backends, `default = []`
  and every backend feature maps 1:1 to its optional path dependency.
- `rust-toolchain.toml` pinning Rust 1.85 with `rustfmt` and `clippy`, and a
  workspace lint set that denies warnings.
- Workspace smoke tests covering core reachability, the default feature set,
  the toolchain pin, the lint configuration, the README badges and this
  changelog entry.
- Feature Description Records for the chart surface (`docs/FDRs/`) — one
  record per core kernel and per scientific-core chart recipe, plus the
  catalogue of deferred chart types and the engineering contract that
  governs their implementation.
- A scaffolded change grid of fifty-two OpenSpec changes implementing the
  chart surface, the kernels it composes and the backends and integrations it
  needs, dispatched as a dependency-ordered DAG of waves.

- Product requirements document (`docs/PRD.md`) establishing the product source
  of truth: the vision, the two audiences the library serves, the
  architectural principles, the capability catalogue, the roadmap and the open
  decisions.
- Ten normative capability specifications covering backend selection, the data
  view boundary, model events, event payloads, backend capabilities, theming,
  the layout engine, the interactive control kit, scripting and dashboards.
- `AGENTS.md` and `openspec/config.yaml` now point at `docs/FDRs/patterns.md`
  as the normative contract for changes that implement the chart surface.

### Changed

- CI now runs the workspace quality gates — formatting, clippy with warnings as
  errors and the workspace test suite — plus a figures job that reruns every
  example and fails on any drift under `docs/figures/`.
- The README now points at the product requirements document and describes the
  facade-over-workspace layout.

## [0.1.0] - Unreleased

### Added

- Initial repository: builder-style figure description, the trait-driven
  feature-gated rendering backends and the documentation figures generated by
  the tool itself.

[Unreleased]: https://github.com/neurono-ml/scienceview/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/neurono-ml/scienceview/releases/tag/v0.1.0
