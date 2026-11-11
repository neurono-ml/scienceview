# Design

## Context

The repository is empty of code but already carries a large specification. The
workspace shape must therefore anticipate ten backends and a dependency-light
core, without pulling any backend dependency into the default build.

The reference for the layout is the sibling project `sciencekit`, which is the
same organisation, the same toolchain and the same change workflow.

## Decisions

### D1 — One crate per backend, one core crate

`crates/scienceview_core` holds the figure model, data boundary, kernels and layout. Each backend is its own crate (`scienceview_svg`, `scienceview_html`, `scienceview_tui`, `scienceview_native`, `scienceview_wasm`, `scienceview_react`, `scienceview_vue`, `scienceview_angular`, `scienceview_tauri`, `scienceview_dioxus`). The root package is a facade: it re-exports whatever features are enabled and nothing else. This keeps the default build free of winit, ratatui and wasm-bindgen.

### D2 — Features gate crates, never behaviour

A feature in the root `Cargo.toml` enables an optional dependency on a backend crate. Enabling a feature only makes a backend *available*; it never selects one, consistent with `backend-selection`.

### D3 — The gates are the contract

CI runs formatting, clippy with warnings as errors, the full test suite and the figures drift job on every pull request. A change that cannot pass these is not mergeable, so the gates are part of the root change rather than a later convenience.

### D4 — Figures are owned by the code branch

The examples write their SVGs to `docs/figures/`, which the `figures` job checks. The book keeps a mirrored copy on `docs/documentations`; the mirror is verified by the book branch's own pipeline, not by this job.

## Risks / Trade-offs

- A workspace with ten backend crates adds a large surface of empty crates; the risk is that the boundaries are drawn before the first backend is written. Mitigation: the crate boundaries follow the feature names already fixed by the PRD, and a crate may be empty until its change lands.
- Clippy with `--all-targets` on an empty workspace can fail on generated code; the workspace enables the lint set explicitly rather than inheriting surprises.

